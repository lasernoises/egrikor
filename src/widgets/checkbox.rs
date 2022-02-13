use crate::*;
use super::{button::Button, or::OrWidget, drawables::{Checkmark, FixedRect}, NoneWidget};

pub fn checkbox<'a>(checked: &'a mut bool) -> impl Widget + 'a {
    Button {
        widget: if *checked {
            OrWidget::<_, _, NoneWidget, NoneWidget>::A(Checkmark(Size::new(16., 16.)))
        } else {
            OrWidget::B(FixedRect(Size::new(16., 16.)))
        },
        on_click: || {
            *checked = !*checked;
        },
    }
}
