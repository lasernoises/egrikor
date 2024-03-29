// Copyright 2020 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A component for building text editing widgets

use super::{
    movement, offset_for_delete_backwards, EditAction, EditableText, MouseAction, Movement,
    Selection, TextLayout, TextStorage,
};
use druid_shell::{Application, Modifiers};
use piet_common::PietText;
use piet_common::{
    kurbo::{Line, Point, Rect},
    Piet,
};

/// A component for widgets that offer text editing.
///
/// `Editor` manages an [`EditableText`] type, applying edits and maintaining
/// selection state.
///
/// The type parameter `T` is the text data being edited.
///
/// [`EditableText`]: trait.EditableText.html
#[derive(Debug, Clone)]
pub struct Editor<T> {
    layout: TextLayout<T>,
    selection: Selection,
    multiline: bool,
    fixed_width: f64,
}

impl<T: From<&'static str>> Editor<T> {
    /// Create a new `Editor`.
    pub fn new() -> Self {
        Editor {
            layout: Default::default(),
            selection: Selection::caret(0),
            multiline: false,
            fixed_width: f64::INFINITY,
        }
    }

    /// Set whether the editor supports multi-line text. Default to false.
    ///
    /// If this is false, inserted text will only insert up to the first
    /// included newline.
    pub fn set_multiline(&mut self, multiline: bool) {
        self.multiline = multiline;
    }

    /// Set an explicit wrap width for this editor.
    ///
    /// By default the editor will not wrap lines; this is suitable for
    /// cases such as a a single-line text field, where the containing
    /// widget will scroll the editor as required.
    pub fn set_wrap_width(&mut self, width: f64) {
        self.layout.set_wrap_width(width);
    }

    /// Return a reference to the inner [`TextLayout`] object.
    ///
    /// [`TextLayout`]: TextLayout
    pub fn layout(&self) -> &TextLayout<T> {
        &self.layout
    }

    /// Return a mutable reference to the inner [`TextLayout`] object.
    ///
    /// [`TextLayout`]: TextLayout
    pub fn layout_mut(&mut self) -> &mut TextLayout<T> {
        &mut self.layout
    }
}

impl<T: TextStorage + EditableText> Editor<T> {
    /// Set the text for this editor.
    ///
    /// This must be set before the editor is used, such as in [`WidgetAdded`].
    ///
    /// [`WidgetAdded`]: ../enum.LifeCycle.html#variant.WidgetAdded
    pub fn set_text(&mut self, text: T) {
        self.selection = self.selection.constrained(&text);
        self.layout.set_text(text);
    }

    /// Return the current selection.
    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    /// Set the current selection.
    ///
    /// The selection will be constrained to the current text.
    pub fn set_selection(&mut self, selection: Selection) {
        let selection = selection.constrained(&self.layout.text);
        self.selection = selection;
    }

    /// Returns the `Rect`s representing the  current selection.
    pub fn selection_rects(&self) -> Vec<Rect> {
        self.layout.rects_for_range(self.selection.range())
    }

    /// Returns the `Line` to draw for the current cursor position.
    pub fn cursor_line(&self) -> Line {
        self.layout
            .cursor_line_for_text_position(self.selection.end)
    }

    /// Handle a mouse click
    pub fn click(&mut self, pos: [f64; 2], mods: Modifiers) {
        self.do_edit(EditAction::Click(self.mouse_action_for_event(pos, mods)));
    }

    /// Handle a mouse drag
    pub fn drag(&mut self, pos: [f64; 2], mods: Modifiers) {
        self.do_edit(EditAction::Drag(self.mouse_action_for_event(pos, mods)));
    }

    /// Handle a copy command
    pub fn copy(&self, _data: &mut T) {
        self.set_clipboard()
    }

    /// Handle a cut command
    pub fn cut(&mut self, _data: &mut T) {
        self.set_clipboard();
        self.delete_backward();
    }

    /// Handle a paste command
    pub fn paste(&mut self, t: String) {
        self.do_edit(EditAction::Paste(t))
    }

    /// Set the selection to the entire buffer.
    pub fn select_all(&mut self) {
        self.selection = Selection::new(0, self.layout.text.len());
    }

    fn mouse_action_for_event(&self, pos: [f64; 2], mods: Modifiers) -> MouseAction {
        let pos = self
            .layout
            .text_position_for_point(Point::from((pos[0], pos[1])));
        MouseAction {
            row: 0,
            column: pos,
            mods: mods,
        }
    }

    /// Update the editor if the data or env has changed.
    ///
    /// The widget owning this `Editor` must call this method during its own
    /// [`update`] call.
    ///
    /// [`update`]: ../trait.Widget.html#tymethod.update
    pub fn update(&mut self) {
        // if self.data_is_stale(new_data) {
        //     self.layout.set_text(new_data.clone());
        //     self.selection = self.selection.constrained(new_data);
        //     ctx.request_layout();
        // } else if self.layout.needs_rebuild_after_update(ctx) {
        //     ctx.request_layout();
        // }
        self.selection = self.selection.constrained(&self.layout.text);
        self.layout.reset_layout();
        // self.rebuild_if_needed(ctx.text());
    }

    /// Must be called in WidgetAdded
    pub fn rebuild_if_needed(&mut self, factory: &mut PietText) {
        self.layout.rebuild_if_needed(factory);
    }

    /// Perform an [`EditAction`](enum.EditAction.html).
    pub fn do_edit(&mut self, edit: EditAction) {
        // if self.data_is_stale(data) {
        //     // log::warn!("editor data changed externally, skipping event {:?}", &edit);
        //     return;
        // }
        match edit {
            EditAction::Insert(chars) | EditAction::Paste(chars) => self.insert(&chars),
            EditAction::Backspace => self.delete_backward(),
            EditAction::Delete => self.delete_forward(),
            EditAction::JumpDelete(mvmt) | EditAction::JumpBackspace(mvmt) => {
                let to_delete = if self.selection.is_caret() {
                    movement(mvmt, self.selection, &self.layout, true)
                } else {
                    self.selection
                };
                self.layout.text.edit(to_delete.range(), "");
                self.selection = Selection::caret(to_delete.min());
            }
            EditAction::Move(mvmt) => {
                self.selection = movement(mvmt, self.selection, &self.layout, false)
            }
            EditAction::ModifySelection(mvmt) => {
                self.selection = movement(mvmt, self.selection, &self.layout, true)
            }
            EditAction::Click(action) => {
                if action.mods.shift() {
                    self.selection.end = action.column;
                } else {
                    self.selection = Selection::caret(action.column);
                }
            }
            EditAction::Drag(action) => self.selection.end = action.column,
            EditAction::SelectAll => self.selection = Selection::new(0, self.layout.text.len()),
        }
    }

    /// Draw this editor at the provided point.
    pub fn draw(&self, ctx: &mut Piet, point: impl Into<Point>) {
        self.layout.draw(ctx, point)
    }

    /// Returns `true` if the data passed here has been changed externally,
    /// which means things like our selection state may be out of sync.
    ///
    /// This would only happen in the unlikely case that somebody else has mutated
    /// the data before us while handling an event; if this is the case we ignore
    /// the event, and our data will be updated in `update`.
    // fn data_is_stale(&self, data: &T) -> bool {
    //     self.layout
    //         .text()
    //         .map(|t| t.as_str() != data.as_str())
    //         .unwrap_or(true)
    // }

    fn insert(&mut self, text: &str) {
        // if we aren't multiline, we insert only up to the first newline
        let text = if self.multiline {
            text
        } else {
            text.split('\n').next().unwrap_or("")
        };
        let sel = self.selection.range();
        self.layout.text.edit(sel, text);
        self.selection = Selection::caret(self.selection.min() + text.len());
    }

    /// Delete backwards, using fancy logic when in caret mode.
    fn delete_backward(&mut self) {
        let cursor_pos = if self.selection.is_caret() {
            let del_end = self.selection.end;
            let del_start = offset_for_delete_backwards(&self.selection, &mut self.layout.text);
            self.layout.text.edit(del_start..del_end, "");
            del_start
        } else {
            self.layout.text.edit(self.selection.range(), "");
            self.selection.min()
        };

        self.selection = Selection::caret(cursor_pos);
    }

    fn delete_forward(&mut self) {
        let to_delete = if self.selection.is_caret() {
            movement(Movement::Right, self.selection, &self.layout, true)
        } else {
            self.selection
        };

        self.layout.text.edit(to_delete.range(), "");
        self.selection = Selection::caret(self.selection.min());
    }

    fn set_clipboard(&self) {
        if let Some(text) = self.layout.text.slice(self.selection.range()) {
            if !text.is_empty() {
                Application::global().clipboard().put_string(text);
            }
        }
    }
}

impl<T: From<&'static str>> Default for Editor<T> {
    fn default() -> Self {
        Editor::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Devanagari codepoints are 3 utf-8 code units each.
    #[test]
    fn backspace_devanagari() {
        let mut editor = Editor::<String>::new();
        // let mut data = "".to_string();
        // editor.set_text(data.clone());

        editor.insert("हिन्दी");
        editor.delete_backward();
        assert_eq!(editor.layout.text, String::from("हिन्द"));
        editor.delete_backward();
        assert_eq!(editor.layout.text, String::from("हिन्"));
        editor.delete_backward();
        assert_eq!(editor.layout.text, String::from("हिन"));
        editor.delete_backward();
        assert_eq!(editor.layout.text, String::from("हि"));
        editor.delete_backward();
        assert_eq!(editor.layout.text, String::from("ह"));
        editor.delete_backward();
        assert_eq!(editor.layout.text, String::from(""));
    }

    /// Test backspace on the combo character o̷
    #[test]
    fn backspace_combining() {
        let mut editor = Editor::<String>::new();
        // let mut data = "".to_string();
        // editor.set_text(data.clone());

        editor.insert("\u{0073}\u{006F}\u{0337}\u{0073}");

        editor.delete_backward();
        editor.delete_backward();

        assert_eq!(editor.layout.text, String::from("\u{0073}\u{006F}"))
    }
}
