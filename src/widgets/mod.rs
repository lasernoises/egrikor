// pub mod button;
// pub mod drawables;
// pub mod or;
pub mod button;
pub mod checkbox;
pub mod drawables;
pub mod dropdown;
pub mod dyn_stateful_widget;
pub mod lists;
pub mod or;
pub mod popup;
pub mod stateful_widget;
pub mod textbox;

use druid_shell::kurbo;
use druid_shell::kurbo::{Rect, Size};
use druid_shell::piet::RenderContext;
use piet_common::Piet;

pub use crate::theme::*;
use crate::{InputState, LayoutConstraint, LayoutCtx, RenderCtx, Widget, WidgetState};

#[derive(Copy, Clone, Default)]
pub struct NoneWidget;

pub struct NoneWidgetState;

impl WidgetState for NoneWidgetState {
    fn new() -> Self {
        NoneWidgetState
    }

    fn min_size(&self) -> Size {
        Size::ZERO
    }
}

impl<E> Widget<E> for NoneWidget {
    type State = NoneWidgetState;

    fn layout(
        &mut self,
        _: &mut NoneWidgetState,
        _: &mut E,
        _: LayoutConstraint,
        _: &mut LayoutCtx,
    ) {
    }
}

pub const PADDING: f64 = 16.0;

pub const COLOR: u32 = 0x00_00_00_FF;
pub const COLOR_HOVER: u32 = 0x33_33_33_FF;
pub const COLOR_DOWN: u32 = 0x44_44_44_FF;

pub const BORDER_WIDTH: f64 = 2.0;

pub const BORDER_COLOR: u32 = 0xFF_FF_FF_FF;

// pub fn measure_rect<W: Widget>(
//     widget: &mut W,
//     state: &mut W::State,
//     max_size: LayoutConstraint,
//     renderer: &mut Piet,
//     theme: &Theme,
// ) -> Size {
//     widget.measure(
//         state,
//         [
//             max_size[0].map(|w| 0f64.max(w - PADDING)),
//             max_size[1].map(|h| 0f64.max(h - PADDING)),
//         ],
//         renderer,
//         theme,
//     ) + Size::new(PADDING * 2., PADDING * 2.)
// }

pub fn render_rect<E, D: Widget<E>>(
    border: bool,
    hover: bool,

    widget: &mut D,
    state: &mut D::State,
    env: &mut E,
    rect: Rect,
    layer: u8,
    focus: bool,
    ctx: &mut RenderCtx,
) {
    if layer == 0 {
        let hover = hover
            && if let Some(point) = ctx.input_state.cursor_pos {
                rect.contains(point)
            } else {
                false
            };

        let brush = &ctx.piet.solid_brush(piet_common::Color::Rgba32(
            match (hover, ctx.input_state.mouse_down) {
                (true, true) => COLOR_DOWN,
                (true, false) => COLOR_HOVER,
                (false, _) => COLOR,
            },
        ));
        ctx.piet.fill(rect, brush);

        if border {
            let rect_pos = (rect.x0 + BORDER_WIDTH / 2.0, rect.y0 + BORDER_WIDTH / 2.0);
            let rect_size = (rect.width() - BORDER_WIDTH, rect.height() - BORDER_WIDTH);
            let rect_shape = kurbo::Rect::from_origin_size(rect_pos, rect_size);

            let brush = ctx
                .piet
                .solid_brush(piet_common::Color::Rgba32(BORDER_COLOR));
            ctx.piet.stroke(rect_shape, &brush, BORDER_WIDTH);
        }
    }

    widget.render(state, env, rect.inset(-PADDING), layer, focus, ctx);
}
