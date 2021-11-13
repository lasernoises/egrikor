use super::*;
use crate::*;

use druid_shell::kurbo::{BezPath, PathEl};
use druid_shell::piet::{Color, Text, TextLayout, TextLayoutBuilder};
use piet_common::Piet;
use piet_common::RenderContext;

pub fn text_elem<C: Debug>(text: &'static str) -> impl Element<C> {
    struct TextElem(&'static str);

    impl<C: Debug> Element<C> for TextElem {
        type Widget = TextWidget;

        fn build(self) -> TextWidget {
            TextWidget {
                variant: WidgetVariant::Normal,
                text: self.0,
                layout: Size::ZERO,
            }
        }

        fn update(self, widget: &mut TextWidget) {
            widget.text = self.0;
        }
    }

    TextElem(text)
}

pub struct TextWidget {
    pub variant: WidgetVariant,
    pub text: &'static str,
    pub layout: Size,
}

impl<C: Debug> Widget<C> for TextWidget {
    type Event = ();

    fn measure(
        &mut self,
        max_size: [Option<f64>; 2],
        renderer: &mut Piet,
        theme: &Theme,
        _: &mut C,
    ) -> Size {
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
            if let Some(max_width) = max_size[0] {
                min_width.max(max_width)
            } else {
                min_width
            },
            if let Some(max_height) = max_size[1] {
                min_height.max(max_height)
            } else {
                min_height
            },
        );

        self.layout
    }

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
        _: &mut C,
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

pub fn checkmark_elem<C: Debug>(size: Size) -> impl Element<C> {
    struct Elem(Size);

    impl<C: Debug> Element<C> for Elem {
        type Widget = Checkmark;

        fn build(self) -> Checkmark {
            Checkmark(self.0)
        }

        fn update(self, widget: &mut Checkmark) {
            widget.0 = self.0;
        }
    }

    Elem(size)
}

pub struct Checkmark(pub Size);

/// Much like the [FixedRect] the [Checkmark] also only has a fixed size, but it also renders a
/// checkmark (as the name suggests).
impl<C: Debug> Widget<C> for Checkmark {
    type Event = ();

    fn measure(
        &mut self,
        _max_size: [Option<f64>; 2],
        _renderer: &mut Piet,
        _theme: &Theme,
        _: &mut C,
    ) -> Size {
        self.0
    }

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
        _: &mut C,
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

pub fn fixed_rect_elem<C: Debug>(size: Size) -> impl Element<C> {
    struct Elem(Size);

    impl<C: Debug> Element<C> for Elem {
        type Widget = FixedRect;

        fn build(self) -> FixedRect {
            FixedRect(self.0)
        }

        fn update(self, widget: &mut FixedRect) {
            widget.0 = self.0;
        }
    }

    Elem(size)
}

pub struct FixedRect(pub Size);

/// Much like the [FixedRect] the [Checkmark] also only has a fixed size, but it also renders a
/// checkmark (as the name suggests).
impl<C: Debug> Widget<C> for FixedRect {
    type Event = ();

    fn measure(
        &mut self,
        _max_size: [Option<f64>; 2],
        _renderer: &mut Piet,
        _theme: &Theme,
        _: &mut C,
    ) -> Size {
        self.0
    }

    fn min_size(&self) -> Size {
        self.0
    }
}
