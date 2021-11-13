use egrikor::ft_row;
use egrikor::widgets::button::checkbox_elem;
use egrikor::widgets::contextualize;
use egrikor::widgets::lists::row::row;
use egrikor::widgets::lists::{expand, ListContent};
use egrikor::widgets::textbox::{textbox, TextBoxContent};
use egrikor::*;

fn example(state: &bool) -> impl Element<(TextBoxContent, bool)> {
    ft_row![
        expand(checkbox_elem(
            *state,
            |state: &mut (TextBoxContent, bool)| state.1 = !state.1
        )),
        expand(contextualize(
            |s: &mut (TextBoxContent, bool)| &mut s.0,
            textbox(),
        )),
    ]
}

fn main() {
    run(
        "elements",
        (TextBoxContent::new(), false),
        move |state| example(&state.1),
    );
}
