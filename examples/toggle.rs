use egrikor::ft_row;
use egrikor::widgets::button::{checkbox_elem, text_button_elem};
use egrikor::widgets::lists::row::row;
use egrikor::widgets::lists::{expand, ListContent};
use egrikor::*;

fn toggle(state: &bool) -> impl Element<bool> {
    ft_row![
        expand(text_button_elem(
            if *state { "true" } else { "false" },
            |state: &mut bool| *state = !*state,
        )),
        expand(text_button_elem(
            if *state { "false" } else { "true" },
            |state: &mut bool| *state = !*state,
        )),
        expand(checkbox_elem(*state, |state: &mut bool| *state = !*state,)),
    ]
}

fn main() {
    run("elements", false, move |state| toggle(state));
}
