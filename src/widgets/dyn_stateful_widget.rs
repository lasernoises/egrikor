use std::{any::Any, marker::PhantomData};

use crate::*;

pub fn stateful_widget<E, S: Default, B: Fn(StatefulWidgetHandler<E, S>)>(
    build: B,
) -> impl Widget<E, State = StatefulWidgetState<S>> {
    StatefulWidget {
        build,
        state: PhantomData,
    }
}

pub struct StatefulWidget<S, B> {
    build: B,
    state: PhantomData<fn(S)>,
}

pub struct StatefulWidgetState<S> {
    state: S,
    widget_state: Box<dyn Any>,
    min_size: Size,
    extra_layers: u8,
}

impl<S: Default> WidgetState for StatefulWidgetState<S> {
    fn new() -> Self {
        Self {
            state: S::default(),
            widget_state: Box::new(()),
            min_size: Size::ZERO,
            extra_layers: 0,
        }
    }

    fn min_size(&self) -> Size {
        self.min_size
    }

    fn extra_layers(&self) -> u8 {
        self.extra_layers
    }
}

impl<E, S: Default, B: Fn(StatefulWidgetHandler<E, S>)> Widget<E> for StatefulWidget<S, B> {
    type State = StatefulWidgetState<S>;

    fn layout(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        constraint: LayoutConstraint,
        ctx: &mut LayoutCtx,
    ) {
        (self.build)(StatefulWidgetHandler {
            env,
            state,
            handler: BuiltStatefulWidgetHandler::Update { constraint, ctx },
        });
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
        (self.build)(StatefulWidgetHandler {
            env,
            state,
            handler: BuiltStatefulWidgetHandler::Render {
                rect,
                layer,
                focus,
                ctx,
            },
        });
    }

    fn test_input_pos_layer(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        rect: Rect,
        input_pos: Point,
    ) -> Option<u8> {
        let mut result = None;
        (self.build)(StatefulWidgetHandler {
            env,
            state,
            handler: BuiltStatefulWidgetHandler::TestInputPosLayer {
                rect,
                input_pos,
                result: &mut result,
            },
        });

        result
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
        let mut result = None;
        (self.build)(StatefulWidgetHandler {
            env,
            state,
            handler: BuiltStatefulWidgetHandler::HandleCursorInput {
                rect,
                cursor_pos,
                cursor_layer,
                input,
                input_state,
                theme,
                focus,
                result: &mut result,
            },
        });

        result.unwrap()
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
        (self.build)(StatefulWidgetHandler {
            env,
            state,
            handler: BuiltStatefulWidgetHandler::HandleKeyboardInput {
                rect,
                input,
                input_state,
                theme,
                focus,
            },
        });
    }
}

pub struct StatefulWidgetHandler<'e, 's, 'c, 'ca, 'cb, 'r, E, S> {
    env: &'e mut E,
    state: &'s mut StatefulWidgetState<S>,
    handler: BuiltStatefulWidgetHandler<'c, 'ca, 'cb, 'r>,
}

struct BuildResult {
    widget_state: Box<dyn Any>,
    min_size: Size,
    extra_layers: u8,
}

enum BuiltStatefulWidgetHandler<'c, 'ca, 'cb, 'r> {
    Update {
        constraint: LayoutConstraint,
        ctx: &'c mut LayoutCtx<'ca>,
    },
    Render {
        rect: Rect,
        layer: u8,
        focus: bool,
        ctx: &'c mut RenderCtx<'ca, 'cb>,
    },
    TestInputPosLayer {
        rect: Rect,
        input_pos: Point,
        result: &'r mut Option<u8>,
    },
    HandleCursorInput {
        rect: Rect,
        cursor_pos: Point,
        cursor_layer: u8,
        input: CursorInput,
        input_state: &'ca InputState,
        theme: &'ca Theme,
        focus: bool,
        result: &'r mut Option<InputReturn>,
    },
    HandleKeyboardInput {
        rect: Rect,
        input: &'ca KeyboardInput,
        input_state: &'ca InputState,
        theme: &'ca Theme,
        focus: bool,
    },
}

impl<'e, 's, 'c, 'ca, 'cb, 'r, E, S> StatefulWidgetHandler<'e, 's, 'c, 'ca, 'cb, 'r, E, S> {
    pub fn state(&self) -> &S {
        &self.state.state
    }

    pub fn widget<WS: 'static + WidgetState, W: Widget<(&'s mut S, &'e mut E), State = WS>>(
        self,
        mut widget: W,
    ) {
        use BuiltStatefulWidgetHandler::*;

        let StatefulWidgetHandler {
            env,
            state,
            handler,
        } = self;

        match handler {
            Update { constraint, ctx } => {
                let widget_state =
                    if let Some(widget_state) = state.widget_state.downcast_mut::<WS>() {
                        widget_state
                    } else {
                        state.widget_state = Box::new(WS::new());
                        state.widget_state.downcast_mut::<WS>().unwrap()
                    };

                widget.layout(widget_state, &mut (&mut state.state, env), constraint, ctx);

                state.min_size = widget_state.min_size();
                state.extra_layers = widget_state.extra_layers();
            }
            Render {
                rect,
                layer,
                focus,
                ctx,
            } => {
                let widget_state = state.widget_state.downcast_mut::<WS>().unwrap();

                widget.render(
                    widget_state,
                    &mut (&mut state.state, env),
                    rect,
                    layer,
                    focus,
                    ctx,
                );
            }
            TestInputPosLayer {
                rect,
                input_pos,
                result,
            } => {
                let widget_state = state.widget_state.downcast_mut::<WS>().unwrap();

                *result = widget.test_input_pos_layer(
                    widget_state,
                    &mut (&mut state.state, env),
                    rect,
                    input_pos,
                );
            }
            HandleCursorInput {
                rect,
                cursor_pos,
                cursor_layer,
                input,
                input_state,
                theme,
                focus,
                result,
            } => {
                let widget_state = state.widget_state.downcast_mut::<WS>().unwrap();

                *result = Some(widget.handle_cursor_input(
                    widget_state,
                    &mut (&mut state.state, env),
                    rect,
                    cursor_pos,
                    cursor_layer,
                    input,
                    input_state,
                    theme,
                    focus,
                ));
            }
            HandleKeyboardInput {
                rect,
                input,
                input_state,
                theme,
                focus,
            } => {
                let widget_state = state.widget_state.downcast_mut::<WS>().unwrap();

                widget.handle_keyboard_input(
                    widget_state,
                    &mut (&mut state.state, env),
                    rect,
                    input,
                    input_state,
                    theme,
                    focus,
                );
            }
        }
    }
}
