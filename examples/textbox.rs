#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use egrikor::widgets::checkbox::checkbox;
use egrikor::widgets::stateful_widget::{stateful_widget, WidgetState};
use egrikor::widgets::lists::item;
// use egrikor::ft_row;
// use egrikor::widgets::button::checkbox_elem;
// use egrikor::widgets::contextualize;
// use egrikor::widgets::lists::row::row;
// use egrikor::widgets::lists::{expand, ListContent};
use egrikor::widgets::textbox::{textbox, TextBoxContent};
use egrikor::widgets::drawables::text;
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
    checked: bool,
}

impl WidgetState for MyState {
    type Widget<'a> = impl Widget + 'a;
    type WidgetState = <Self::Widget<'static> as Widget>::State;

    fn init_state() -> Self {
        MyState {
            a: TextBoxContent::new(),
            b: TextBoxContent::new(),
            checked: false,
        }
    }

    fn build<'a>(&'a mut self) -> Self::Widget<'a> {
        row_widget![
            item(textbox(&mut self.a), true),
            item(text("Hello"), true),
            item(textbox(&mut self.b), true),
            item(
                checkbox(self.checked, || self.checked = !self.checked),
                false,
            ),
        ]
    }
}

fn example() -> impl Widget {
    stateful_widget::<MyState>()
    // stateful_widget(|| TextBoxContent::new(), |s| textbox(s))
}

fn main() {
    run(
        "TextBox",
        example(),
        // (TextBoxContent::new(), false),
        // move |state| example(&state.1),
    );
}
