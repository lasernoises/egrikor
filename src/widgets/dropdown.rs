use crate::*;
use super::{popup::*, NoneWidget};
use super::checkbox::checkbox;

use super::stateful_widget::{WidgetState, stateful_widget};
use super::lists::row::row;
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
            base: item!(p: State => checkbox(true, || { p.open = true; })),
            popup: if open {
                Some(item!(p: State => row(p, IterFlexContent {
                    iter: (0..8).map(|x| flex_item!(p: State => checkbox(true, || {
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
