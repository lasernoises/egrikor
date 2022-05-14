use crate::*;
use super::{button::Button, or::OrWidget, drawables::{Checkmark, FixedRect}, NoneWidget};

pub fn checkbox<E>(checked: bool, on_click: impl FnMut(&mut E)) -> impl Widget<E> {
    Button {
        widget: if checked {
            OrWidget::<_, _, NoneWidget, NoneWidget>::A(Checkmark(Size::new(16., 16.)))
        } else {
            OrWidget::B(FixedRect(Size::new(16., 16.)))
        },
        on_click,
    }
}
