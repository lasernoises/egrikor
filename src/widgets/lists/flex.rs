use piet_common::Piet;

use super::*;

macro_rules! flex {
    (
        $struct:ident,
        $state_struct:ident,
        $function:ident,
        $primary_axis:ident,
        $secondary_axis:ident,
        $primary_size:ident,
        $secondary_size:ident
    ) => {
        pub fn $function<E, C: FlexContent<E>>(content: C) -> impl Widget<E, State = $state_struct<C::State>> {
            $struct { content }
        }

        pub struct $struct<C> {
            content: C,
        }

        pub struct $state_struct<S> {
            content_state: S,

            // TODO: find a way to make this not dependent on order; like the keys common in react-style
            // frameworks
            focus: Option<u16>,

            size: Size,
            no_expand_size: f64,
            expand_count: u32,
            extra_layers: u8,
        }

        impl<S: FlexContentState> WidgetState for $state_struct<S> {
            fn new() -> Self {
                Self {
                    content_state: S::new(),
                    focus: None,
                    size: Size::ZERO,
                    no_expand_size: 0.,
                    expand_count: 0,
                    extra_layers: 0,
                }
            }

            fn min_size(&self) -> Size {
                self.size
            }

            fn extra_layers(&self) -> u8 {
                self.extra_layers
            }
        }

        impl<E, C: FlexContent<E>> Widget<E> for $struct<C> {
            type State = $state_struct<C::State>;

            fn layout(
                &mut self,
                state: &mut Self::State,
                env: &mut E,
                constraint: LayoutConstraint,
                ctx: &mut LayoutCtx,
            ) {
                struct MeasureHandler<'a, 'b, E> {
                    env: &'a mut E,
                    constraint: &'a LayoutConstraint,
                    ctx: &'a mut LayoutCtx<'b>,
                    size: Size,
                    expand_count: u32,
                    extra_layers: u8,
                }

                impl<'a, 'b, E> FlexContentHandler<E> for MeasureHandler<'a, 'b, E> {
                    fn widget<W: Widget<E>>(
                        &mut self,
                        widget: &mut W,
                        state: &mut W::State,
                        expand: bool,
                    ) {
                        if expand {
                            self.expand_count += 1;
                        } else {
                            widget.layout(state, self.env, LayoutConstraint {
                                $primary_axis: None,
                                $secondary_axis: self.constraint.$secondary_axis,
                            }, self.ctx);

                            let min_size = state.min_size();
                            self.size.$primary_size += min_size.$primary_size;
                            self.size.$secondary_size = self.size.$secondary_size.max(min_size.$secondary_size);

                            self.extra_layers = self.extra_layers.max(state.extra_layers());
                        }
                    }
                }

                let mut handler = MeasureHandler {
                    env,
                    constraint: &constraint,
                    ctx,
                    size: Size::ZERO,
                    expand_count: 0,
                    extra_layers: 0,
                };

                self.content.all(&mut state.content_state, &mut handler);

                state.expand_count = handler.expand_count;
                state.extra_layers = handler.extra_layers;
                let first_pass_size = handler.size;

                let min_length = first_pass_size.$primary_size;
                state.no_expand_size = min_length;

                struct MeasureExpandHandler<'a, 'b, E> {
                    env: &'a mut E,
                    constraint: &'a LayoutConstraint,
                    ctx: &'a mut LayoutCtx<'b>,
                    size: Size,
                    extra_layers: u8,
                }

                impl<'a, 'b, E> FlexContentHandler<E> for MeasureExpandHandler<'a, 'b, E> {
                    fn widget<W: Widget<E>>(
                        &mut self,
                        widget: &mut W,
                        state: &mut W::State,
                        expand: bool,
                    ) {
                        if expand {
                            widget.layout(state, self.env, *self.constraint, self.ctx);

                            let min_size = state.min_size();

                            self.size.$primary_size += min_size.$primary_size;
                            self.size.$secondary_size = self.size.$secondary_size.max(min_size.$secondary_size);
                            self.extra_layers = self.extra_layers.max(state.extra_layers());
                        }
                    }
                }

                let mut handler = MeasureExpandHandler {
                    env,
                    constraint: &LayoutConstraint {
                        $primary_axis: constraint.$primary_axis.map(|w| (w - min_length) / state.expand_count as f64),
                        $secondary_axis: constraint.$secondary_axis,
                    },
                    ctx,
                    size: Size::ZERO,
                    extra_layers: state.extra_layers,
                };

                self.content.all(&mut state.content_state, &mut handler);

                state.size = Size {
                    $primary_size: constraint.$primary_axis
                        .map(|w| w.max(handler.size.$primary_size))
                        .unwrap_or(handler.size.$primary_size),
                    $secondary_size: constraint.$primary_axis
                        .map(|w| w.max(handler.size.$secondary_size))
                        .unwrap_or(handler.size.$secondary_size),
                };

                state.extra_layers = handler.extra_layers;
            }

            fn render(
                &mut self,
                state: &mut Self::State,
                env: &mut E,
                rect: Rect,
                layer: u8,
                focus: bool,
                ctx: &mut RenderCtx,
            ) {
                let min_length = state.no_expand_size;

                let extra_length = rect.$primary_size() - min_length;
                let expand_length = extra_length / state.expand_count as f64;

                struct RenderHandler<'a, 'b, 'c, E> {
                    env: &'a mut E,
                    expand_length: f64,
                    layer: u8,
                    extra_layers: u8,
                    pos: Point,
                    size: Size,
                    focus: Option<u16>,
                    ctx: &'a mut RenderCtx<'b, 'c>,
                    i: u16,
                }

                impl<'a, 'b, 'c, E> FlexContentHandler<E> for RenderHandler<'a, 'b, 'c, E> {
                    fn widget<W: Widget<E>>(
                        &mut self,
                        widget: &mut W,
                        state: &mut W::State,
                        expand: bool,
                    ) {
                        let widget_length = if expand {
                            self.expand_length
                        } else {
                            state.min_size().$primary_size
                        };

                        if self.layer <= state.extra_layers() {
                            let empty_input_state: InputState = Default::default();

                            let input_state = if self.layer == self.extra_layers {
                                self.ctx.input_state
                            } else {
                                &empty_input_state
                            };

                            widget.render(
                                state,
                                self.env,
                                Rect::from_origin_size(self.pos, Size {
                                    $primary_size: widget_length,
                                    $secondary_size: self.size.$secondary_size,
                                }),
                                self.layer,
                                self.focus == Some(self.i),
                                &mut RenderCtx {
                                    piet: &mut *self.ctx.piet,
                                    input_state: &input_state,
                                    ..*self.ctx
                                },
                            );
                        }

                        self.pos.$primary_axis += widget_length;
                        self.i += 1;
                    }
                }

                let mut handler = RenderHandler {
                    env,
                    expand_length,
                    layer,
                    extra_layers: state.extra_layers,
                    pos: rect.origin(),
                    size: rect.size(),
                    focus: if focus { state.focus } else { None },
                    ctx,
                    i: 0,
                };

                self.content.all(&mut state.content_state, &mut handler);
            }

            fn handle_cursor_input(
                &mut self,
                state: &mut Self::State,
                env: &mut E,
                rect: Rect,
                cursor_pos: Point,
                cursor_layer: u8,
                input: CursorInput,
                input_state: &InputState,
                theme: &Theme,
                focus: bool,
            ) -> InputReturn {
                let min_length = state.no_expand_size;

                let extra_width = rect.$primary_size() - min_length;
                let expand_length = extra_width / state.expand_count as f64;

                struct CursorInputHandler<'a, E> {
                    env: &'a mut E,
                    pos: Point,
                    size: Size,
                    expand_length: f64,
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

                impl<'a, E> FlexContentHandler<E> for CursorInputHandler<'a, E> {
                    fn widget<W: Widget<E>>(
                        &mut self,
                        widget: &mut W,
                        state: &mut W::State,
                        expand: bool,
                    ) {
                        let widget_length = if expand {
                            self.expand_length
                        } else {
                            state.min_size().$primary_size
                        };

                        let extra_layers = state.extra_layers();

                        if extra_layers >= self.cursor_layer {
                            let ret = widget.handle_cursor_input(
                                state,
                                self.env,
                                Rect::from_origin_size(self.pos, Size {
                                    $primary_size: widget_length,
                                    $secondary_size: self.size.$secondary_size,
                                }),
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

                        self.pos.$primary_axis += widget_length;
                        self.i += 1;
                    }
                }

                let mut handler = CursorInputHandler {
                    env,
                    pos: rect.origin(),
                    size: rect.size(),
                    expand_length,
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

                self.content.all(&mut state.content_state, &mut handler);

                InputReturn {
                    demand_focus: handler.demand_focus,
                }
            }

            fn handle_keyboard_input(
                &mut self,
                state: &mut Self::State,
                env: &mut E,
                rect: Rect,
                input: &KeyboardInput,
                input_state: &InputState,
                theme: &Theme,
                focus: bool,
            ) {
                struct KeyboardInputHandler<'a, E> {
                    env: &'a mut E,
                    pos: Point,
                    size: Size,
                    expand_length: f64,
                    input: &'a KeyboardInput,
                    input_state: &'a InputState,
                    theme: &'a Theme,
                    focus: Option<u16>,
                    i: u16,
                }

                impl<'a, E> FlexContentHandler<E> for KeyboardInputHandler<'a, E> {
                    fn widget<W: Widget<E>>(
                        &mut self,
                        widget: &mut W,
                        state: &mut W::State,
                        expand: bool,
                    ) {
                        let widget_length = if expand {
                            self.expand_length
                        } else {
                            state.min_size().$primary_size
                        };

                        let focus = self.focus == Some(self.i);
                        if focus {
                            widget.handle_keyboard_input(
                                state,
                                self.env,
                                Rect::from_origin_size(self.pos, Size {
                                    $primary_size: widget_length,
                                    $secondary_size: self.size.$secondary_size,
                                }),
                                self.input,
                                self.input_state,
                                self.theme,
                                focus,
                            );
                        }

                        self.pos.$primary_axis += widget_length;
                        self.i += 1;
                    }
                }

                let min_length = state.no_expand_size;

                let extra_width = rect.$primary_size() - min_length;
                let expand_length = extra_width / state.expand_count as f64;

                let mut handler = KeyboardInputHandler {
                    env,
                    pos: rect.origin(),
                    size: rect.size(),
                    expand_length,
                    input,
                    input_state,
                    theme,
                    focus: if focus { state.focus } else { None },
                    i: 0,
                };

                self.content.all(&mut state.content_state, &mut handler);
            }
        }
    };
}

flex!(Row, RowState, row, x, y, width, height);
flex!(Col, ColState, col, y, x, height, width);
