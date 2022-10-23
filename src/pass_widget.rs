use crate::*;

pub enum Pass<'la, 'lb, 'ra, 'rb, 'rc, 't, 'is, 'ka, T> {
    Layout {
        ctx: &'la mut LayoutCtx<'lb, 't>,
        constraint: LayoutConstraint,
        ret: fn() -> T,
    }, // min size comes from layout ???

    // can we make input a sub parameter of render
    // so do it in the same pass?
    // might not be a good idea because of frame tearing
    Render {
        ctx: &'ra mut RenderCtx<'rb, 'rc, 't, 'is>,
        rect: Rect,
        layer: u8,
        focus: bool,

        // or just T?
        ret: fn() -> T,
    },

    TestInputPosLayer {
        rect: Rect,
        input_pos: Point,
        ret: fn(Option<u8>) -> T,
    },

    // can we integrate this into the layout pass
    CursorInput {
        rect: Rect,
        cursor_pos: Point,
        cursor_layer: u8,
        input: CursorInput,
        input_state: &'is InputState,
        theme: &'t Theme,
        focus: bool,

        ret: fn(InputReturn) -> T,
    },
    KeyboardInput {
        rect: Rect,
        input: &'ka KeyboardInput,
        input_state: &'is InputState,
        theme: &'t Theme,
        focus: bool,

        ret: fn() -> T,
    },
}

pub trait PassWidget<E> {
    type State: WidgetState;

    fn pass<R>(&mut self, state: &mut Self::State, env: &mut E, pass: Pass<R>) -> R;
}

pub struct PassWidgetWidget<W>(pub W);

impl<E, W: PassWidget<E>> Widget<E> for PassWidgetWidget<W> {
    type State = W::State;

    fn layout(
        &mut self,
        state: &mut W::State,
        env: &mut E,
        constraint: LayoutConstraint,
        ctx: &mut LayoutCtx,
    ) {
        self.0.pass(
            state,
            env,
            Pass::Layout {
                ctx,
                constraint,
                ret: || (),
            },
        )
    }

    fn render(
        &mut self,
        state: &mut W::State,
        env: &mut E,
        rect: Rect,
        layer: u8,
        focus: bool,
        ctx: &mut RenderCtx,
    ) {
        self.0.pass(
            state,
            env,
            Pass::Render {
                ctx,
                rect,
                layer,
                focus,
                ret: || (),
            },
        )
    }

    fn test_input_pos_layer(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        rect: Rect,
        input_pos: Point,
    ) -> Option<u8> {
        self.0.pass(
            state,
            env,
            Pass::TestInputPosLayer {
                rect,
                input_pos,
                ret: |r| r,
            },
        )
    }

    fn handle_cursor_input(
        &mut self,
        state: &mut W::State,
        env: &mut E,
        rect: Rect,
        cursor_pos: Point,
        cursor_layer: u8,
        input: CursorInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
    ) -> InputReturn {
        self.0.pass(
            state,
            env,
            Pass::CursorInput {
                rect,
                cursor_pos,
                cursor_layer,
                input,
                input_state,
                theme,
                focus,
                ret: |r| r,
            },
        )
    }

    fn handle_keyboard_input(
        &mut self,
        state: &mut W::State,
        env: &mut E,
        rect: Rect,
        input: &KeyboardInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
    ) {
        self.0.pass(
            state,
            env,
            Pass::KeyboardInput {
                rect,
                input,
                input_state,
                theme,
                focus,
                ret: || (),
            },
        )
    }
}

pub struct WidgetPassWidget<W>(pub W);

impl<E, W: Widget<E>> PassWidget<E> for WidgetPassWidget<W> {
    type State = W::State;

    fn pass<R>(&mut self, state: &mut Self::State, env: &mut E, pass: Pass<R>) -> R {
        use Pass::*;

        match pass {
            Layout {
                ctx,
                constraint,
                ret,
            } => {
                self.0.layout(state, env, constraint, ctx);
                ret()
            }

            Render {
                ctx,
                rect,
                layer,
                focus,
                ret,
            } => {
                self.0.render(state, env, rect, layer, focus, ctx);
                ret()
            }

            TestInputPosLayer {
                rect,
                input_pos,
                ret,
            } => ret(self.0.test_input_pos_layer(state, env, rect, input_pos)),

            CursorInput {
                rect,
                cursor_pos,
                cursor_layer,
                input,
                input_state,
                theme,
                focus,
                ret,
            } => ret(self.0.handle_cursor_input(
                state,
                env,
                rect,
                cursor_pos,
                cursor_layer,
                input,
                input_state,
                theme,
                focus,
            )),

            KeyboardInput {
                rect,
                input,
                input_state,
                theme,
                focus,
                ret,
            } => {
                self.0
                    .handle_keyboard_input(state, env, rect, input, input_state, theme, focus);
                ret()
            }
        }
    }
}
