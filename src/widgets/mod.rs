pub mod button;
pub mod drawables;
pub mod lists;
pub mod or;
pub mod textbox;

use std::fmt::Debug;

use druid_shell::kurbo;
use druid_shell::kurbo::{Rect, Size};
use druid_shell::piet::RenderContext;
use piet_common::Piet;

use super::{Element, InputState, Widget};
pub use crate::theme::*;

pub fn contextualize<A: Debug, B: Debug, F: Fn(&mut A) -> &mut B, E: Element<B>>(
    f: F,
    element: E,
) -> impl Element<A> {
    ContextualizeElement(f, element)
}

pub struct ContextualizeElement<F, E>(F, E);

impl<A: Debug, B: Debug, F: Fn(&mut A) -> &mut B, E: Element<B>> Element<A>
    for ContextualizeElement<F, E>
{
    type Widget = ContextualizeWidget<F, E::Widget>;

    fn build(self) -> Self::Widget {
        ContextualizeWidget(self.0, self.1.build())
    }

    fn update(self, widget: &mut Self::Widget) {
        widget.0 = self.0;
        self.1.update(&mut widget.1);
    }
}

pub struct ContextualizeWidget<F, W>(F, W);

impl<A: Debug, B: Debug, F: Fn(&mut A) -> &mut B, W: Widget<B>> Widget<A>
    for ContextualizeWidget<F, W>
{
    type Event = W::Event;

    fn measure(
        &mut self,
        max_size: [Option<f64>; 2],
        renderer: &mut Piet,
        theme: &Theme,
        context: &mut A,
    ) -> Size {
        let context = (self.0)(context);
        self.1.measure(max_size, renderer, theme, context)
    }

    fn min_size(&self) -> Size {
        self.1.min_size()
    }

    fn extra_layers(&self) -> u8 {
        self.1.extra_layers()
    }

    fn render(
        &mut self,
        rect: Rect,
        renderer: &mut Piet,
        theme: &Theme,
        input_state: &InputState,
        layer: u8,
        focus: bool,
        context: &mut A,
    ) {
        let context = (self.0)(context);
        self.1
            .render(rect, renderer, theme, input_state, layer, focus, context)
    }

    fn test_input_pos_layer(&mut self, rect: Rect, input_pos: kurbo::Point) -> Option<u8> {
        self.1.test_input_pos_layer(rect, input_pos)
    }

    fn handle_cursor_input(
        &mut self,
        rect: Rect,
        cursor_pos: kurbo::Point,
        cursor_layer: u8,
        input: super::CursorInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
        context: &mut A,
    ) -> (super::InputReturn, Option<Self::Event>) {
        let context = (self.0)(context);
        self.1.handle_cursor_input(
            rect,
            cursor_pos,
            cursor_layer,
            input,
            input_state,
            theme,
            focus,
            context,
        )
    }

    fn handle_keyboard_input(
        &mut self,
        rect: Rect,
        input: &super::KeyboardInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
        context: &mut A,
    ) -> Option<Self::Event> {
        let context = (self.0)(context);
        self.1
            .handle_keyboard_input(rect, input, input_state, theme, focus, context)
    }
}

pub struct NoneWidget;

impl<C: Debug> Widget<C> for NoneWidget {
    type Event = ();

    fn measure(
        &mut self,
        _max_size: [Option<f64>; 2],
        _renderer: &mut Piet,
        _theme: &Theme,
        _context: &mut C,
    ) -> Size {
        Size::ZERO
    }

    fn min_size(&self) -> Size {
        Size::ZERO
    }
}

pub struct NoneElement;

impl<C: Debug> Element<C> for NoneElement {
    type Widget = NoneWidget;

    fn build(self) -> Self::Widget {
        NoneWidget
    }

    fn update(self, _widget: &mut Self::Widget) {}
}

pub const PADDING: f64 = 16.0;

pub const COLOR: u32 = 0x00_00_00_FF;
pub const COLOR_HOVER: u32 = 0x33_33_33_FF;
pub const COLOR_DOWN: u32 = 0x44_44_44_FF;

pub const BORDER_WIDTH: f64 = 2.0;

pub const BORDER_COLOR: u32 = 0xFF_FF_FF_FF;

pub fn measure_rect<C: Debug, D: Widget<C>>(
    drawable: &mut D,
    max_size: [Option<f64>; 2],
    renderer: &mut Piet,
    theme: &Theme,
    context: &mut C,
) -> Size {
    drawable.measure(
        [
            max_size[0].map(|w| 0f64.max(w - PADDING)),
            max_size[1].map(|h| 0f64.max(h - PADDING)),
        ],
        renderer,
        theme,
        context,
    ) + Size::new(PADDING * 2., PADDING * 2.)
}

pub fn render_rect<C: Debug, D: Widget<C>>(
    border: bool,
    hover: bool,

    drawable: &mut D,
    rect: Rect,
    renderer: &mut Piet,
    theme: &Theme,
    input_state: &InputState,
    layer: u8,
    focus: bool,
    context: &mut C,
) {
    if layer == 0 {
        let hover = hover
            && if let Some(point) = input_state.cursor_pos {
                rect.contains(point)
            } else {
                false
            };

        let brush = &renderer.solid_brush(piet_common::Color::Rgba32(
            match (hover, input_state.mouse_down) {
                (true, true) => COLOR_DOWN,
                (true, false) => COLOR_HOVER,
                (false, _) => COLOR,
            },
        ));
        renderer.fill(rect, brush);

        if border {
            let rect_pos = (rect.x0 + BORDER_WIDTH / 2.0, rect.y0 + BORDER_WIDTH / 2.0);
            let rect_size = (rect.width() - BORDER_WIDTH, rect.height() - BORDER_WIDTH);
            let rect_shape = kurbo::Rect::from_origin_size(rect_pos, rect_size);

            let brush = renderer.solid_brush(piet_common::Color::Rgba32(BORDER_COLOR));
            renderer.stroke(rect_shape, &brush, BORDER_WIDTH);
        }
    }

    drawable.render(
        rect.inset(-PADDING),
        renderer,
        theme,
        input_state,
        layer,
        focus,
        context,
    );
}
