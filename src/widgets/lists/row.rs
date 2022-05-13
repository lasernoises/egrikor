use piet_common::Piet;

use super::*;

pub fn row<'a, P, C: FlexContent<P> + 'a>(params: &'a mut P, content: C) -> impl Widget + 'a {
    Row { params, content }
}

pub struct Row<'a, P, C> {
    params: &'a mut P,
    content: C,
}

pub struct RowState<S> {
    content_state: S,

    // TODO: find a way to make this not dependent on order; like the keys common in react-style
    // frameworks
    focus: Option<u16>,

    size: Size,
    no_expand_size: f64,
    expand_count: u32,
    extra_layers: u8,
}

impl<'a, P, C: FlexContent<P>> Widget for Row<'a, P, C> {
    type State = RowState<C::State>;

    fn build(
        &mut self,
        constraint: LayoutConstraint,
        ctx: &mut LayoutCtx,
    ) -> Self::State {
        let mut state = Self::State {
            content_state: C::State::new(),
            focus: None,
            size: Size::ZERO,
            no_expand_size: 0.,
            expand_count: 0,
            extra_layers: 0,
        };

        self.update(&mut state, constraint, ctx);

        state
    }

    fn update(
        &mut self,
        state: &mut Self::State,
        constraint: LayoutConstraint,
        ctx: &mut LayoutCtx,
    ) {
        struct MeasureHandler<'a, 'b> {
            constraint: &'a LayoutConstraint,
            ctx: &'a mut LayoutCtx<'b>,
            size: Size,
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
                        widget.update(state, [None, self.constraint[1]], self.ctx);
                    } else {
                        *state = Some(widget.build(
                            [None, self.constraint[1]],
                            self.ctx,
                        ));
                    }

                    let state = state.as_mut().unwrap();

                    let min_size = widget.min_size(state);
                    self.size.width += min_size.width;
                    self.size.height = self.size.height.max(min_size.height);

                    self.extra_layers = self.extra_layers.max(widget.extra_layers(state));
                }
            }
        }

        let mut handler = MeasureHandler {
            constraint: &constraint,
            ctx,
            size: Size::ZERO,
            expand_count: 0,
            extra_layers: 0,
        };

        self.content.all(self.params, &mut state.content_state, &mut handler);

        state.expand_count = handler.expand_count;
        state.extra_layers = handler.extra_layers;
        let first_pass_size = handler.size;

        let min_width = first_pass_size.width;
        state.no_expand_size = min_width;

        struct MeasureExpandHandler<'a, 'b> {
            constraint: &'a [Option<f64>; 2],
            ctx: &'a mut LayoutCtx<'b>,
            size: Size,
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
                        widget.update(state, *self.constraint, self.ctx);
                    } else {
                        *state = Some(widget.build(*self.constraint, self.ctx));
                    }

                    let state = state.as_mut().unwrap();
                    let min_size = widget.min_size(state);

                    self.size.width += min_size.width;
                    self.size.height = self.size.height.max(min_size.height);
                    self.extra_layers = self.extra_layers.max(widget.extra_layers(state));
                }
            }
        }

        let mut handler = MeasureExpandHandler {
            constraint: &[
                constraint[0].map(|w| (w - min_width) / state.expand_count as f64),
                constraint[1],
            ],
            ctx,
            size: Size::ZERO,
            extra_layers: state.extra_layers,
        };

        self.content.all(self.params, &mut state.content_state, &mut handler);

        state.size = Size::new(
            constraint[0]
                .map(|w| w.max(handler.size.width))
                .unwrap_or(handler.size.width),
            constraint[1]
                .map(|w| w.max(handler.size.height))
                .unwrap_or(handler.size.height),
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
        layer: u8,
        focus: bool,
        ctx: &mut RenderCtx,
    ) {
        let min_width = state.no_expand_size;

        let extra_width = rect.width() - min_width;
        let expand_width = extra_width / state.expand_count as f64;

        struct RenderHandler<'a, 'b, 'c> {
            expand_width: f64,
            layer: u8,
            extra_layers: u8,
            pos: Point,
            size: Size,
            focus: Option<u16>,
            ctx: &'a mut RenderCtx<'b, 'c>,
            i: u16,
        }

        impl<'a, 'b, 'c> FlexContentHandler for RenderHandler<'a, 'b, 'c> {
            fn widget<W: Widget>(
                &mut self,
                widget: &mut W,
                state: &mut Option<W::State>,
                expand: bool,
            ) {
                let state = state.as_mut().unwrap();

                let widget_width = if expand {
                    self.expand_width
                } else {
                    widget.min_size(state).width
                };

                if self.layer <= widget.extra_layers(state) {
                    let empty_input_state: InputState = Default::default();

                    let input_state = if self.layer == self.extra_layers {
                        self.ctx.input_state
                    } else {
                        &empty_input_state
                    };

                    widget.render(
                        state,
                        Rect::from_origin_size(self.pos, Size { width: widget_width, height: self.size.height }),
                        self.layer,
                        self.focus == Some(self.i),
                        &mut RenderCtx {
                            piet: &mut *self.ctx.piet,
                            input_state: &input_state,
                            ..*self.ctx
                        },
                    );
                }

                self.pos.x += widget_width;
                self.i += 1;
            }
        }

        let mut handler = RenderHandler {
            expand_width,
            layer,
            extra_layers: state.extra_layers,
            pos: rect.origin(),
            size: rect.size(),
            focus: if focus { state.focus } else { None },
            ctx,
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
    ) -> InputReturn {
        let min_width = state.no_expand_size;

        let extra_width = rect.width() - min_width;
        let expand_width = extra_width / state.expand_count as f64;

        struct CursorInputHandler<'a> {
            pos: Point,
            size: Size,
            expand_width: f64,
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

                let widget_width = if expand {
                    self.expand_width
                } else {
                    widget.min_size(state).width
                };

                let extra_layers = widget.extra_layers(state);

                if extra_layers >= self.cursor_layer {
                    let ret = widget.handle_cursor_input(
                        state,
                        Rect::from_origin_size(self.pos, Size { width: widget_width, height: self.size.height }),
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
            focus,
            focus_state: &mut state.focus,
            i: 0,
            demand_focus: false,
        };

        self.content.all(self.params, &mut state.content_state, &mut handler);

        InputReturn {
            demand_focus: handler.demand_focus,
        }
    }

    fn handle_keyboard_input(
        &mut self,
        state: &mut Self::State,
        rect: Rect,
        input: &KeyboardInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
    ) {
        struct KeyboardInputHandler<'a> {
            pos: Point,
            size: Size,
            expand_width: f64,
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

                let widget_width = if expand {
                    self.expand_width
                } else {
                    widget.min_size(state).width
                };

                let focus = self.focus == Some(self.i);
                if focus {
                    widget.handle_keyboard_input(
                        state,
                        Rect::from_origin_size(self.pos, Size { width: widget_width, height: self.size.height }),
                        self.input,
                        self.input_state,
                        self.theme,
                        focus,
                    );
                }

                self.pos.x += widget_width;
                self.i += 1;
            }
        }

        let min_width = state.no_expand_size;

        let extra_width = rect.width() - min_width;
        let expand_width = extra_width / state.expand_count as f64;

        let mut handler = KeyboardInputHandler {
            pos: rect.origin(),
            size: rect.size(),
            expand_width,
            input,
            input_state,
            theme,
            focus: if focus { state.focus } else { None },
            i: 0,
        };

        self.content.all(self.params, &mut state.content_state, &mut handler);
    }
}
