use druid_shell::kurbo;
use druid_shell::kurbo::{Rect, Size};
use piet_common::Piet;

use super::NoneWidget;
pub use crate::theme::*;
use crate::{InputState, LayoutConstraint, Widget};

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
    A(A),
    B(B),
    C(C),
    D(D),
}

impl<A, B, C, D> Widget for OrWidget<A, B, C, D>
where
    A: Widget,
    B: Widget,
    C: Widget,
    D: Widget,
{
    type State = OrState<A::State, B::State, C::State, D::State>;

    fn build(
        &mut self,
        constraint: LayoutConstraint,
        renderer: &mut Piet,
        theme: &Theme,
    ) -> Self::State {
        match self {
            OrWidget::A(e) => OrState::A(A::build(e, constraint, renderer, theme)),
            OrWidget::B(e) => OrState::B(B::build(e, constraint, renderer, theme)),
            OrWidget::C(e) => OrState::C(C::build(e, constraint, renderer, theme)),
            OrWidget::D(e) => OrState::D(D::build(e, constraint, renderer, theme)),
        }
    }

    fn update(
        &mut self,
        state: &mut Self::State,
        constraint: LayoutConstraint,
        renderer: &mut Piet,
        theme: &Theme,
    ) {
        match self {
            OrWidget::A(e) => {
                if let OrState::A(w) = state {
                    e.update(w, constraint, renderer, theme);
                } else {
                    *state = OrState::A(e.build(constraint, renderer, theme));
                }
            }
            OrWidget::B(e) => {
                if let OrState::B(w) = state {
                    e.update(w, constraint, renderer, theme);
                } else {
                    *state = OrState::B(e.build(constraint, renderer, theme));
                }
            }
            OrWidget::C(e) => {
                if let OrState::C(w) = state {
                    e.update(w, constraint, renderer, theme);
                } else {
                    *state = OrState::C(e.build(constraint, renderer, theme));
                }
            }
            OrWidget::D(e) => {
                if let OrState::D(w) = state {
                    e.update(w, constraint, renderer, theme);
                } else {
                    *state = OrState::D(e.build(constraint, renderer, theme));
                }
            }
        }
    }

    fn min_size(&self, state: &Self::State) -> Size {
        match self {
            OrWidget::A(w) => w.min_size(state.as_a().unwrap()),
            OrWidget::B(w) => w.min_size(state.as_b().unwrap()),
            OrWidget::C(w) => w.min_size(state.as_c().unwrap()),
            OrWidget::D(w) => w.min_size(state.as_d().unwrap()),
        }
    }

    fn extra_layers(&self, state: &Self::State) -> u8 {
        match self {
            OrWidget::A(w) => w.extra_layers(state.as_a().unwrap()),
            OrWidget::B(w) => w.extra_layers(state.as_b().unwrap()),
            OrWidget::C(w) => w.extra_layers(state.as_c().unwrap()),
            OrWidget::D(w) => w.extra_layers(state.as_d().unwrap()),
        }
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
        use OrWidget::*;

        match self {
            A(w) => w.render(
                state.as_a_mut().unwrap(),
                rect,
                renderer,
                theme,
                input_state,
                layer,
                focus,
            ),
            B(w) => w.render(
                state.as_b_mut().unwrap(),
                rect,
                renderer,
                theme,
                input_state,
                layer,
                focus,
            ),
            C(w) => w.render(
                state.as_c_mut().unwrap(),
                rect,
                renderer,
                theme,
                input_state,
                layer,
                focus,
            ),
            D(w) => w.render(
                state.as_d_mut().unwrap(),
                rect,
                renderer,
                theme,
                input_state,
                layer,
                focus,
            ),
        }
    }

    fn test_input_pos_layer(
        &mut self,
        state: &mut Self::State,
        rect: Rect,
        input_pos: kurbo::Point,
    ) -> Option<u8> {
        use OrWidget::*;

        match self {
            A(w) => w.test_input_pos_layer(state.as_a_mut().unwrap(), rect, input_pos),
            B(w) => w.test_input_pos_layer(state.as_b_mut().unwrap(), rect, input_pos),
            C(w) => w.test_input_pos_layer(state.as_c_mut().unwrap(), rect, input_pos),
            D(w) => w.test_input_pos_layer(state.as_d_mut().unwrap(), rect, input_pos),
        }
    }

    fn handle_cursor_input(
        &mut self,
        state: &mut Self::State,
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
            A(w) => {
                w.handle_cursor_input(
                    state.as_a_mut().unwrap(),
                    rect,
                    cursor_pos,
                    cursor_layer,
                    input,
                    input_state,
                    theme,
                    focus,
                )
            }
            B(w) => {
                w.handle_cursor_input(
                    state.as_b_mut().unwrap(),
                    rect,
                    cursor_pos,
                    cursor_layer,
                    input,
                    input_state,
                    theme,
                    focus,
                )
            }
            C(w) => {
                w.handle_cursor_input(
                    state.as_c_mut().unwrap(),
                    rect,
                    cursor_pos,
                    cursor_layer,
                    input,
                    input_state,
                    theme,
                    focus,
                )
            }
            D(w) => {
                w.handle_cursor_input(
                    state.as_d_mut().unwrap(),
                    rect,
                    cursor_pos,
                    cursor_layer,
                    input,
                    input_state,
                    theme,
                    focus,
                )
            }
        }
    }

    fn handle_keyboard_input(
        &mut self,
        state: &mut Self::State,
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
