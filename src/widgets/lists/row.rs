use piet_common::Piet;

use super::*;

#[macro_export]
macro_rules! ft_row {
    () => {
        row(EmptyListContent)
    };
    ($($e:expr),+ $(,)?) => {{
        let content = $crate::ft_list_content![$($e),*];
        row(content)
    }}
}

pub fn row<C: Debug, D: ListContent<C>>(content: D) -> impl Element<C> {
    Row { content }
}

pub struct Row<D> {
    content: D,
}

impl<C: Debug, D: ListContent<C>> Element<C> for Row<D> {
    type Widget = RowWidget<D::Widgets>;

    fn build(self) -> Self::Widget {
        RowWidget {
            content: self.content.build(),
            focus: None,
            size: Size::ZERO,
            no_expand_size: 0.,
            expand_count: 0,
            extra_layers: 0,
        }
    }

    fn update(self, widget: &mut Self::Widget) {
        self.content.update(&mut widget.content);
    }
}

pub struct RowWidget<W> {
    content: W,

    // TODO: find a way to make this not dependent on order; like the keys common in react-style
    // frameworks
    focus: Option<u16>,

    size: Size,
    no_expand_size: f64,
    expand_count: u32,
    extra_layers: u8,
}

impl<C: Debug, W: ListContentWidgets<C>> Widget<C> for RowWidget<W> {
    type Event = ();

    fn measure(
        &mut self,
        max_size: [Option<f64>; 2],
        renderer: &mut Piet,
        theme: &Theme,
        context: &mut C,
    ) -> Size {
        struct MeasureHandler<'a, 'b, C> {
            constraint: &'a [Option<f64>; 2],
            renderer: &'a mut Piet<'b>,
            size: Size,
            theme: &'a Theme,
            expand_count: u32,
            extra_layers: u8,
            context: &'a mut C,
        }

        impl<'a, 'b, C: Debug> WidgetHandler<C> for MeasureHandler<'a, 'b, C> {
            fn widget<W: Widget<C>>(&mut self, widget: &mut W, expand: bool) {
                if expand {
                    self.expand_count += 1;
                } else {
                    let widget_size = widget.measure(
                        [None, self.constraint[1]],
                        self.renderer,
                        self.theme,
                        self.context,
                    );

                    self.size.width += widget_size.width;
                    self.size.height = self.size.height.max(widget_size.height);
                    self.extra_layers = self.extra_layers.max(widget.extra_layers());
                }
            }
        }

        let mut handler = MeasureHandler {
            constraint: &max_size,
            renderer,
            size: Size::ZERO,
            theme,
            expand_count: 0,
            extra_layers: 0,
            context,
        };

        self.content.all(&mut handler);

        self.expand_count = handler.expand_count;
        self.extra_layers = handler.extra_layers;
        let first_pass_size = handler.size;

        let min_width = first_pass_size.width;
        self.no_expand_size = min_width;

        struct MeasureExpandHandler<'a, 'b, C> {
            constraint: &'a [Option<f64>; 2],
            renderer: &'a mut Piet<'b>,
            size: Size,
            theme: &'a Theme,
            extra_layers: u8,
            context: &'a mut C,
        }

        impl<'a, 'b, C: Debug> WidgetHandler<C> for MeasureExpandHandler<'a, 'b, C> {
            fn widget<W: Widget<C>>(&mut self, widget: &mut W, expand: bool) {
                if expand {
                    let widget_size =
                        widget.measure(*self.constraint, self.renderer, self.theme, self.context);

                    self.size.width += widget_size.width;
                    self.size.height = self.size.height.max(widget_size.height);
                    self.extra_layers = self.extra_layers.max(widget.extra_layers());
                }
            }
        }

        let mut handler = MeasureExpandHandler {
            constraint: &[
                max_size[0].map(|w| (w - min_width) / self.expand_count as f64),
                max_size[1],
            ],
            renderer,
            size: Size::ZERO,
            theme,
            extra_layers: self.extra_layers,
            context,
        };

        self.content.all(&mut handler);

        self.size = Size::new(
            max_size[0]
                .map(|w| w.max(handler.size.width))
                .unwrap_or(handler.size.width),
            max_size[1]
                .map(|w| w.max(handler.size.height))
                .unwrap_or(handler.size.height),
        );

        self.extra_layers = handler.extra_layers;

        self.size
    }

    fn min_size(&self) -> Size {
        self.size
    }

    fn extra_layers(&self) -> u8 {
        self.extra_layers
    }

    fn render(
        &mut self,
        rect: Rect,
        renderer: &mut Piet,
        theme: &Theme,
        input_state: &InputState,
        layer: u8,
        focus: bool,
        context: &mut C,
    ) {
        let min_width = self.no_expand_size;

        let extra_width = rect.width() - min_width;
        let expand_width = extra_width / self.expand_count as f64;

        struct RenderHandler<'a, 'b, C> {
            expand_width: f64,
            layer: u8,
            input_state: &'a InputState,
            extra_layers: u8,
            pos: Point,
            size: Size,
            renderer: &'a mut Piet<'b>,
            theme: &'a Theme,
            context: &'a mut C,
            focus: Option<u16>,
            i: u16,
        }

        impl<'a, 'b, C: Debug> WidgetHandler<C> for RenderHandler<'a, 'b, C> {
            fn widget<W: Widget<C>>(&mut self, widget: &mut W, expand: bool) {
                let widget_width = if expand {
                    self.expand_width
                } else {
                    widget.min_size().width
                };

                if self.layer <= widget.extra_layers() {
                    let empty_input_state: InputState = Default::default();

                    let input_state = if self.layer == self.extra_layers {
                        self.input_state
                    } else {
                        &empty_input_state
                    };

                    widget.render(
                        Rect::from_origin_size(self.pos, (widget_width, self.size.height)),
                        self.renderer,
                        self.theme,
                        input_state,
                        self.layer,
                        self.focus == Some(self.i),
                        self.context,
                    );
                }

                self.pos.x += widget_width;
                self.i += 1;
            }
        }

        let mut handler = RenderHandler {
            expand_width,
            layer,
            input_state,
            extra_layers: self.extra_layers,
            pos: rect.origin(),
            size: rect.size(),
            renderer,
            theme,
            context,
            focus: if focus { self.focus } else { None },
            i: 0,
        };

        self.content.all(&mut handler);
    }

    fn handle_cursor_input(
        &mut self,
        rect: Rect,
        cursor_pos: Point,
        cursor_layer: u8,
        input: CursorInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
        context: &mut C,
    ) -> (InputReturn, Option<Self::Event>) {
        let min_width = self.no_expand_size;

        let extra_width = rect.width() - min_width;
        let expand_width = extra_width / self.expand_count as f64;

        struct CursorInputHandler<'a, C> {
            pos: Point,
            size: Size,
            expand_width: f64,
            cursor_pos: Point,
            cursor_layer: u8,
            theme: &'a Theme,
            input: CursorInput,
            input_state: &'a InputState,
            context: &'a mut C,
            focus: bool,
            focus_state: &'a mut Option<u16>,
            i: u16,
            demand_focus: bool,
        }

        impl<'a, C: Debug> WidgetHandler<C> for CursorInputHandler<'a, C> {
            fn widget<W: Widget<C>>(&mut self, widget: &mut W, expand: bool) {
                let widget_width = if expand {
                    self.expand_width
                } else {
                    widget.min_size().width
                };

                let extra_layers = widget.extra_layers();

                if extra_layers >= self.cursor_layer {
                    let (ret, _) = widget.handle_cursor_input(
                        Rect::from_origin_size(self.pos, (widget_width, self.size.height)),
                        self.cursor_pos,
                        self.cursor_layer,
                        self.input,
                        self.input_state,
                        self.theme,
                        self.focus && *self.focus_state == Some(self.i),
                        self.context,
                    );

                    if ret.demand_focus {
                        *self.focus_state = Some(self.i);
                        self.demand_focus = true;
                    }
                }

                self.pos.x += widget_width;
                self.i += 1;
            }
        }

        let mut handler = CursorInputHandler {
            pos: rect.origin(),
            size: rect.size(),
            expand_width,
            cursor_pos,
            cursor_layer,
            theme,
            input,
            input_state,
            context,
            focus,
            focus_state: &mut self.focus,
            i: 0,
            demand_focus: false,
        };

        self.content.all(&mut handler);

        (
            InputReturn {
                demand_focus: handler.demand_focus,
            },
            None,
        )
    }

    fn handle_keyboard_input(
        &mut self,
        rect: Rect,
        input: &KeyboardInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
        context: &mut C,
    ) -> Option<Self::Event> {
        struct KeyboardInputHandler<'a, C> {
            pos: Point,
            size: Size,
            expand_width: f64,
            input: &'a KeyboardInput,
            input_state: &'a InputState,
            theme: &'a Theme,
            context: &'a mut C,
            focus: Option<u16>,
            i: u16,
        }

        impl<'a, C: Debug> WidgetHandler<C> for KeyboardInputHandler<'a, C> {
            fn widget<W: Widget<C>>(&mut self, widget: &mut W, expand: bool) {
                let widget_width = if expand {
                    self.expand_width
                } else {
                    widget.min_size().width
                };

                let focus = self.focus == Some(self.i);
                if focus {
                    widget.handle_keyboard_input(
                        Rect::from_origin_size(self.pos, (widget_width, self.size.height)),
                        self.input,
                        self.input_state,
                        self.theme,
                        focus,
                        self.context,
                    );
                }

                self.pos.x += widget_width;
                self.i += 1;
            }
        }

        let min_width = self.no_expand_size;

        let extra_width = rect.width() - min_width;
        let expand_width = extra_width / self.expand_count as f64;

        let mut state = KeyboardInputHandler {
            pos: rect.origin(),
            size: rect.size(),
            expand_width,
            input,
            input_state,
            theme,
            context,
            focus: if focus { self.focus } else { None },
            i: 0,
        };

        self.content.all(&mut state);

        None
    }
}
