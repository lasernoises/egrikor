use crate::*;
use super::{popup::*, NoneWidget};

use super::stateful_widget::{WidgetState, stateful_widget};
use super::button::text_button;
use super::lists::col::col;
use super::lists::iter::IterFlexContent;

struct State {
    open: bool,
}

impl WidgetState for State {
    type WidgetState = <Self::Widget<'static> as Widget>::State;

    type Widget<'a> = impl Widget + 'a where Self: 'a;

    fn init_state() -> Self {
        State { open: false }
    }

    fn build<'a>(&'a mut self) -> Self::Widget<'a> {
        let open = self.open;

        Popup {
            params: self,
            base: item!(p: State => text_button("Popup", || { p.open = true; })),
            popup: if open {
                Some(item!(p: State => col(p, IterFlexContent {
                    iter: (0..8).map(|x| flex_item!(p: State => text_button("Option", || {
                        p.open = false;
                    }))),
                })))
            } else { None },
            on_close: || {  },
        }
    }
}

pub fn dropdown() -> impl Widget {
    stateful_widget::<State>()
}
