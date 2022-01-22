#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use egrikor::widgets::stateful_widget::{stateful_widget, WidgetState};
use egrikor::widgets::lists::item;
// use egrikor::ft_row;
// use egrikor::widgets::button::checkbox_elem;
// use egrikor::widgets::contextualize;
// use egrikor::widgets::lists::row::row;
// use egrikor::widgets::lists::{expand, ListContent};
use egrikor::widgets::textbox::{textbox, TextBoxContent};
use egrikor::*;

// fn example(state: &bool) -> impl Element<(TextBoxContent, bool)> {
//     ft_row![
//         expand(checkbox_elem(
//             *state,
//             |state: &mut (TextBoxContent, bool)| state.1 = !state.1
//         )),
//         expand(contextualize(
//             |s: &mut (TextBoxContent, bool)| &mut s.0,
//             textbox(),
//         )),
//     ]
// }

struct MyState {
    a: TextBoxContent,
    b: TextBoxContent,
}

impl WidgetState for MyState {
    type Widget<'a> = impl Widget + 'a;
    type WidgetState = <Self::Widget<'static> as Widget>::State;

    fn init_state() -> Self {
        MyState { a: TextBoxContent::new(), b: TextBoxContent::new() }
    }

    fn build<'a>(&'a mut self) -> Self::Widget<'a> {
        row_widget![
            item(textbox(&mut self.a), false),
            item(textbox(&mut self.b), true),
        ]
    }
}

fn example() -> impl Widget {
    stateful_widget::<MyState>()
    // stateful_widget(|| TextBoxContent::new(), |s| textbox(s))
}

fn main() {
    run(
        "elements",
        example(),
        // (TextBoxContent::new(), false),
        // move |state| example(&state.1),
    );
}
