use std::fmt::Debug;

use druid_shell::kurbo;
use druid_shell::kurbo::{Rect, Size};
use piet_common::Piet;

use super::{NoneElement, NoneWidget};
pub use crate::theme::*;
use crate::{Element, InputState, Widget};

/// More variants can be added here in the future. This is obviously a
/// suboptimal solution for the problem, the best solution would require
/// variadic generics or perhaps a macro that generates variants as needed
/// (it's unclear to me if or how that would work with a declarative macro).
pub enum OrElem<A = NoneElement, B = NoneElement, C = NoneElement, D = NoneElement> {
    A(A),
    B(B),
    C(C),
    D(D),
}

impl<A, B, C, D, E> Element<E> for OrElem<A, B, C, D>
where
    A: Element<E>,
    B: Element<E>,
    C: Element<E>,
    D: Element<E>,
    E: Debug,
{
    type Widget = OrWidget<A::Widget, B::Widget, C::Widget, D::Widget>;

    fn build(self) -> Self::Widget {
        match self {
            OrElem::A(e) => OrWidget::A(e.build()),
            OrElem::B(e) => OrWidget::B(e.build()),
            OrElem::C(e) => OrWidget::C(e.build()),
            OrElem::D(e) => OrWidget::D(e.build()),
        }
    }

    fn update(self, widget: &mut Self::Widget) {
        match self {
            OrElem::A(e) => {
                if let OrWidget::A(w) = widget {
                    e.update(w);
                } else {
                    *widget = OrWidget::A(e.build());
                }
            }
            OrElem::B(e) => {
                if let OrWidget::B(w) = widget {
                    e.update(w);
                } else {
                    *widget = OrWidget::B(e.build());
                }
            }
            OrElem::C(e) => {
                if let OrWidget::C(w) = widget {
                    e.update(w);
                } else {
                    *widget = OrWidget::C(e.build());
                }
            }
            OrElem::D(e) => {
                if let OrWidget::D(w) = widget {
                    e.update(w);
                } else {
                    *widget = OrWidget::D(e.build());
                }
            }
        }
    }
}

pub enum OrWidget<A = NoneWidget, B = NoneWidget, C = NoneWidget, D = NoneWidget> {
    A(A),
    B(B),
    C(C),
    D(D),
}

impl<A, B, C, D, E> Widget<E> for OrWidget<A, B, C, D>
where
    A: Widget<E>,
    B: Widget<E>,
    C: Widget<E>,
    D: Widget<E>,
    E: Debug,
{
    type Event = ();

    fn measure(
        &mut self,
        max_size: [Option<f64>; 2],
        renderer: &mut Piet,
        theme: &Theme,
        context: &mut E,
    ) -> Size {
        match self {
            OrWidget::A(w) => w.measure(max_size, renderer, theme, context),
            OrWidget::B(w) => w.measure(max_size, renderer, theme, context),
            OrWidget::C(w) => w.measure(max_size, renderer, theme, context),
            OrWidget::D(w) => w.measure(max_size, renderer, theme, context),
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
        rect: Rect,
        renderer: &mut Piet,
        theme: &Theme,
        input_state: &InputState,
        layer: u8,
        focus: bool,
        context: &mut E,
    ) {
        use OrWidget::*;

        match self {
            A(w) => w.render(rect, renderer, theme, input_state, layer, focus, context),
            B(w) => w.render(rect, renderer, theme, input_state, layer, focus, context),
            C(w) => w.render(rect, renderer, theme, input_state, layer, focus, context),
            D(w) => w.render(rect, renderer, theme, input_state, layer, focus, context),
        }
    }

    fn test_input_pos_layer(&mut self, rect: Rect, input_pos: kurbo::Point) -> Option<u8> {
        use OrWidget::*;

        match self {
            A(w) => w.test_input_pos_layer(rect, input_pos),
            B(w) => w.test_input_pos_layer(rect, input_pos),
            C(w) => w.test_input_pos_layer(rect, input_pos),
            D(w) => w.test_input_pos_layer(rect, input_pos),
        }
    }

    fn handle_cursor_input(
        &mut self,
        rect: Rect,
        cursor_pos: kurbo::Point,
        cursor_layer: u8,
        input: crate::CursorInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
        context: &mut E,
    ) -> (crate::InputReturn, Option<Self::Event>) {
        use OrWidget::*;

        (
            match self {
                A(w) => {
                    w.handle_cursor_input(
                        rect,
                        cursor_pos,
                        cursor_layer,
                        input,
                        input_state,
                        theme,
                        focus,
                        context,
                    )
                    .0
                }
                B(w) => {
                    w.handle_cursor_input(
                        rect,
                        cursor_pos,
                        cursor_layer,
                        input,
                        input_state,
                        theme,
                        focus,
                        context,
                    )
                    .0
                }
                C(w) => {
                    w.handle_cursor_input(
                        rect,
                        cursor_pos,
                        cursor_layer,
                        input,
                        input_state,
                        theme,
                        focus,
                        context,
                    )
                    .0
                }
                D(w) => {
                    w.handle_cursor_input(
                        rect,
                        cursor_pos,
                        cursor_layer,
                        input,
                        input_state,
                        theme,
                        focus,
                        context,
                    )
                    .0
                }
            },
            None,
        )
    }

    fn handle_keyboard_input(
        &mut self,
        rect: Rect,
        input: &crate::KeyboardInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
        context: &mut E,
    ) -> Option<Self::Event> {
        use OrWidget::*;

        match self {
            A(w) => {
                w.handle_keyboard_input(rect, input, input_state, theme, focus, context);
            }
            B(w) => {
                w.handle_keyboard_input(rect, input, input_state, theme, focus, context);
            }
            C(w) => {
                w.handle_keyboard_input(rect, input, input_state, theme, focus, context);
            }
            D(w) => {
                w.handle_keyboard_input(rect, input, input_state, theme, focus, context);
            }
        }

        None
    }
}
