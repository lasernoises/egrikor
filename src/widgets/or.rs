use druid_shell::kurbo;
use druid_shell::kurbo::{Rect, Size};
use piet_common::Piet;

use super::NoneWidget;
pub use crate::theme::*;
use crate::{InputState, LayoutConstraint, LayoutCtx, RenderCtx, Widget, WidgetState};

/// More variants can be added here in the future. This is obviously a
/// suboptimal solution for the problem, the best solution would require
/// variadic generics or perhaps a macro that generates variants as needed
/// (it's unclear to me if or how that would work with a declarative macro).
pub enum OrWidget<A = NoneWidget, B = NoneWidget, C = NoneWidget, D = NoneWidget> {
    A(A),
    B(B),
    C(C),
    D(D),
}

impl<A, B, C, D> OrState<A, B, C, D> {
    pub fn as_a_mut(&mut self) -> Option<&mut A> {
        if let OrState::A(w) = self {
            Some(w)
        } else {
            None
        }
    }

    pub fn as_b_mut(&mut self) -> Option<&mut B> {
        if let OrState::B(w) = self {
            Some(w)
        } else {
            None
        }
    }

    pub fn as_c_mut(&mut self) -> Option<&mut C> {
        if let OrState::C(w) = self {
            Some(w)
        } else {
            None
        }
    }

    pub fn as_d_mut(&mut self) -> Option<&mut D> {
        if let OrState::D(w) = self {
            Some(w)
        } else {
            None
        }
    }

    pub fn as_a(&self) -> Option<&A> {
        if let OrState::A(w) = self {
            Some(w)
        } else {
            None
        }
    }

    pub fn as_b(&self) -> Option<&B> {
        if let OrState::B(w) = self {
            Some(w)
        } else {
            None
        }
    }

    pub fn as_c(&self) -> Option<&C> {
        if let OrState::C(w) = self {
            Some(w)
        } else {
            None
        }
    }

    pub fn as_d(&self) -> Option<&D> {
        if let OrState::D(w) = self {
            Some(w)
        } else {
            None
        }
    }
}

pub enum OrState<A = (), B = (), C = (), D = ()> {
    None,
    A(A),
    B(B),
    C(C),
    D(D),
}

impl<A: WidgetState, B: WidgetState, C: WidgetState, D: WidgetState> WidgetState
    for OrState<A, B, C, D>
{
    fn new() -> Self {
        Self::None
    }

    fn min_size(&self) -> Size {
        match self {
            Self::None => panic!(),
            Self::A(s) => s.min_size(),
            Self::B(s) => s.min_size(),
            Self::C(s) => s.min_size(),
            Self::D(s) => s.min_size(),
        }
    }

    fn extra_layers(&self) -> u8 {
        match self {
            Self::None => panic!(),
            Self::A(s) => s.extra_layers(),
            Self::B(s) => s.extra_layers(),
            Self::C(s) => s.extra_layers(),
            Self::D(s) => s.extra_layers(),
        }
    }
}

impl<E, A, B, C, D> Widget<E> for OrWidget<A, B, C, D>
where
    A: Widget<E>,
    B: Widget<E>,
    C: Widget<E>,
    D: Widget<E>,
{
    type State = OrState<A::State, B::State, C::State, D::State>;

    fn layout(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        constraint: LayoutConstraint,
        ctx: &mut LayoutCtx,
    ) {
        match self {
            OrWidget::A(e) => {
                let state = if let OrState::A(w) = state {
                    w
                } else {
                    *state = OrState::A(A::State::new());
                    state.as_a_mut().unwrap()
                };
                e.layout(state, env, constraint, ctx);
            }
            OrWidget::B(e) => {
                let state = if let OrState::B(w) = state {
                    w
                } else {
                    *state = OrState::B(B::State::new());
                    state.as_b_mut().unwrap()
                };
                e.layout(state, env, constraint, ctx);
            }
            OrWidget::C(e) => {
                let state = if let OrState::C(w) = state {
                    w
                } else {
                    *state = OrState::C(C::State::new());
                    state.as_c_mut().unwrap()
                };
                e.layout(state, env, constraint, ctx);
            }
            OrWidget::D(e) => {
                let state = if let OrState::D(w) = state {
                    w
                } else {
                    *state = OrState::D(D::State::new());
                    state.as_d_mut().unwrap()
                };
                e.layout(state, env, constraint, ctx);
            }
        }
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
        use OrWidget::*;

        match self {
            A(w) => w.render(state.as_a_mut().unwrap(), env, rect, layer, focus, ctx),
            B(w) => w.render(state.as_b_mut().unwrap(), env, rect, layer, focus, ctx),
            C(w) => w.render(state.as_c_mut().unwrap(), env, rect, layer, focus, ctx),
            D(w) => w.render(state.as_d_mut().unwrap(), env, rect, layer, focus, ctx),
        }
    }

    fn test_input_pos_layer(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        rect: Rect,
        input_pos: kurbo::Point,
    ) -> Option<u8> {
        use OrWidget::*;

        match self {
            A(w) => w.test_input_pos_layer(state.as_a_mut().unwrap(), env, rect, input_pos),
            B(w) => w.test_input_pos_layer(state.as_b_mut().unwrap(), env, rect, input_pos),
            C(w) => w.test_input_pos_layer(state.as_c_mut().unwrap(), env, rect, input_pos),
            D(w) => w.test_input_pos_layer(state.as_d_mut().unwrap(), env, rect, input_pos),
        }
    }

    fn handle_cursor_input(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        rect: Rect,
        cursor_pos: kurbo::Point,
        cursor_layer: u8,
        input: crate::CursorInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
    ) -> crate::InputReturn {
        use OrWidget::*;

        match self {
            A(w) => w.handle_cursor_input(
                state.as_a_mut().unwrap(),
                env,
                rect,
                cursor_pos,
                cursor_layer,
                input,
                input_state,
                theme,
                focus,
            ),
            B(w) => w.handle_cursor_input(
                state.as_b_mut().unwrap(),
                env,
                rect,
                cursor_pos,
                cursor_layer,
                input,
                input_state,
                theme,
                focus,
            ),
            C(w) => w.handle_cursor_input(
                state.as_c_mut().unwrap(),
                env,
                rect,
                cursor_pos,
                cursor_layer,
                input,
                input_state,
                theme,
                focus,
            ),
            D(w) => w.handle_cursor_input(
                state.as_d_mut().unwrap(),
                env,
                rect,
                cursor_pos,
                cursor_layer,
                input,
                input_state,
                theme,
                focus,
            ),
        }
    }

    fn handle_keyboard_input(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        rect: Rect,
        input: &crate::KeyboardInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
    ) {
        use OrWidget::*;

        match self {
            A(w) => {
                w.handle_keyboard_input(
                    state.as_a_mut().unwrap(),
                    env,
                    rect,
                    input,
                    input_state,
                    theme,
                    focus,
                );
            }
            B(w) => {
                w.handle_keyboard_input(
                    state.as_b_mut().unwrap(),
                    env,
                    rect,
                    input,
                    input_state,
                    theme,
                    focus,
                );
            }
            C(w) => {
                w.handle_keyboard_input(
                    state.as_c_mut().unwrap(),
                    env,
                    rect,
                    input,
                    input_state,
                    theme,
                    focus,
                );
            }
            D(w) => {
                w.handle_keyboard_input(
                    state.as_d_mut().unwrap(),
                    env,
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
