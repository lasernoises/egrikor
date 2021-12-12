use super::*;
use crate::*;

use druid_shell::kurbo::{BezPath, PathEl};
use druid_shell::piet::{Color, Text, TextLayout, TextLayoutBuilder};
use piet_common::Piet;
use piet_common::RenderContext;

pub fn text_elem(text: &'static str) -> impl WidgetParams<'static> {
    TextElem(text)
}

struct TextElem(&'static str);

impl WidgetParams for TextElem {
    type Widget = TextElem;

}

pub struct TextWidget {
    pub variant: WidgetVariant,
    // pub text: &'static str,
    pub layout: Size,
}

impl Widget for TextWidget {
    type Params = TextElem;
    type Event = ();

    fn build(
        self,
        constraint: LayoutConstraint,
        renderer: &mut Piet,
        theme: &Theme,
    ) -> TextWidget {
        let mut widget = TextWidget {
            variant: WidgetVariant::Normal,
            // text: self.0,
            layout: Size::ZERO,
        };
        self.update(constraint, renderer, theme, &mut widget);
        widget
    }

    fn update(
        &mut self,
        params: &mut Self::Params,
        constraint: LayoutConstraint,
        renderer: &mut Piet,
        theme: &Theme,
    ) {
        self.text = self.0;

        let theme = theme.text.get(self.variant, true);

        let text_factory = renderer.text();
        let font = text_factory.font_family(theme.font).unwrap();
        let layout = text_factory
            .new_text_layout(self.text)
            .font(font, theme.size as f64)
            .build()
            .unwrap();

        let layout_size = layout.size();
        let min_width: f64 = layout_size.width;
        let min_height: f64 = layout_size.height;

        self.layout = Size::new(
            if let Some(max_width) = constraint[0] {
                min_width.max(max_width)
            } else {
                min_width
            },
            if let Some(max_height) = constraint[1] {
                min_height.max(max_height)
            } else {
                min_height
            },
        );
    }

    // fn measure(
    //     &mut self,
    //     max_size: [Option<f64>; 2],
    //     renderer: &mut Piet,
    //     theme: &Theme,
    //     _: &mut C,
    // ) -> Size {
    //     let theme = theme.text.get(self.variant, true);

    //     let text_factory = renderer.text();
    //     let font = text_factory.font_family(theme.font).unwrap();
    //     let layout = text_factory
    //         .new_text_layout(self.text)
    //         .font(font, theme.size as f64)
    //         .build()
    //         .unwrap();

    //     let layout_size = layout.size();
    //     let min_width: f64 = layout_size.width;
    //     let min_height: f64 = layout_size.height;

    //     self.layout = Size::new(
    //         if let Some(max_width) = max_size[0] {
    //             min_width.max(max_width)
    //         } else {
    //             min_width
    //         },
    //         if let Some(max_height) = max_size[1] {
    //             min_height.max(max_height)
    //         } else {
    //             min_height
    //         },
    //     );

    //     self.layout
    // }

    fn min_size(&self) -> Size {
        self.layout
    }

    fn extra_layers(&self) -> u8 {
        0
    }

    fn render(
        &mut self,
        rect: Rect,
        renderer: &mut Piet,
        theme: &Theme,
        _: &InputState,
        _: u8,
        _: bool,
    ) {
        let theme = theme.text.get(self.variant, true);

        let text_factory = renderer.text();
        let font = text_factory.font_family(theme.font).unwrap();
        let layout = text_factory
            .new_text_layout(self.text)
            .font(font, theme.size as f64)
            .text_color(Color::WHITE)
            .build()
            .unwrap();

        let text_width = layout.size().width;

        let line_metric = layout.line_metric(0).unwrap();

        renderer.draw_text(
            &layout,
            (
                rect.x0 + (rect.width() - text_width) / 2.0,
                (rect.y0 + rect.height() / 2.0) - line_metric.height / 2.0,
                // + line_metric.y_offset - line_metric.baseline,
            ),
        );
    }
}

pub fn checkmark_elem(size: Size) -> impl WidgetParams<'static> {
    CheckmarkElem(size)
}

struct CheckmarkElem(Size);

impl WidgetParams for CheckmarkElem {
    type Widget = Checkmark;
}

pub struct Checkmark();

/// Much like the [FixedRect] the [Checkmark] also only has a fixed size, but it also renders a
/// checkmark (as the name suggests).
impl Widget for Checkmark {
    type Params = CheckmarkElem;
    type Event = ();

    fn build(
        params: &mut Self::Params,
        constraint: LayoutConstraint,
        renderer: &mut Piet,
        theme: &Theme,
    ) -> Self {
        Checkmark()
    }

    fn update(
        &mut self,
        params: &mut Self::Params,
        constraint: LayoutConstraint,
        renderer: &mut Piet,
        theme: &Theme,
    ) {}

    // fn measure(
    //     &mut self,
    //     _max_size: [Option<f64>; 2],
    //     _renderer: &mut Piet,
    //     _theme: &Theme,
    //     _: &mut C,
    // ) -> Size {
    //     self.0
    // }

    // This will always be called after measure.
    fn min_size(&self) -> Size {
        self.0
    }

    /// These are the extra layers (there is always one at least from the perspective of a widget).
    /// Layers are relative a widget in layer 1 will not know that there's stuff below.
    /// Containers must return the max of their children here.
    fn extra_layers(&self) -> u8 {
        0
    }

    /// Single-layer widgets can just ignore the `layer` parameter since `render` they should only
    /// be called for layers a widget actually has.
    fn render(
        &mut self,
        rect: Rect,
        renderer: &mut Piet,
        _: &Theme,
        _: &InputState,
        _: u8,
        _: bool,
    ) {
        let rect = Rect::from_center_size(rect.center(), self.0).inset(4.);

        renderer.stroke(
            BezPath::from_vec(vec![
                PathEl::MoveTo((rect.x0, rect.y0 + rect.height() / 2.).into()),
                PathEl::LineTo((rect.x0 + rect.width() / 3., rect.y1 - rect.height() / 6.).into()),
                PathEl::LineTo((rect.x1, rect.y0 + rect.height() / 6.).into()),
            ]),
            &Color::Rgba32(0xFF_FF_FF_FF),
            4.0,
        );
    }
}

pub fn fixed_rect_elem<'a>(size: Size) -> impl WidgetParams {
    FixedRectParams(size)
}

struct FixedRectParams(Size);

impl WidgetParams for FixedRectParams {
    type Widget = FixedRect;
}


pub struct FixedRect();

/// Much like the [FixedRect] the [Checkmark] also only has a fixed size, but it also renders a
/// checkmark (as the name suggests).
impl Widget for FixedRect {
    type Event = ();

    fn build(
        _: &mut Self::Params,
        _: LayoutConstraint,
        _: &mut Piet,
        _: &Theme,
    ) -> FixedRect {
        FixedRect()
    }

    fn update(
        &mut self,
        _: &mut Self::Params,
        _: LayoutConstraint,
        _: &mut Piet,
        _: &Theme,
    ) {
    }

    fn min_size(&self) -> Size {
        self.0
    }
}
