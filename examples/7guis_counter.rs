#![feature(type_alias_impl_trait)]

use egrikor::widgets::button::text_button;
use egrikor::*;
use egrikor::widgets::lists::row;
use egrikor::widgets::drawables::text;

fn example() -> impl Widget<Runtime> {
    stateful_widget!(u32, 0, count => {
        let label = Box::leak(format!("{count}").into_boxed_str());

        row(flex_content![
            text(label),
            text_button::<(&mut u32, &mut E), _>("Count", |e| { *e.0 += 1; }),
        ])
    })
}

fn main() {
    run(
        "7GUIs Counter",
        example(),
    );
}
