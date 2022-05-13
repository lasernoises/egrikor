//! A textbox widget.

use std::fmt::Debug;
use std::time::Duration;

use crate::text::{
    BasicTextInput, EditAction, Editor, FontDescriptor, LayoutMetrics, Selection,
    TextInput, TextLayout,
};
use piet_common::kurbo::{Affine, Insets, Size};
use piet_common::{Piet, PietText};
// use crate::widget::prelude::*;
// use crate::{
//     theme, Affine, Color, Cursor, Data, FontDescriptor, HotKey, KbKey, KeyOrValue, Point, Selector,
//     SysMods, TextAlignment, TimerToken, Vec2,
// };

use druid_shell::TimerToken;
use piet_common::kurbo::{Point, Vec2};
use piet_common::{Color, RenderContext, TextAlignment};

use crate::*;

const MAC_OR_LINUX: bool = true; //cfg!(any(target_os = "macos", target_os = "linux"));
const CURSOR_BLINK_DURATION: Duration = Duration::from_millis(500);

// const BEGIN_EDITING: Selector = Selector::new("druid.builtin.textbox-begin-editing");
// const COMPLETE_EDITING: Selector = Selector::new("druid.builtin.textbox-complete-editing");
// const CANCEL_EDITING: Selector = Selector::new("druid.builtin.textbox-cancel-editing");

pub fn textbox<'a>(content: &'a mut TextBoxContent) -> impl Widget + 'a {
    TextBoxParams(content)
}

struct TextBoxParams<'a>(&'a mut TextBoxContent);

impl<'a> Widget for TextBoxParams<'a> {
    type State = TextBox;

    fn build(
        &mut self,
        constraint: LayoutConstraint,
        renderer: &mut Piet,
        theme: &Theme,
    ) -> Self::State {
        let mut state = TextBox {
            min_size: Size::ZERO,
            // content: self.0,
        };
        self.update(&mut state, constraint, renderer, theme);
        state
    }

    fn update(
        &mut self,
        state: &mut Self::State,
        constraint: LayoutConstraint,
        renderer: &mut Piet,
        _theme: &Theme,
    ) {
        // self.editor.rebuild_if_needed(renderer.text());

        let width = 100.;
        let text_insets = Insets::new(4.0, 2.0, 4.0, 2.0);

        self.0.placeholder.rebuild_if_needed(renderer.text());
        if let Some(width) = constraint[0] {
            if self.0.multiline {
                self.0.editor.set_wrap_width(width - text_insets.x_value());
            }
        }
        self.0.editor.rebuild_if_needed(renderer.text());

        // for placeholder text when empty we don't need that shit right now
        // let text_metrics = if data.is_empty() {
        //     self.placeholder.layout_metrics()
        // } else {
        //     self.editor.layout().layout_metrics()
        // };

        let text_metrics = self.0.editor.layout().layout_metrics();

        let height = text_metrics.size.height + text_insets.y_value();
        // if we have a non-left text-alignment, we need to manually adjust our position.
        self.0.update_alignment_adjustment(
            width - text_insets.x_value(),
            &text_metrics,
        );
        self.0.text_pos = Point::new(text_insets.x0 + self.0.alignment_offset, text_insets.y0);

        // Not needed for now.
        // let bottom_padding = (height - text_metrics.size.height) / 2.0;
        // let baseline_off =
        //     bottom_padding + (text_metrics.size.height - text_metrics.first_baseline);
        // renderer.set_baseline_offset(baseline_off);

        state.min_size = Size::new(width, height);
    }

    // This will always be called after measure.
    fn min_size(&self, state: &Self::State) -> Size {
        state.min_size
    }

    /// Single-layer widgets can just ignore the `layer` parameter since `render` they should only
    /// be called for layers a widget actually has.
    fn render(
        &mut self,
        state: &mut Self::State,
        rect: Rect,
        renderer: &mut Piet,
        _theme: &Theme,
        _input_state: &InputState,
        _layer: u8,
        focus: bool,
    ) {
        // let size = renderer.size();
        let background_color = Color::rgb8(0x3a, 0x3a, 0x3a);
        let selection_color = Color::rgb8(0xf3, 0x00, 0x21);
        let cursor_color = Color::WHITE;
        let border_width = 1.;
        let text_insets = Insets::new(4.0, 2.0, 4.0, 2.0);

        let border_color = if focus {
            Color::rgb8(0x00, 0x8d, 0xdd)
        } else {
            Color::rgb8(0x3a, 0x3a, 0x3a)
        };

        // Paint the background
        let clip_rect = Size::new(rect.width() - border_width, rect.height())
            .to_rect()
            .inset(-border_width / 2.0)
            .to_rounded_rect(2.);

        // let clip_rect = rect
        //     .inset((border_width, 0.))
        //     .inset(-border_width / 2.)
        //     .to_rounded_rect(2.);

        renderer
            .with_save(|rc| {
                rc.transform(Affine::translate((rect.x0, rect.y0)));

                rc.fill(clip_rect, &background_color);

                // Render text, selection, and cursor inside a clip
                rc.with_save(|rc| {
                    rc.clip(clip_rect);

                    // Shift everything inside the clip by the hscroll_offset
                    rc.transform(Affine::translate((-self.0.hscroll_offset, 0.)));

                    let text_pos = self.0.text_position();
                    // Draw selection rect
                    if focus {
                        for sel in self.0.editor.selection_rects() {
                            let sel = sel + text_pos.to_vec2();
                            let rounded = sel.to_rounded_rect(1.0);
                            rc.fill(rounded, &selection_color);
                        }
                    }
                    self.0.editor.draw(rc, text_pos);
                    // if !data.is_empty() {
                    //     if is_focused {
                    //         for sel in self.editor.selection_rects() {
                    //             let sel = sel + text_pos.to_vec2();
                    //             let rounded = sel.to_rounded_rect(1.0);
                    //             rc.fill(rounded, &selection_color);
                    //         }
                    //     }
                    //     self.editor.draw(rc, text_pos);
                    // } else {
                    //     self.placeholder.draw(rc, text_pos);
                    // }

                    // Paint the cursor if focused and there's no selection
                    if focus
                    /* && self.should_draw_cursor() */
                    {
                        // if there's no data, we always draw the cursor based on
                        // our alignment.
                        // let cursor = if data.is_empty() {
                        //     let dx = match self.alignment {
                        //         TextAlignment::Start | TextAlignment::Justified => text_insets.x0,
                        //         TextAlignment::Center => size.width / 2.0,
                        //         TextAlignment::End => size.width - text_insets.x1,
                        //     };
                        //     self.editor.cursor_line() + Vec2::new(dx, text_insets.y0)
                        // } else {
                        let cursor = {
                            // the cursor position can extend past the edge of the layout
                            // (commonly when there is trailing whitespace) so we clamp it
                            // to the right edge.
                            let mut cursor = self.0.editor.cursor_line() + text_pos.to_vec2();
                            let dx = rect.width() + self.0.hscroll_offset
                                - text_insets.x0
                                - cursor.p0.x;
                            if dx < 0.0 {
                                cursor = cursor + Vec2::new(dx, 0.);
                            }
                            cursor
                        };

                        rc.stroke(cursor, &cursor_color, 1.);
                    }

                    Ok(())
                })
                .unwrap();

                // Paint the border
                rc.stroke(clip_rect, &border_color, border_width);

                Ok(())
            })
            .unwrap();
    }

    fn handle_cursor_input(
        &mut self,
        state: &mut Self::State,
        rect: Rect,
        cursor_pos: Point,
        _cursor_layer: u8,
        input: CursorInput,
        input_state: &InputState,
        _theme: &Theme,
        _focus: bool,
    ) -> InputReturn {
        self.0.suppress_adjust_hscroll = false;

        match input {
            CursorInput::Down(_button) => {
                // ctx.request_focus();
                // ctx.set_active(true);
                // let mut mouse = mouse.clone();
                // let rect_theme = theme.rect.get(WidgetVariant::Normal, true);

                if rect.contains(cursor_pos) {
                    let cursor_pos = [
                        cursor_pos.x - rect.x0 + self.0.hscroll_offset - self.0.alignment_offset,
                        cursor_pos.x - rect.y0,
                    ];

                    // mouse_pos += Vec2::new(self.hscroll_offset - self.alignment_offset, 0.0);

                    // if !mouse.focus {
                    // }
                    self.0.was_focused_from_click = true;
                    // self.reset_cursor_blink(ctx.request_timer(CURSOR_BLINK_DURATION));
                    self.0.editor.click(cursor_pos, input_state.mods);

                    return InputReturn { demand_focus: true };
                }

                // ctx.request_paint();
            }
            CursorInput::Move => {
                let cursor_pos = [
                    cursor_pos.x - rect.x0 + self.0.hscroll_offset - self.0.alignment_offset,
                    cursor_pos.y - rect.y0,
                ];

                // ctx.set_cursor(&Cursor::IBeam);
                if input_state.mouse_down
                    && cursor_pos[0] >= 0.
                    && cursor_pos[1] >= 0.
                    && cursor_pos[0] <= rect.width()
                    && cursor_pos[1] <= rect.height()
                {
                    self.0.editor.drag(cursor_pos, input_state.mods);
                }
                // if ctx.is_active() {
                //     // ctx.request_paint();
                // }
            }
            _ => (),
        }

        Default::default()
    }

    fn handle_keyboard_input(
        &mut self,
        state: &mut Self::State,
        _rect: Rect,
        input: &KeyboardInput,
        _input_state: &InputState,
        _theme: &Theme,
        _focus: bool,
    ) {
        self.0.suppress_adjust_hscroll = false;

        match input {
            KeyboardInput::KeyDown(key_event) => {
                match key_event {
                    // Tab and shift+tab
                    // k_e if HotKey::new(None, KbKey::Tab).matches(k_e) => ctx.focus_next(),
                    // k_e if HotKey::new(SysMods::Shift, KbKey::Tab).matches(k_e) => ctx.focus_prev(),
                    k_e => {
                        if let Some(edit) = BasicTextInput.handle_event(k_e) {
                            self.0.suppress_adjust_hscroll = matches!(edit, EditAction::SelectAll);
                            self.0.editor.do_edit(edit);
                            self.0.editor.update();
                            // an explicit request update in case the selection
                            // state has changed, but the data hasn't.
                            // ctx.request_update();
                            // ctx.request_paint();
                        }
                    }
                };
                // self.reset_cursor_blink(ctx.request_timer(CURSOR_BLINK_DURATION));
                // ctx.request_paint();
            }
        }
    }

    // fn handle_input(
    //     &mut self,
    //     rect: Rect,
    //     input: &Input,
    //     input_state: &InputState,
    //     theme: &Theme,
    //     _: bool,
    // ) -> InputReturn {
    //     self.suppress_adjust_hscroll = false;
    //     match input {
    //         &Input::MouseDown { pos: mouse_pos, button } => {
    //             // ctx.request_focus();
    //             // ctx.set_active(true);
    //             // let mut mouse = mouse.clone();
    //             let rect_theme = theme.rect.get(WidgetVariant::Normal, true);

    //             if rect.contains(mouse_pos) {
    //                 let mouse_pos = [
    //                     mouse_pos.x - rect.x0
    //                         + self.hscroll_offset - self.alignment_offset,
    //                     mouse_pos.x - rect.y0,
    //                 ];

    //                 // mouse_pos += Vec2::new(self.hscroll_offset - self.alignment_offset, 0.0);

    //                 // if !mouse.focus {
    //                 // }
    //                 self.was_focused_from_click = true;
    //                 // self.reset_cursor_blink(ctx.request_timer(CURSOR_BLINK_DURATION));
    //                 self.editor.click(mouse_pos, input_state.mods);

    //                 return InputReturn { demand_focus: true };
    //             }

    //             // ctx.request_paint();
    //         }
    //         Input::MouseMove { pos: mouse_pos } => {
    //             let mouse_pos = [
    //                 mouse_pos.x - rect.x0
    //                     + self.hscroll_offset - self.alignment_offset,
    //                 mouse_pos.y - rect.y0,
    //             ];

    //             // ctx.set_cursor(&Cursor::IBeam);
    //             if
    //                 input_state.mouse_down
    //                     && mouse_pos[0] >= 0.
    //                     && mouse_pos[1] >= 0.
    //                     && mouse_pos[0] <= rect.width()
    //                     && mouse_pos[1] <= rect.height()
    //             {
    //                 self.editor.drag(mouse_pos, input_state.mods);
    //             }
    //             // if ctx.is_active() {
    //             //     // ctx.request_paint();
    //             // }
    //         }
    //         // Input::MouseUp { pos, button } => {
    //         //     if ctx.is_active() {
    //         //         ctx.set_active(false);
    //         //         // ctx.request_paint();
    //         //     }
    //         // }

    //         // Event::Timer(id) => {
    //         //     if *id == self.cursor_timer {
    //         //         self.cursor_on = !self.cursor_on;
    //         //         ctx.request_paint();
    //         //         self.cursor_timer = ctx.request_timer(CURSOR_BLINK_DURATION);
    //         //     }
    //         // }
    //         // Event::Command(ref cmd) if ctx.is_focused() && cmd.is(crate::commands::COPY) => {
    //         //     self.editor.copy(data);
    //         //     ctx.set_handled();
    //         // }
    //         // Event::Command(ref cmd) if ctx.is_focused() && cmd.is(crate::commands::CUT) => {
    //         //     self.editor.cut(data);
    //         //     ctx.set_handled();
    //         // }
    //         // Event::Command(cmd) if cmd.is(TextBox::PERFORM_EDIT) => {
    //         //     let edit = cmd.get_unchecked(TextBox::PERFORM_EDIT);
    //         //     self.editor.do_edit(edit.to_owned(), data);
    //         // }
    //         // Event::Paste(ref item) => {
    //         //     if let Some(string) = item.get_string() {
    //         //         self.editor.paste(string, data);
    //         //     }
    //         // }
    //         Input::KeyDown(key_event) => {
    //             match key_event {
    //                 // Tab and shift+tab
    //                 // k_e if HotKey::new(None, KbKey::Tab).matches(k_e) => ctx.focus_next(),
    //                 // k_e if HotKey::new(SysMods::Shift, KbKey::Tab).matches(k_e) => ctx.focus_prev(),
    //                 k_e => {
    //                     if let Some(edit) = BasicTextInput.handle_event(k_e) {
    //                         self.suppress_adjust_hscroll = matches!(edit, EditAction::SelectAll);
    //                         self.editor.do_edit(edit);
    //                         self.editor.update();
    //                         // an explicit request update in case the selection
    //                         // state has changed, but the data hasn't.
    //                         // ctx.request_update();
    //                         // ctx.request_paint();
    //                     }
    //                 }
    //             };
    //             // self.reset_cursor_blink(ctx.request_timer(CURSOR_BLINK_DURATION));
    //             // ctx.request_paint();
    //         }
    //         _ => (),
    //     }

    //     Default::default()
    // }
}

#[derive(Debug, Clone)]
pub struct TextBoxContent {
    placeholder: TextLayout<String>,
    editor: Editor<String>,
    // this can be Box<dyn TextInput> in the future
    hscroll_offset: f64,
    // in cases like SelectAll, we don't adjust the viewport after an event.
    suppress_adjust_hscroll: bool,
    cursor_timer: TimerToken,
    cursor_on: bool,
    multiline: bool,
    alignment: TextAlignment,
    alignment_offset: f64,
    text_pos: Point,
    /// true if a click event caused us to gain focus.
    ///
    /// On macOS, if focus happens via click then we set the selection based
    /// on the click position; if focus happens automatically (e.g. on tab)
    /// then we select our entire contents.
    was_focused_from_click: bool,
}

/// A widget that allows user text input.
///
/// # Editing values
///
/// If the text you are editing represents a value of some other type, such
/// as a number, you should use a [`ValueTextBox`] and an appropriate
/// [`Formatter`]. You can create a [`ValueTextBox`] by passing the appropriate
/// [`Formatter`] to [`TextBox::with_formatter`].
#[derive(Debug)]
pub struct TextBox {
    // placeholder: TextLayout<String>,
    // editor: Editor<T>,
    // // this can be Box<dyn TextInput> in the future
    // hscroll_offset: f64,
    // // in cases like SelectAll, we don't adjust the viewport after an event.
    // suppress_adjust_hscroll: bool,
    // cursor_timer: TimerToken,
    // cursor_on: bool,
    // multiline: bool,
    // alignment: TextAlignment,
    // alignment_offset: f64,
    // text_pos: Point,
    // /// true if a click event caused us to gain focus.
    // ///
    // /// On macOS, if focus happens via click then we set the selection based
    // /// on the click position; if focus happens automatically (e.g. on tab)
    // /// then we select our entire contents.
    // was_focused_from_click: bool,
    min_size: Size,
    // content: &'a mut TextBoxContent,
}

impl TextBoxContent {
    /// Create a new TextBox widget.
    pub fn new() -> Self {
        let mut placeholder = TextLayout::from_text("");
        placeholder.set_text_color(Color::rgb8(0x80, 0x80, 0x80));
        Self {
            editor: Editor::new(),
            hscroll_offset: 0.,
            suppress_adjust_hscroll: false,
            cursor_timer: TimerToken::INVALID,
            cursor_on: false,
            placeholder,
            multiline: false,
            alignment: TextAlignment::Start,
            alignment_offset: 0.0,
            text_pos: Point::ZERO,
            was_focused_from_click: false,
            // min_size: Size::ZERO,
        }
    }

    /// Create a new multi-line `TextBox`.
    pub fn multiline() -> Self {
        let mut this = TextBoxContent::new();
        this.editor.set_multiline(true);
        this.multiline = true;
        this
    }

    /// Builder-style method to set the `TextBox`'s placeholder text.
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder.set_text(placeholder.into());
        self
    }

    /// Builder-style method for setting the text size.
    ///
    /// The argument can be either an `f64` or a [`Key<f64>`].
    ///
    /// [`Key<f64>`]: ../struct.Key.html
    pub fn with_text_size(mut self, size: impl Into<f64>) -> Self {
        self.set_text_size(size);
        self
    }

    /// Builder-style method to set the [`TextAlignment`].
    ///
    /// This is only relevant when the `TextBox` is *not* [`multiline`],
    /// in which case it determines how the text is positioned inside the
    /// `TextBox` when it does not fill the available space.
    ///
    /// # Note:
    ///
    /// This does not behave exactly like [`TextAlignment`] does when used
    /// with label; in particular this does not account for reading direction.
    /// This means that `TextAlignment::Start` (the default) always means
    /// *left aligned*, and `TextAlignment::End` always means *right aligned*.
    ///
    /// This should be considered a bug, but it will not be fixed until proper
    /// BiDi support is implemented.
    ///
    /// [`TextAlignment`]: enum.TextAlignment.html
    /// [`multiline`]: #method.multiline
    pub fn with_text_alignment(mut self, alignment: TextAlignment) -> Self {
        self.set_text_alignment(alignment);
        self
    }

    /// Builder-style method for setting the font.
    ///
    /// The argument can be a [`FontDescriptor`] or a [`Key<FontDescriptor>`]
    /// that refers to a font defined in the [`Env`].
    ///
    /// [`Env`]: ../struct.Env.html
    /// [`FontDescriptor`]: ../struct.FontDescriptor.html
    /// [`Key<FontDescriptor>`]: ../struct.Key.html
    pub fn with_font(mut self, font: impl Into<FontDescriptor>) -> Self {
        self.set_font(font);
        self
    }

    /// Builder-style method for setting the text color.
    ///
    /// The argument can be either a `Color` or a [`Key<Color>`].
    ///
    /// [`Key<Color>`]: ../struct.Key.html
    pub fn with_text_color(mut self, color: impl Into<Color>) -> Self {
        self.set_text_color(color);
        self
    }

    /// Set the `TextBox`'s placeholder text.
    pub fn set_placeholder(&mut self, placeholder: impl Into<String>) {
        self.placeholder.set_text(placeholder.into());
    }

    /// Set the text size.
    ///
    /// The argument can be either an `f64` or a [`Key<f64>`].
    ///
    /// [`Key<f64>`]: ../struct.Key.html
    pub fn set_text_size(&mut self, size: impl Into<f64>) {
        let size = size.into();
        self.editor.layout_mut().set_text_size(size.clone());
        self.placeholder.set_text_size(size);
    }

    /// Set the font.
    ///
    /// The argument can be a [`FontDescriptor`] or a [`Key<FontDescriptor>`]
    /// that refers to a font defined in the [`Env`].
    ///
    /// [`Env`]: ../struct.Env.html
    /// [`FontDescriptor`]: ../struct.FontDescriptor.html
    /// [`Key<FontDescriptor>`]: ../struct.Key.html
    pub fn set_font(&mut self, font: impl Into<FontDescriptor>) {
        let font = font.into();
        self.editor.layout_mut().set_font(font.clone());
        self.placeholder.set_font(font);
    }

    /// Set the [`TextAlignment`] for this `TextBox``.
    ///
    /// This is only relevant when the `TextBox` is *not* [`multiline`],
    /// in which case it determines how the text is positioned inside the
    /// `TextBox` when it does not fill the available space.
    ///
    /// # Note:
    ///
    /// This does not behave exactly like [`TextAlignment`] does when used
    /// with label; in particular this does not account for reading direction.
    /// This means that `TextAlignment::Start` (the default) always means
    /// *left aligned*, and `TextAlignment::End` always means *right aligned*.
    ///
    /// This should be considered a bug, but it will not be fixed until proper
    /// BiDi support is implemented.
    ///
    /// [`TextAlignment`]: enum.TextAlignment.html
    /// [`multiline`]: #method.multiline
    pub fn set_text_alignment(&mut self, alignment: TextAlignment) {
        self.alignment = alignment;
    }

    /// Set the text color.
    ///
    /// The argument can be either a `Color` or a [`Key<Color>`].
    ///
    /// If you change this property, you are responsible for calling
    /// [`request_layout`] to ensure the label is updated.
    ///
    /// [`request_layout`]: ../struct.EventCtx.html#method.request_layout
    /// [`Key<Color>`]: ../struct.Key.html
    pub fn set_text_color(&mut self, color: impl Into<Color>) {
        self.editor.layout_mut().set_text_color(color);
    }

    /// Return the [`Editor`] used by this `TextBox`.
    ///
    /// This is only needed in advanced cases, such as if you want to customize
    /// the drawing of the text.
    pub fn editor(&self) -> &Editor<String> {
        &self.editor
    }

    /// The point, relative to the origin, where this text box draws its
    /// [`TextLayout`].
    ///
    /// This is exposed in case the user wants to do additional drawing based
    /// on properties of the text.
    ///
    /// This is not valid until `layout` has been called.
    pub fn text_position(&self) -> Point {
        self.text_pos
    }

    pub fn text(&self) -> &str {
        &self.editor().layout().text
    }

    pub fn set_text(&mut self, text: String) {
        self.editor.set_text(text);
    }

    /// Set the textbox's selection.
    pub fn set_selection(&mut self, selection: Selection) {
        self.editor.set_selection(selection);
    }

    /// Set the text and force the editor to update.
    ///
    /// This should be rarely needed; the main use-case would be if you need
    /// to manually set the text and then immediately do hit-testing or other
    /// tasks that rely on having an up-to-date text layout.
    pub fn force_rebuild(&mut self, text: String, factory: &mut PietText) {
        self.editor.set_text(text);
        self.editor.rebuild_if_needed(factory);
    }

    /// Calculate a stateful scroll offset
    fn update_hscroll(&mut self, self_width: f64) {
        let cursor_x = self.editor.cursor_line().p0.x;
        // if the text ends in trailing whitespace, that space is not included
        // in its reported width, but we need to include it for these calculations.
        // see https://github.com/linebender/druid/issues/1430
        let overall_text_width = self.editor.layout().size().width.max(cursor_x);
        let text_insets = Insets::new(4.0, 2.0, 4.0, 2.0);

        //// when advancing the cursor, we want some additional padding
        if overall_text_width < self_width - text_insets.x_value() {
            // There's no offset if text is smaller than text box
            //
            // [***I*  ]
            // ^
            self.hscroll_offset = 0.;
        } else if cursor_x > self_width - text_insets.x_value() + self.hscroll_offset {
            // If cursor goes past right side, bump the offset
            //       ->
            // **[****I]****
            //   ^
            self.hscroll_offset = cursor_x - self_width + text_insets.x_value();
        } else if cursor_x < self.hscroll_offset {
            // If cursor goes past left side, match the offset
            //    <-
            // **[I****]****
            //   ^
            self.hscroll_offset = cursor_x;
        } else if self.hscroll_offset > overall_text_width - self_width + text_insets.x_value() {
            // If the text is getting shorter, keep as small offset as possible
            //        <-
            // **[****I]
            //   ^
            self.hscroll_offset = overall_text_width - self_width + text_insets.x_value();
        }
    }

    fn reset_cursor_blink(&mut self, token: TimerToken) {
        self.cursor_on = true;
        self.cursor_timer = token;
    }

    // on macos we only draw the cursor if the selection is non-caret
    #[cfg(target_os = "macos")]
    fn should_draw_cursor(&self) -> bool {
        self.cursor_on && self.editor.selection().is_caret()
    }

    #[cfg(not(target_os = "macos"))]
    fn should_draw_cursor(&self) -> bool {
        self.cursor_on
    }

    fn update_alignment_adjustment(&mut self, available_width: f64, metrics: &LayoutMetrics) {
        self.alignment_offset = if self.multiline {
            0.0
        } else {
            let extra_space = (available_width - metrics.size.width).max(0.0);
            match self.alignment {
                TextAlignment::Start | TextAlignment::Justified => 0.0,
                TextAlignment::End => extra_space,
                TextAlignment::Center => extra_space / 2.0,
            }
        }
    }
}

/*
impl<T: TextStorage + EditableText> Widget<T> for TextBox<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, _env: &Env) {
        self.suppress_adjust_hscroll = false;
        match event {
            Event::MouseDown(mouse) => {
                ctx.request_focus();
                ctx.set_active(true);
                let mut mouse = mouse.clone();
                mouse.pos += Vec2::new(self.hscroll_offset - self.alignment_offset, 0.0);

                if !mouse.focus {
                    self.was_focused_from_click = true;
                    self.reset_cursor_blink(ctx.request_timer(CURSOR_BLINK_DURATION));
                    self.editor.click(&mouse, data);
                }

                ctx.request_paint();
            }
            Event::MouseMove(mouse) => {
                let mut mouse = mouse.clone();
                mouse.pos += Vec2::new(self.hscroll_offset - self.alignment_offset, 0.0);
                ctx.set_cursor(&Cursor::IBeam);
                if ctx.is_active() {
                    self.editor.drag(&mouse, data);
                    ctx.request_paint();
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    ctx.request_paint();
                }
            }
            Event::Timer(id) => {
                if *id == self.cursor_timer {
                    self.cursor_on = !self.cursor_on;
                    ctx.request_paint();
                    self.cursor_timer = ctx.request_timer(CURSOR_BLINK_DURATION);
                }
            }
            Event::Command(ref cmd) if ctx.is_focused() && cmd.is(crate::commands::COPY) => {
                self.editor.copy(data);
                ctx.set_handled();
            }
            Event::Command(ref cmd) if ctx.is_focused() && cmd.is(crate::commands::CUT) => {
                self.editor.cut(data);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(TextBox::PERFORM_EDIT) => {
                let edit = cmd.get_unchecked(TextBox::PERFORM_EDIT);
                self.editor.do_edit(edit.to_owned(), data);
            }
            Event::Paste(ref item) => {
                if let Some(string) = item.get_string() {
                    self.editor.paste(string, data);
                }
            }
            Event::KeyDown(key_event) => {
                match key_event {
                    // Tab and shift+tab
                    k_e if HotKey::new(None, KbKey::Tab).matches(k_e) => ctx.focus_next(),
                    k_e if HotKey::new(SysMods::Shift, KbKey::Tab).matches(k_e) => ctx.focus_prev(),
                    k_e => {
                        if let Some(edit) = self.input_handler.handle_event(k_e) {
                            self.suppress_adjust_hscroll = matches!(edit, EditAction::SelectAll);
                            self.editor.do_edit(edit, data);
                            // an explicit request update in case the selection
                            // state has changed, but the data hasn't.
                            ctx.request_update();
                            ctx.request_paint();
                        }
                    }
                };
                self.reset_cursor_blink(ctx.request_timer(CURSOR_BLINK_DURATION));
                ctx.request_paint();
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                ctx.register_for_focus();
                self.editor.set_text(data.to_owned());
                self.editor.rebuild_if_needed(ctx.text(), env);
            }
            LifeCycle::FocusChanged(is_focused) => {
                if MAC_OR_LINUX && *is_focused && !self.was_focused_from_click {
                    self.editor.select_all(data);
                }
                self.was_focused_from_click = false;
                self.reset_cursor_blink(ctx.request_timer(CURSOR_BLINK_DURATION));
                ctx.request_paint();
            }
            _ => (),
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &T, data: &T, env: &Env) {
        self.editor.update(ctx, data, env);
        if !self.suppress_adjust_hscroll && !self.multiline {
            self.update_hscroll(ctx.size().width, env);
        }
        if ctx.env_changed() && self.placeholder.needs_rebuild_after_update(ctx) {
            ctx.request_layout();
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let width = env.get(theme::WIDE_WIDGET_WIDTH);
        let text_insets = env.get(theme::TEXTBOX_INSETS);

        self.placeholder.rebuild_if_needed(ctx.text(), env);
        if self.multiline {
            self.editor
                .set_wrap_width(bc.max().width - text_insets.x_value());
        }
        self.editor.rebuild_if_needed(ctx.text(), env);

        let text_metrics = if data.is_empty() {
            self.placeholder.layout_metrics()
        } else {
            self.editor.layout().layout_metrics()
        };

        let height = text_metrics.size.height + text_insets.y_value();
        let size = bc.constrain((width, height));
        // if we have a non-left text-alignment, we need to manually adjust our position.
        self.update_alignment_adjustment(size.width - text_insets.x_value(), &text_metrics);
        self.text_pos = Point::new(text_insets.x0 + self.alignment_offset, text_insets.y0);

        let bottom_padding = (size.height - text_metrics.size.height) / 2.0;
        let baseline_off =
            bottom_padding + (text_metrics.size.height - text_metrics.first_baseline);
        ctx.set_baseline_offset(baseline_off);

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = ctx.size();
        let background_color = env.get(theme::BACKGROUND_LIGHT);
        let selection_color = env.get(theme::SELECTION_COLOR);
        let cursor_color = env.get(theme::CURSOR_COLOR);
        let border_width = env.get(theme::TEXTBOX_BORDER_WIDTH);
        let text_insets = env.get(theme::TEXTBOX_INSETS);

        let is_focused = ctx.is_focused();

        let border_color = if is_focused {
            env.get(theme::PRIMARY_LIGHT)
        } else {
            env.get(theme::BORDER_DARK)
        };

        // Paint the background
        let clip_rect = Size::new(size.width - border_width, size.height)
            .to_rect()
            .inset(-border_width / 2.0)
            .to_rounded_rect(env.get(theme::TEXTBOX_BORDER_RADIUS));

        ctx.fill(clip_rect, &background_color);

        // Render text, selection, and cursor inside a clip
        ctx.with_save(|rc| {
            rc.clip(clip_rect);

            // Shift everything inside the clip by the hscroll_offset
            rc.transform(Affine::translate((-self.hscroll_offset, 0.)));

            let text_pos = self.text_position();
            // Draw selection rect
            if !data.is_empty() {
                if is_focused {
                    for sel in self.editor.selection_rects() {
                        let sel = sel + text_pos.to_vec2();
                        let rounded = sel.to_rounded_rect(1.0);
                        rc.fill(rounded, &selection_color);
                    }
                }
                self.editor.draw(rc, text_pos);
            } else {
                self.placeholder.draw(rc, text_pos);
            }

            // Paint the cursor if focused and there's no selection
            if is_focused && self.should_draw_cursor() {
                // if there's no data, we always draw the cursor based on
                // our alignment.
                let cursor = if data.is_empty() {
                    let dx = match self.alignment {
                        TextAlignment::Start | TextAlignment::Justified => text_insets.x0,
                        TextAlignment::Center => size.width / 2.0,
                        TextAlignment::End => size.width - text_insets.x1,
                    };
                    self.editor.cursor_line() + Vec2::new(dx, text_insets.y0)
                } else {
                    // the cursor position can extend past the edge of the layout
                    // (commonly when there is trailing whitespace) so we clamp it
                    // to the right edge.
                    let mut cursor = self.editor.cursor_line() + text_pos.to_vec2();
                    let dx = size.width + self.hscroll_offset - text_insets.x0 - cursor.p0.x;
                    if dx < 0.0 {
                        cursor = cursor + Vec2::new(dx, 0.);
                    }
                    cursor
                };
                rc.stroke(cursor, &cursor_color, 1.);
            }
        });

        // Paint the border
        ctx.stroke(clip_rect, &border_color, border_width);
    }
}
*/
