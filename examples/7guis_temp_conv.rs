#![feature(type_alias_impl_trait)]

use std::str::FromStr;
use egrikor::*;
use egrikor::widgets::lists::row;
use egrikor::widgets::textbox::{textbox, TextBoxContent};

fn example() -> impl Widget<Runtime> {
    struct State {
        celsius: TextBoxContent,
        fahrenheit: TextBoxContent,
    }

    stateful_widget!(State, State {
        celsius: TextBoxContent::new(),
        fahrenheit: TextBoxContent::new(),
    }, state => {
        row(flex_content![
            textbox::<(&mut State, &mut E)>(
                |e| &mut e.0.celsius,
                |e| {
                    let celsius = f64::from_str(e.0.celsius.text());

                    if let Ok(celsius) = celsius {
                        let fahrenheit = celsius * (9. / 5.) + 32.;
                        e.0.fahrenheit.set_text(format!("{fahrenheit}"));
                    }
                },
            ),
            textbox::<(&mut State, &mut E)>(
                |e| &mut e.0.fahrenheit,
                |e| {
                    let fahrenheit = f64::from_str(e.0.fahrenheit.text());

                    if let Ok(fahrenheit) = fahrenheit {
                        let celsius = (fahrenheit - 32.) * (5. / 9.);
                        e.0.celsius.set_text(format!("{celsius}"));
                    }
                },
            ),
        ])
    })
}

fn main() {
    run(
        "7GUIs TempConv",
        example(),
    );
}
