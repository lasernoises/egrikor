use crate::*;

pub struct Popup<B, P, C> {
    pub base: B,
    pub popup: Option<P>,
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
    // base_rect
    Rect::from_origin_size(
        (base_rect.x0, base_rect.y1),
        (base_rect.width(), popup_min_size.height),
    )
}

pub struct State<B, Q> {
    base: B,
    popup: Option<Q>,
    min_size: Size,
    extra_layers: u8,
}

impl<E, B: Widget<E>, P: Widget<E>, C: Fn(&mut E)> Widget<E> for Popup<B, P, C> {
    type State = State<B::State, P::State>;

    fn build(
        &mut self,
        env: &mut E,
        constraint: LayoutConstraint,
        ctx: &mut LayoutCtx,
    ) -> Self::State {
        let base_state = self.base.build(env, constraint, ctx);
        let min_size = self.base.min_size(&base_state);

        let mut extra_layers = self.base.extra_layers(&base_state);

        let popup = self.popup.as_mut().map(|p| {
            let popup_state = p.build(env, LayoutConstraint { x: None, y: None }, ctx);
            extra_layers += 1 + p.extra_layers(&popup_state);
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
        env: &mut E,
        constraint: LayoutConstraint,
        ctx: &mut LayoutCtx,
    ) {
        self.base.update(&mut state.base, env, constraint, ctx);
        state.min_size = self.base.min_size(&state.base);
        state.extra_layers = self.base.extra_layers(&state.base);

        if let Some(ref mut popup) = self.popup {
            let popup_state = if let Some(ref mut state) = state.popup {
                popup.update(state, env, LayoutConstraint { x: None, y: None }, ctx);
                state
            } else {
                state.popup = Some(popup.build(env, LayoutConstraint { x: None, y: None }, ctx));
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
        env: &mut E,
        rect: Rect,
        layer: u8,
        focus: bool,
        ctx: &mut RenderCtx,
    ) {
        match layer {
            0 => self.base.render(&mut state.base, env, rect, 0, focus, ctx),
            1 => {
                if let Some(ref mut popup) = self.popup {
                    let state = state.popup.as_mut().unwrap();
                    let popup_rect = popup_rect(rect, popup.min_size(state));
                    popup.render(state, env, popup_rect, 0, focus, ctx);
                }
            }
            _ => (),
        }
    }

    fn test_input_pos_layer(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        rect: Rect,
        input_pos: Point,
    ) -> Option<u8> {
        if let Some(ref mut popup) = self.popup {
            let popup_state = state.popup.as_mut().unwrap();
            let popup_rect = popup_rect(rect, popup.min_size(popup_state));

            if let Some(layer) = popup.test_input_pos_layer(popup_state, env, popup_rect, input_pos) {
                Some(layer + 1)
            } else {
                drop(popup);

                self.base
                    .test_input_pos_layer(&mut state.base, env, rect, input_pos)
            }
        } else {
            self.base
                .test_input_pos_layer(&mut state.base, env, rect, input_pos)
        }
    }

    fn handle_cursor_input(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        rect: Rect,
        cursor_pos: Point,
        cursor_layer: u8,
        input: CursorInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
    ) -> InputReturn {
        if let Some(ref mut popup) = self.popup {
            let popup_state = state.popup.as_mut().unwrap();
            let popup_rect = popup_rect(rect, popup.min_size(popup_state));

            let click_outside_popup = if let CursorInput::Up(..) = input {
                !popup_rect.contains(cursor_pos)
            } else {
                false
            };

            if click_outside_popup {
                (self.on_close)(env);
            } else {
                popup.handle_cursor_input(
                    popup_state,
                    env,
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
            self.base.handle_cursor_input(
                &mut state.base,
                env,
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
