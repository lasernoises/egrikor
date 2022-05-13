use crate::*;

use super::lists::FlexItemBuild;

pub struct Popup<'a, P, B, Q, C> {
    pub params: &'a mut P,
    pub base: B,
    pub popup: Option<Q>,
    pub on_close: C,
}

// impl<B: Widget, P: Widget, C> Popup<B, P, C> {
//     fn popup_rect<C: Debug>(&mut self, base_rect: Rect) -> Rect {
//         Rect::from_origin_size(
//             (base_rect.x0, base_rect.y1),
//             (base_rect.width(), self.popup.min_size().height),
//         )
//     }
// }

fn popup_rect(base_rect: Rect, popup_min_size: Size) -> Rect {
    base_rect
    // Rect::from_origin_size(
    //     (base_rect.x0, base_rect.y1),
    //     (base_rect.width(), popup_min_size.height),
    // )
}

pub struct State<B, Q> {
    base: B,
    popup: Option<Q>,
    min_size: Size,
    extra_layers: u8,
}

impl<'a, P, B: FlexItemBuild<Params = P>, Q: FlexItemBuild<Params = P>, C: FnMut()> Widget for Popup<'a, P, B, Q, C> {
    type State = State<B::State, Q::State>;

    fn build(
        &mut self,
        constraint: LayoutConstraint,
        ctx: &mut LayoutCtx,
    ) -> Self::State {
        let mut base = self.base.build(self.params);
        let base_state = base.build(constraint, ctx);
        let min_size = base.min_size(&base_state);

        let mut extra_layers = base.extra_layers(&base_state);

        drop(base);

        let popup = self
            .popup
            .as_ref()
            .map(|p| {
                let mut popup = p.build(self.params);
                let popup_state = popup.build([None; 2], ctx);
                extra_layers += 1 + popup.extra_layers(&popup_state);
                popup_state
            });

        State {
            base: base_state,
            popup,
            min_size,
            extra_layers,
        }
    }

    fn update(
        &mut self,
        state: &mut Self::State,
        constraint: LayoutConstraint,
        ctx: &mut LayoutCtx,
    ) {
        let mut base = self.base.build(self.params);
        base.update(&mut state.base, constraint, ctx);
        state.min_size = base.min_size(&state.base);
        state.extra_layers = base.extra_layers(&state.base);

        drop(base);

        if let Some(ref mut popup) = self.popup {
            let mut popup = popup.build(self.params);
            let popup_state = if let Some(ref mut state) = state.popup {
                popup.update(state, [None; 2], ctx);
                state
            } else {
                state.popup = Some(popup.build([None; 2], ctx));
                state.popup.as_mut().unwrap()
            };

            state.extra_layers += 1 + popup.extra_layers(&popup_state);
        } else {
            state.popup = None;
        }
    }

    fn min_size(&self, state: &Self::State) -> Size {
        // self.base.build(self.params).min_size(&state.0)
        state.min_size
    }

    fn extra_layers(&self, state: &Self::State) -> u8 {
        // self.base.build(self.params).extra_layers(&state.0)
        //     + 1
        //     + self
        //         .popup
        //         .as_ref()
        //         .map(|p| p.build(self.params).extra_layers(state.1.as_ref().unwrap()))
        //         .unwrap_or(0)
        state.extra_layers
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
        match layer {
            0 => self
                .base
                .build(self.params)
                .render(&mut state.base, rect, renderer, theme, input_state, 0, focus),
            1 => {
                if let Some(ref mut popup) = self.popup {
                    let mut popup = popup.build(self.params);
                    let state = state.popup.as_mut().unwrap();
                    let popup_rect = popup_rect(rect, popup.min_size(state));
                    popup.render(state, popup_rect, renderer, theme, input_state, 0, focus);
                }
            }
            _ => (),
        }
    }

    fn test_input_pos_layer(
        &mut self,
        state: &mut Self::State,
        rect: Rect,
        input_pos: Point,
    ) -> Option<u8> {
        if let Some(ref mut popup) = self.popup {
            let mut popup = popup.build(self.params);
            let popup_state = state.popup.as_mut().unwrap();
            let popup_rect = popup_rect(rect, popup.min_size(popup_state));

            if let Some(layer) = popup.test_input_pos_layer(popup_state, popup_rect, input_pos) {
                Some(layer + 1)
            } else {
                drop(popup);

                self.base
                    .build(self.params)
                    .test_input_pos_layer(&mut state.base, rect, input_pos)
            }
        } else {
            self.base
                .build(self.params)
                .test_input_pos_layer(&mut state.base, rect, input_pos)
        }
    }

    fn handle_cursor_input(
        &mut self,
        state: &mut Self::State,
        rect: Rect,
        cursor_pos: Point,
        cursor_layer: u8,
        input: CursorInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
    ) -> InputReturn {
        if let Some(ref mut popup) = self.popup {
            let mut popup = popup.build(self.params);
            let popup_state = state.popup.as_mut().unwrap();
            let popup_rect = popup_rect(rect, popup.min_size(popup_state));

            let click_outside_popup = if let CursorInput::Up(..) = input {
                !popup_rect.contains(cursor_pos)
            } else {
                false
            };

            if click_outside_popup {
                (self.on_close)();
            } else {
                popup.handle_cursor_input(
                    popup_state,
                    popup_rect,
                    cursor_pos,
                    cursor_layer.saturating_sub(1),
                    input,
                    input_state,
                    theme,
                    focus,
                );
            }
        } else {
            self.base.build(self.params).handle_cursor_input(
                &mut state.base,
                rect,
                cursor_pos,
                cursor_layer,
                input,
                input_state,
                theme,
                focus,
            );
        }

        Default::default()
    }
}
