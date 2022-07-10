#[macro_export]
macro_rules! stateful_widget {
    ($state:ident, $init:expr, $state_var:ident => $build:expr) => {{
        type StateWidgetState = impl Sized;
        type StateWidget<'a, 'b, E: 'b> = impl $crate::Widget<
            (&'a mut $state, &'b mut E),
            State = StateWidgetState,
        >;

        fn build<'a, 'b, E: 'b>($state_var: &mut $state) -> StateWidget<'a, 'b, E> {
            $build
        }

        struct StatefulState {
            state: $state,
            widget_state: StateWidgetState,

            min_size: Size,
            extra_layers: u8,
        }

        struct StatefulWidget;

        impl<E> $crate::Widget<E> for StatefulWidget {
            type State = StatefulState;

            fn build(
                &mut self,
                env: &mut E,
                constraint: crate::LayoutConstraint,
                ctx: &mut LayoutCtx,
            ) -> Self::State {
                let mut state: $state = $init;
                let mut widget = build(&mut state);

                let mut env = (&mut state, env);
                let widget_state = widget.build(&mut env, constraint, ctx);

                let min_size = widget.min_size(&widget_state);
                let extra_layers = widget.extra_layers(&widget_state);

                drop(widget);

                StatefulState {
                    state,
                    widget_state,

                    min_size,
                    extra_layers,
                }
            }

            fn update(
                &mut self,
                state: &mut Self::State,
                env: &mut E,
                constraint: crate::LayoutConstraint,
                ctx: &mut LayoutCtx,
            ) {
                let mut widget = build(&mut state.state);
                widget.update(&mut state.widget_state, &mut (&mut state.state, env), constraint, ctx);

                state.min_size = widget.min_size(&state.widget_state);
                state.extra_layers = widget.extra_layers(&state.widget_state);
            }

            fn min_size(&self, state: &Self::State) -> druid_shell::kurbo::Size {
                state.min_size
            }

            fn extra_layers(&self, state: &Self::State) -> u8 {
                state.extra_layers
            }

            fn render(
                &mut self,
                state: &mut Self::State,
                env: &mut E,
                rect: druid_shell::kurbo::Rect,
                layer: u8,
                focus: bool,
                ctx: &mut RenderCtx,
            ) {
                let mut widget = build(&mut state.state);
                widget.render(
                    &mut state.widget_state,
                    &mut (&mut state.state, env),
                    rect,
                    layer,
                    focus,
                    ctx,
                );
            }

            fn test_input_pos_layer(
                &mut self,
                state: &mut Self::State,
                env: &mut E,
                rect: druid_shell::kurbo::Rect,
                input_pos: druid_shell::kurbo::Point,
            ) -> Option<u8> {
                let mut widget = build(&mut state.state);
                widget.test_input_pos_layer(&mut state.widget_state, &mut (&mut state.state, env), rect, input_pos)
            }

            fn handle_cursor_input(
                &mut self,
                state: &mut Self::State,
                env: &mut E,
                rect: druid_shell::kurbo::Rect,
                cursor_pos: druid_shell::kurbo::Point,
                cursor_layer: u8,
                input: crate::CursorInput,
                input_state: &crate::InputState,
                theme: &super::Theme,
                focus: bool,
            ) -> crate::InputReturn {
                let mut widget = build(&mut state.state);

                widget.handle_cursor_input(
                    &mut state.widget_state,
                    &mut (&mut state.state, env),
                    rect,
                    cursor_pos,
                    cursor_layer,
                    input,
                    input_state,
                    theme,
                    focus,
                )
            }

            fn handle_keyboard_input(
                &mut self,
                state: &mut Self::State,
                env: &mut E,
                rect: druid_shell::kurbo::Rect,
                input: &crate::KeyboardInput,
                input_state: &crate::InputState,
                theme: &super::Theme,
                focus: bool,
            ) {
                let mut widget = build(&mut state.state);
                widget.handle_keyboard_input(
                    &mut state.widget_state,
                    &mut (&mut state.state, env),
                    rect,
                    input,
                    input_state,
                    theme,
                    focus,
                );
            }
        }

        StatefulWidget
    }};
}
