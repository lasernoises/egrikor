use egrikor::widgets::dropdown::dropdown;
use egrikor::*;

fn example() -> impl Widget<Runtime> {
    // row(flex_content![
    //     stateful_widget(|b: &mut u8| col(flex_content![
    //         dropdown(),
    //         checkbox(*b == 0, |b: &mut u8| *b = 0),
    //         checkbox(*b == 1, |b: &mut u8| *b = 1),
    //         checkbox(*b == 2, |b: &mut u8| *b = 2),
    //         stateful_widget(|b: &mut bool| checkbox(*b, |b: &mut bool| *b = !*b)),
    //     ])),
    //     stateful_widget(|c: &mut TextBoxContent| col(flex_content![
    //         textbox(|c| c),
    //         textbox(|c| c),
    //     ])),
    // ])
    dropdown()
}

fn main() {
    run(
        "Basic",
        example(),
    );
}
