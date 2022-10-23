use super::popup::*;
use crate::pass_widget::{PassWidget, WidgetPassWidget, PassWidgetWidget};
use crate::*;

// use super::stateful_widget::stateful_widget;
use super::button::text_button;
use super::lists::iter::IterFlexContent;
use super::lists::{col, FlexItem};

// struct State {
//     open: bool,
// }

// impl WidgetState for State {
//     type WidgetState = <Self::Widget<'static> as Widget>::State;

//     type Widget<'a> = impl Widget + 'a where Self: 'a;

//     fn init_state() -> Self {
//         State { open: false }
//     }

//     fn build<'a>(&'a mut self) -> Self::Widget<'a> {
//         let open = self.open;

//         Popup {
//             params: self,
//             base: item!(p: State => text_button("Popup", || { p.open = true; })),
//             popup: if open {
//                 Some(item!(p: State => col(p, IterFlexContent {
//                     iter: (0..8).map(|x| flex_item!(p: State => text_button("Option", || {
//                         p.open = false;
//                     }))),
//                 })))
//             } else { None },
//             on_close: || {  },
//         }
//     }
// }

type StateWidgetState = impl WidgetState;

pub struct State {
    open: bool,
    widget_state: StateWidgetState,
}

impl WidgetState for State {
    fn new() -> Self {
        State {
            open: false,
            widget_state: StateWidgetState::new(),
        }
    }

    fn min_size(&self) -> Size {
        self.widget_state.min_size()
    }

    fn extra_layers(&self) -> u8 {
        self.widget_state.extra_layers()
    }
}

pub struct Dropdown<J, H> {
    selected: &'static str,
    items: J,
    on_select: H,
}

impl<E, I, J: Iterator<Item = (I, &'static str)> + Clone, H: Fn(&mut E, &I)> PassWidget<E>
    for Dropdown<J, H>
{
    type State = State;

    fn pass<R>(&mut self, state: &mut State, env: &mut E, pass: pass_widget::Pass<R>) -> R {
        let on_select = &self.on_select;

        let mut widget = WidgetPassWidget(Popup {
            base: text_button(self.selected, |e: &mut (&mut bool, &mut E)| {
                *e.0 = true;
            }),
            popup: if state.open {
                Some(col(IterFlexContent {
                    iter: self.items.clone().map(|(ident, label)| FlexItem {
                        widget: text_button(label, move |e: &mut (&mut bool, &mut E)| {
                            *e.0 = false;
                            on_select(e.1, &ident);
                        }),
                        expand: true,
                    }),
                }))
            } else {
                None
            },
            on_close: |e: &mut (&mut bool, &mut E)| *e.0 = false,
        });

        widget.pass(&mut state.widget_state, &mut (&mut state.open, env), pass)
    }
}

pub fn dropdown<I, E>(
    selected: &'static str,
    items: impl Iterator<Item = (I, &'static str)> + Clone,
    on_select: impl Fn(&mut E, &I),
) -> impl Widget<E, State = State> {
    PassWidgetWidget(Dropdown {
        selected,
        items,
        on_select,
    })
}
