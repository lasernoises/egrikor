use crate::*;
use super::{popup::*, NoneWidget};

use super::stateful_widget::stateful_widget;
use super::button::text_button;
use super::lists::{col, FlexItem};
use super::lists::iter::IterFlexContent;

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

pub fn dropdown<E>() -> impl Widget<E> {
    stateful_widget(|open: &mut bool| {

        Popup {
            base: text_button("Popup", |open: &mut bool| { *open = true; }),
            popup: if *open {
                Some(col(IterFlexContent {
                    iter: (0..8).map(|x| FlexItem {
                        widget: text_button("Option", |open: &mut bool| {
                            *open = false;
                        }),
                        expand: true,
                    }),
                }))
            } else { None },
            on_close: |open: &mut bool| { *open = false },
        }
    })
}
