use super::{drawables, popup::*, NoneWidget};
use crate::*;

// use super::stateful_widget::stateful_widget;
use super::button::text_button;
use super::lists::iter::IterFlexContent;
use super::lists::{col, FlexItem};
use super::dyn_stateful_widget::*;

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

type State = impl Sized;

pub fn dropdown<I, E>(
    selected: &'static str,
    items: impl Iterator<Item = (I, &'static str)> + Clone,
    on_select: impl Fn(&mut E, &I),
) -> impl Widget<E, State = State> {
    stateful_widget(move |h: StatefulWidgetHandler<E, bool>| {
        let open = *h.state();
        let on_select = &on_select;
        h.widget(Popup {
            base: text_button(selected, |e: &mut (&mut bool, &mut E)| { *e.0 = true; }),
            popup: if open {
                Some(col(IterFlexContent {
                    iter: items.clone().map(|(ident, label)| FlexItem {
                        widget: text_button(label, move |e: &mut (&mut bool, &mut E)| {
                            *e.0 = false;
                            on_select(e.1, &ident);
                        }),
                        expand: true,
                    }),
                }))
            } else { None },
            on_close: |e: &mut (&mut bool, &mut E)| { *e.0 = false },
        });
    })
}
