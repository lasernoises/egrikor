use std::marker::PhantomData;

use druid_shell::kurbo::Size;

use crate::Widget;

#[macro_export]
macro_rules! stateful_widget {
    ($state:ident, $init_state:expr, $build:ident) => {
        struct StatefulWidget<I> {
            init_state: I,
        }

        struct State {
            state: $state,
            widget_state: 

            min_size: druid_shell::kurbo::Size,
            extra_layers: u8,
        }

        impl<I: Fn() -> $state> Widget for StatefulWidget<I> {
            type Event = ();
            type State = State;

            fn build(
                &mut self,
                constraint: crate::LayoutConstraint,
                renderer: &mut piet_common::Piet,
                theme: &super::Theme,
            ) -> Self::State {
                let mut state = (self.init_state)();
                let mut widget = $build(&mut state);

                let widget_state = widget.build(constraint, renderer, theme);

                let min_size = widget.min_size(&widget_state);
                let extra_layers = widget.extra_layers(&widget_state);

                StatefulWidgetState {
                    state,
                    widget_state,

                    min_size,
                    extra_layers,
                }
            }

            fn update(
                &mut self,
                state: &mut Self::State,
                constraint: crate::LayoutConstraint,
                renderer: &mut piet_common::Piet,
                theme: &super::Theme,
            ) {
                let mut widget = $build(&mut state.state);
                widget.update(&mut state.widget_state, constraint, renderer, theme);

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
                rect: druid_shell::kurbo::Rect,
                renderer: &mut piet_common::Piet,
                theme: &super::Theme,
                input_state: &crate::InputState,
                layer: u8,
                focus: bool,
            ) {
                let mut widget = $build(&mut state.state);
                widget.render(
                    &mut state.widget_state,
                    rect,
                    renderer,
                    theme,
                    input_state,
                    layer,
                    focus,
                );
            }

            fn test_input_pos_layer(
                &mut self,
                state: &mut Self::State,
                rect: druid_shell::kurbo::Rect,
                input_pos: druid_shell::kurbo::Point,
            ) -> Option<u8> {
                let mut widget = $build(&mut state.state);
                widget.test_input_pos_layer(&mut state.widget_state, rect, input_pos)
            }

            fn handle_cursor_input(
                &mut self,
                state: &mut Self::State,
                rect: druid_shell::kurbo::Rect,
                cursor_pos: druid_shell::kurbo::Point,
                cursor_layer: u8,
                input: crate::CursorInput,
                input_state: &crate::InputState,
                theme: &super::Theme,
                focus: bool,
            ) -> (crate::InputReturn, Option<Self::Event>) {
                let mut widget = $build(&mut state.state);
                (
                    widget
                        .handle_cursor_input(
                            &mut state.widget_state,
                            rect,
                            cursor_pos,
                            cursor_layer,
                            input,
                            input_state,
                            theme,
                            focus,
                        )
                        .0,
                    None,
                )
            }

            fn handle_keyboard_input(
                &mut self,
                state: &mut Self::State,
                rect: druid_shell::kurbo::Rect,
                input: &crate::KeyboardInput,
                input_state: &crate::InputState,
                theme: &super::Theme,
                focus: bool,
            ) -> Option<Self::Event> {
                let mut widget = $build(&mut state.state);
                widget.handle_keyboard_input(
                    &mut state.widget_state,
                    rect,
                    input,
                    input_state,
                    theme,
                    focus,
                );

                None
            }
        }

        StatefulWidget {

        }
    };
}

pub trait WidgetState {
    type WidgetState;
    type Widget<'a>: Widget<State = Self::WidgetState> + 'a where Self: 'a;
    // type Widget: Widget + 'a;

    fn init_state() -> Self;

    fn build<'a>(&'a mut self) -> Self::Widget<'a>;
}

struct StatefulWidget<S> {
    s: PhantomData<S>,
}

struct StatefulWidgetState<S, W> {
    state: S,
    widget_state: W,

    min_size: Size,
    extra_layers: u8,
}

impl<S: WidgetState> Widget for StatefulWidget<S>
    // where Self: 'a,
{
    type Event = ();
    type State = StatefulWidgetState<S, S::WidgetState>;

    fn build(
        &mut self,
        constraint: crate::LayoutConstraint,
        renderer: &mut piet_common::Piet,
        theme: &super::Theme,
    ) -> Self::State {
        let mut state = S::init_state();
        let mut widget = state.build();

        let widget_state = widget.build(constraint, renderer, theme);

        let min_size = widget.min_size(&widget_state);
        let extra_layers = widget.extra_layers(&widget_state);

        drop(widget);

        StatefulWidgetState {
            state,
            widget_state,

            min_size,
            extra_layers,
        }
    }

    fn update(
        &mut self,
        state: &mut Self::State,
        constraint: crate::LayoutConstraint,
        renderer: &mut piet_common::Piet,
        theme: &super::Theme,
    ) {
        let mut widget = state.state.build();
        widget.update(&mut state.widget_state, constraint, renderer, theme);

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
        rect: druid_shell::kurbo::Rect,
        renderer: &mut piet_common::Piet,
        theme: &super::Theme,
        input_state: &crate::InputState,
        layer: u8,
        focus: bool,
    ) {
        let mut widget = state.state.build();
        widget.render(
            &mut state.widget_state,
            rect,
            renderer,
            theme,
            input_state,
            layer,
            focus,
        );
    }

    fn test_input_pos_layer(
        &mut self,
        state: &mut Self::State,
        rect: druid_shell::kurbo::Rect,
        input_pos: druid_shell::kurbo::Point,
    ) -> Option<u8> {
        let mut widget = state.state.build();
        widget.test_input_pos_layer(&mut state.widget_state, rect, input_pos)
    }

    fn handle_cursor_input(
        &mut self,
        state: &mut Self::State,
        rect: druid_shell::kurbo::Rect,
        cursor_pos: druid_shell::kurbo::Point,
        cursor_layer: u8,
        input: crate::CursorInput,
        input_state: &crate::InputState,
        theme: &super::Theme,
        focus: bool,
    ) -> (crate::InputReturn, Option<Self::Event>) {
        let mut widget = state.state.build();
        (
            widget
                .handle_cursor_input(
                    &mut state.widget_state,
                    rect,
                    cursor_pos,
                    cursor_layer,
                    input,
                    input_state,
                    theme,
                    focus,
                )
                .0,
            None,
        )
    }

    fn handle_keyboard_input(
        &mut self,
        state: &mut Self::State,
        rect: druid_shell::kurbo::Rect,
        input: &crate::KeyboardInput,
        input_state: &crate::InputState,
        theme: &super::Theme,
        focus: bool,
    ) -> Option<Self::Event> {
        let mut widget = state.state.build();
        widget.handle_keyboard_input(
            &mut state.widget_state,
            rect,
            input,
            input_state,
            theme,
            focus,
        );

        None
    }
}

pub fn stateful_widget<'a, S: WidgetState>() -> impl Widget
where
    S: 'static,
{
    StatefulWidget {
        s: PhantomData::<S>,
    }
}
