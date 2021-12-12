use druid_shell::kurbo;
use druid_shell::kurbo::{Rect, Size};
use piet_common::Piet;

use super::NoneWidget;
pub use crate::theme::*;
use crate::{InputState, LayoutConstraint, Widget, WidgetParams};

/// More variants can be added here in the future. This is obviously a
/// suboptimal solution for the problem, the best solution would require
/// variadic generics or perhaps a macro that generates variants as needed
/// (it's unclear to me if or how that would work with a declarative macro).
pub enum OrElem<A = NoneWidget, B = NoneWidget, C = NoneWidget, D = NoneWidget> {
    A(A),
    B(B),
    C(C),
    D(D),
}

impl<A, B, C, D> OrElem<A, B, C, D> {
    pub fn as_a_mut(&mut self) -> Option<&mut A> {
        if let OrElem::A(w) = self {
            Some(w)
        } else {
            None
        }
    }

    pub fn as_b_mut(&mut self) -> Option<&mut B> {
        if let OrElem::B(w) = self {
            Some(w)
        } else {
            None
        }
    }

    pub fn as_c_mut(&mut self) -> Option<&mut C> {
        if let OrElem::C(w) = self {
            Some(w)
        } else {
            None
        }
    }

    pub fn as_d_mut(&mut self) -> Option<&mut D> {
        if let OrElem::D(w) = self {
            Some(w)
        } else {
            None
        }
    }
}

impl<A, B, C, D> WidgetParams for OrElem<A, B, C, D>
where
    A: WidgetParams,
    B: WidgetParams,
    C: WidgetParams,
    D: WidgetParams,
{
    type Widget = OrWidget<A::Widget, B::Widget, C::Widget, D::Widget>;
}

pub enum OrWidget<A = NoneWidget, B = NoneWidget, C = NoneWidget, D = NoneWidget> {
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
    type Params = OrElem<A::Params, B::Params, C::Params, D::Params>;
    type Event = ();

    fn build(
        params: &mut Self::Params,
        constraint: LayoutConstraint,
        renderer: &mut Piet,
        theme: &Theme,
    ) -> Self {
        match params {
            OrElem::A(e) => OrWidget::A(A::build(
                e,
                constraint,
                renderer,
                theme,
            )),
            OrElem::B(e) => OrWidget::B(B::build(
                e,
                constraint,
                renderer,
                theme,
            )),
            OrElem::C(e) => OrWidget::C(C::build(
                e,
                constraint,
                renderer,
                theme,
            )),
            OrElem::D(e) => OrWidget::D(D::build(
                e,
                constraint,
                renderer,
                theme,
            )),
        }
    }

    fn update(
        &mut self,
        params: &mut Self::Params,
        constraint: LayoutConstraint,
        renderer: &mut Piet,
        theme: &Theme,
    ) {
        match params {
            OrElem::A(e) => {
                if let OrWidget::A(w) = self {
                    w.update(e, constraint, renderer, theme);
                } else {
                    *self = OrWidget::A(A::build(e, constraint, renderer, theme));
                }
            }
            OrElem::B(e) => {
                if let OrWidget::B(w) = self {
                    w.update(e, constraint, renderer, theme);
                } else {
                    *self = OrWidget::B(B::build(e, constraint, renderer, theme));
                }
            }
            OrElem::C(e) => {
                if let OrWidget::C(w) = self {
                    w.update(e, constraint, renderer, theme);
                } else {
                    *self = OrWidget::C(C::build(e, constraint, renderer, theme));
                }
            }
            OrElem::D(e) => {
                if let OrWidget::D(w) = self {
                    w.update(e, constraint, renderer, theme);
                } else {
                    *self = OrWidget::D(D::build(e, constraint, renderer, theme));
                }
            }
        }
    }

    fn min_size(&self) -> Size {
        match self {
            OrWidget::A(w) => w.min_size(),
            OrWidget::B(w) => w.min_size(),
            OrWidget::C(w) => w.min_size(),
            OrWidget::D(w) => w.min_size(),
        }
    }

    fn extra_layers(&self) -> u8 {
        match self {
            OrWidget::A(w) => w.extra_layers(),
            OrWidget::B(w) => w.extra_layers(),
            OrWidget::C(w) => w.extra_layers(),
            OrWidget::D(w) => w.extra_layers(),
        }
    }

    fn render(
        &mut self,
        params: &mut Self::Params,
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
                params.as_a_mut().unwrap(),
                rect,
                renderer,
                theme,
                input_state,
                layer,
                focus,
            ),
            B(w) => w.render(
                params.as_b_mut().unwrap(),
                rect,
                renderer,
                theme,
                input_state,
                layer,
                focus,
            ),
            C(w) => w.render(
                params.as_c_mut().unwrap(),
                rect,
                renderer,
                theme,
                input_state,
                layer,
                focus,
            ),
            D(w) => w.render(
                params.as_d_mut().unwrap(),
                rect,
                renderer,
                theme,
                input_state,
                layer,
                focus,
            ),
        }
    }

    fn test_input_pos_layer(&mut self, params: &mut Self::Params, rect: Rect, input_pos: kurbo::Point) -> Option<u8> {
        use OrWidget::*;

        match self {
            A(w) => w.test_input_pos_layer(params.as_a_mut().unwrap(), rect, input_pos),
            B(w) => w.test_input_pos_layer(params.as_b_mut().unwrap(), rect, input_pos),
            C(w) => w.test_input_pos_layer(params.as_c_mut().unwrap(), rect, input_pos),
            D(w) => w.test_input_pos_layer(params.as_d_mut().unwrap(), rect, input_pos),
        }
    }

    fn handle_cursor_input(
        &mut self,
        params: &mut Self::Params,
        rect: Rect,
        cursor_pos: kurbo::Point,
        cursor_layer: u8,
        input: crate::CursorInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
    ) -> (crate::InputReturn, Option<Self::Event>) {
        use OrWidget::*;

        (
            match self {
                A(w) => {
                    w.handle_cursor_input(
                        params.as_a_mut().unwrap(),
                        rect,
                        cursor_pos,
                        cursor_layer,
                        input,
                        input_state,
                        theme,
                        focus,
                    )
                    .0
                }
                B(w) => {
                    w.handle_cursor_input(
                        params.as_b_mut().unwrap(),
                        rect,
                        cursor_pos,
                        cursor_layer,
                        input,
                        input_state,
                        theme,
                        focus,
                    )
                    .0
                }
                C(w) => {
                    w.handle_cursor_input(
                        params.as_c_mut().unwrap(),
                        rect,
                        cursor_pos,
                        cursor_layer,
                        input,
                        input_state,
                        theme,
                        focus,
                    )
                    .0
                }
                D(w) => {
                    w.handle_cursor_input(
                        params.as_d_mut().unwrap(),
                        rect,
                        cursor_pos,
                        cursor_layer,
                        input,
                        input_state,
                        theme,
                        focus,
                    )
                    .0
                }
            },
            None,
        )
    }

    fn handle_keyboard_input(
        &mut self,
        params: &mut Self::Params,
        rect: Rect,
        input: &crate::KeyboardInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
    ) -> Option<Self::Event> {
        use OrWidget::*;

        match self {
            A(w) => {
                w.handle_keyboard_input(params.as_a_mut().unwrap(), rect, input, input_state, theme, focus);
            }
            B(w) => {
                w.handle_keyboard_input(params.as_b_mut().unwrap(), rect, input, input_state, theme, focus);
            }
            C(w) => {
                w.handle_keyboard_input(params.as_c_mut().unwrap(), rect, input, input_state, theme, focus);
            }
            D(w) => {
                w.handle_keyboard_input(params.as_d_mut().unwrap(), rect, input, input_state, theme, focus);
            }
        }

        None
    }
}
