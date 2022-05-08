use piet_common::Piet;

use super::*;

pub fn col<'a, P, C: FlexContent<P> + 'a>(params: &'a mut P, content: C) -> impl Widget + 'a {
    Col { params, content }
}

pub struct Col<'a, P, C> {
    params: &'a mut P,
    content: C,
}

pub struct ColState<S> {
    content_state: S,

    // TODO: find a way to make this not dependent on order; like the keys common in react-style
    // frameworks
    focus: Option<u16>,

    size: Size,
    no_expand_size: f64,
    expand_count: u32,
    extra_layers: u8,
}

impl<'a, P, C: FlexContent<P>> Widget for Col<'a, P, C> {
    type State = ColState<C::State>;
    type Event = ();

    fn build(
        &mut self,
        constraint: LayoutConstraint,
        renderer: &mut Piet,
        theme: &Theme,
    ) -> Self::State {
        let mut state = Self::State {
            content_state: C::State::new(),
            focus: None,
            size: Size::ZERO,
            no_expand_size: 0.,
            expand_count: 0,
            extra_layers: 0,
        };

        self.update(&mut state, constraint, renderer, theme);

        state
    }

    fn update(
        &mut self,
        state: &mut Self::State,
        constraint: LayoutConstraint,
        renderer: &mut Piet,
        theme: &Theme,
    ) {
        struct MeasureHandler<'a, 'b> {
            constraint: &'a LayoutConstraint,
            renderer: &'a mut Piet<'b>,
            size: Size,
            theme: &'a Theme,
            expand_count: u32,
            extra_layers: u8,
        }

        impl<'a, 'b> FlexContentHandler for MeasureHandler<'a, 'b> {
            fn widget<W: Widget>(
                &mut self,
                widget: &mut W,
                state: &mut Option<W::State>,
                expand: bool,
            ) {
                if expand {
                    self.expand_count += 1;
                } else {
                    if let Some(state) = state {
                        widget.update(state, [None, self.constraint[1]], self.renderer, self.theme);
                    } else {
                        *state = Some(widget.build(
                            [None, self.constraint[1]],
                            self.renderer,
                            self.theme,
                        ));
                    }

                    let state = state.as_mut().unwrap();

                    let min_size = widget.min_size(state);
                    self.size.height += min_size.height;
                    self.size.width = self.size.width.max(min_size.width);

                    self.extra_layers = self.extra_layers.max(widget.extra_layers(state));
                }
            }
        }

        let mut handler = MeasureHandler {
            constraint: &constraint,
            renderer,
            size: Size::ZERO,
            theme,
            expand_count: 0,
            extra_layers: 0,
        };

        self.content.all(self.params, &mut state.content_state, &mut handler);

        state.expand_count = handler.expand_count;
        state.extra_layers = handler.extra_layers;
        let first_pass_size = handler.size;

        let min_height = first_pass_size.height;
        state.no_expand_size = min_height;

        struct MeasureExpandHandler<'a, 'b> {
            constraint: &'a [Option<f64>; 2],
            renderer: &'a mut Piet<'b>,
            size: Size,
            theme: &'a Theme,
            extra_layers: u8,
        }

        impl<'a, 'b> FlexContentHandler for MeasureExpandHandler<'a, 'b> {
            fn widget<W: Widget>(
                &mut self,
                widget: &mut W,
                state: &mut Option<W::State>,
                expand: bool,
            ) {
                if expand {
                    if let Some(state) = state {
                        widget.update(state, *self.constraint, self.renderer, self.theme);
                    } else {
                        *state = Some(widget.build(*self.constraint, self.renderer, self.theme));
                    }

                    let state = state.as_mut().unwrap();
                    let min_size = widget.min_size(state);

                    self.size.height += min_size.height;
                    self.size.width = self.size.width.max(min_size.width);
                    self.extra_layers = self.extra_layers.max(widget.extra_layers(state));
                }
            }
        }

        let mut handler = MeasureExpandHandler {
            constraint: &[
                constraint[0].map(|w| (w - min_height) / state.expand_count as f64),
                constraint[1],
            ],
            renderer,
            size: Size::ZERO,
            theme,
            extra_layers: state.extra_layers,
        };

        self.content.all(self.params, &mut state.content_state, &mut handler);

        state.size = Size::new(
            constraint[0]
                .map(|w| w.max(handler.size.height))
                .unwrap_or(handler.size.height),
            constraint[1]
                .map(|w| w.max(handler.size.width))
                .unwrap_or(handler.size.width),
        );

        state.extra_layers = handler.extra_layers;
    }

    fn min_size(&self, state: &Self::State) -> Size {
        state.size
    }

    fn extra_layers(&self, state: &Self::State) -> u8 {
        state.extra_layers
    }

    fn render(
        &mut self,
        state: &mut Self::State,
        rect: Rect,
        renderer: &mut Piet,
        theme: &Theme,
        input_state: &InputState,
        layer: u8,
        focus: bool,
    ) {
        let min_height = state.no_expand_size;

        let extra_height = rect.height() - min_height;
        let expand_height = extra_height / state.expand_count as f64;

        struct RenderHandler<'a, 'b> {
            expand_height: f64,
            layer: u8,
            input_state: &'a InputState,
            extra_layers: u8,
            pos: Point,
            size: Size,
            renderer: &'a mut Piet<'b>,
            theme: &'a Theme,
            focus: Option<u16>,
            i: u16,
        }

        impl<'a, 'b> FlexContentHandler for RenderHandler<'a, 'b> {
            fn widget<W: Widget>(
                &mut self,
                widget: &mut W,
                state: &mut Option<W::State>,
                expand: bool,
            ) {
                let state = state.as_mut().unwrap();

                let widget_height = if expand {
                    self.expand_height
                } else {
                    widget.min_size(state).height
                };

                if self.layer <= widget.extra_layers(state) {
                    let empty_input_state: InputState = Default::default();

                    let input_state = if self.layer == self.extra_layers {
                        self.input_state
                    } else {
                        &empty_input_state
                    };

                    widget.render(
                        state,
                        Rect::from_origin_size(self.pos, Size { height: widget_height, width: self.size.width }),
                        self.renderer,
                        self.theme,
                        input_state,
                        self.layer,
                        self.focus == Some(self.i),
                    );
                }

                self.pos.y += widget_height;
                self.i += 1;
            }
        }

        let mut handler = RenderHandler {
            expand_height,
            layer,
            input_state,
            extra_layers: state.extra_layers,
            pos: rect.origin(),
            size: rect.size(),
            renderer,
            theme,
            focus: if focus { state.focus } else { None },
            i: 0,
        };

        self.content.all(self.params, &mut state.content_state, &mut handler);
    }

    fn handle_cursor_input(
        &mut self,
        state: &mut Self::State,
        rect: Rect,
        cursor_pos: Point,
        cursor_layer: u8,
        input: CursorInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
    ) -> (InputReturn, Option<Self::Event>) {
        let min_height = state.no_expand_size;

        let extra_height = rect.height() - min_height;
        let expand_height = extra_height / state.expand_count as f64;

        struct CursorInputHandler<'a> {
            pos: Point,
            size: Size,
            expand_height: f64,
            cursor_pos: Point,
            cursor_layer: u8,
            theme: &'a Theme,
            input: CursorInput,
            input_state: &'a InputState,
            focus: bool,
            focus_state: &'a mut Option<u16>,
            i: u16,
            demand_focus: bool,
        }

        impl<'a> FlexContentHandler for CursorInputHandler<'a> {
            fn widget<W: Widget>(
                &mut self,
                widget: &mut W,
                state: &mut Option<W::State>,
                expand: bool,
            ) {
                let state = state.as_mut().unwrap();

                let widget_height = if expand {
                    self.expand_height
                } else {
                    widget.min_size(state).height
                };

                let extra_layers = widget.extra_layers(state);

                if extra_layers >= self.cursor_layer {
                    let (ret, _) = widget.handle_cursor_input(
                        state,
                        Rect::from_origin_size(self.pos, Size { height: widget_height, width: self.size.width }),
                        self.cursor_pos,
                        self.cursor_layer,
                        self.input,
                        self.input_state,
                        self.theme,
                        self.focus && *self.focus_state == Some(self.i),
                    );

                    if ret.demand_focus {
                        *self.focus_state = Some(self.i);
                        self.demand_focus = true;
                    }
                }

                self.pos.y += widget_height;
                self.i += 1;
            }
        }

        let mut handler = CursorInputHandler {
            pos: rect.origin(),
            size: rect.size(),
            expand_height,
            cursor_pos,
            cursor_layer,
            theme,
            input,
            input_state,
            focus,
            focus_state: &mut state.focus,
            i: 0,
            demand_focus: false,
        };

        self.content.all(self.params, &mut state.content_state, &mut handler);

        (
            InputReturn {
                demand_focus: handler.demand_focus,
            },
            None,
        )
    }

    fn handle_keyboard_input(
        &mut self,
        state: &mut Self::State,
        rect: Rect,
        input: &KeyboardInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
    ) -> Option<Self::Event> {
        struct KeyboardInputHandler<'a> {
            pos: Point,
            size: Size,
            expand_height: f64,
            input: &'a KeyboardInput,
            input_state: &'a InputState,
            theme: &'a Theme,
            focus: Option<u16>,
            i: u16,
        }

        impl<'a> FlexContentHandler for KeyboardInputHandler<'a> {
            fn widget<W: Widget>(
                &mut self,
                widget: &mut W,
                state: &mut Option<W::State>,
                expand: bool,
            ) {
                let state = state.as_mut().unwrap();

                let widget_height = if expand {
                    self.expand_height
                } else {
                    widget.min_size(state).height
                };

                let focus = self.focus == Some(self.i);
                if focus {
                    widget.handle_keyboard_input(
                        state,
                        Rect::from_origin_size(self.pos, Size { height: widget_height, width: self.size.width }),
                        self.input,
                        self.input_state,
                        self.theme,
                        focus,
                    );
                }

                self.pos.y += widget_height;
                self.i += 1;
            }
        }

        let min_height = state.no_expand_size;

        let extra_height = rect.height() - min_height;
        let expand_height = extra_height / state.expand_count as f64;

        let mut handler = KeyboardInputHandler {
            pos: rect.origin(),
            size: rect.size(),
            expand_height,
            input,
            input_state,
            theme,
            focus: if focus { state.focus } else { None },
            i: 0,
        };

        self.content.all(self.params, &mut state.content_state, &mut handler);

        None
    }
}
