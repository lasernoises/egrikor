#[macro_export]
macro_rules! stateful_widget {
    ($state:ident, $init:expr, $state_var:ident => $build:expr) => {{
        type StateWidgetState = impl $crate::WidgetState;
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

            min_size: druid_shell::kurbo::Size,
            extra_layers: u8,
        }

        impl $crate::WidgetState for StatefulState {
            fn new() -> Self {
                Self {
                    state: $init,
                    widget_state: StateWidgetState::new(),
                    min_size: druid_shell::kurbo::Size::ZERO,
                    extra_layers: 0,
                }
            }

            fn min_size(&self) -> druid_shell::kurbo::Size {
                self.min_size
            }

            fn extra_layers(&self) -> u8 {
                self.extra_layers
            }
        }

        struct StatefulWidget;

        impl<E> $crate::pass_widget::PassWidget<E> for StatefulWidget {
            type State = StatefulState;

            fn pass<R>(&mut self, state: &mut Self::State, env: &mut E, pass: $crate::pass_widget::Pass<R>) -> R {
                let mut widget = $crate::pass_widget::WidgetPassWidget(build(&mut state.state));

                let result = widget.pass(&mut state.widget_state, &mut (&mut state.state, env), pass);

                state.min_size = state.widget_state.min_size();
                state.extra_layers = state.widget_state.extra_layers();

                result
            }
        }

        $crate::pass_widget::PassWidgetWidget(StatefulWidget)
    }};
}
