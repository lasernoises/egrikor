use std::fmt::Debug;

use crate::*;

pub mod col;
pub mod row;

#[macro_export]
macro_rules! ft_list_content {
    () => {
        $crate::widgets::lists::EmptyListContent
    };
    ($($e:expr),+ $(,)?) => {
        $crate::widgets::lists::EmptyListContent $(.then($e))*
    };
}

pub trait WidgetHandler {
    fn widget<W: Widget>(&mut self, widget: &mut W, expand: bool);
}

pub trait ListContent {
    type Widgets: ListContentWidgets<C>;

    fn then<O: ListContent<C>>(self, other: O) -> Then<Self, O>
    where
        Self: Sized,
    {
        Then { a: self, b: other }
    }

    fn build(self) -> Self::Widgets;

    fn update(self, widgets: &mut Self::Widgets);
}

// impl<E: WidgetParams> ListContent<C> for E {
//     type Widgets = ListItemWidgets<C, E::Widget>;

//     fn build(self) -> Self::Widgets {
//         ListItemWidgets {
//             widget: self.build(),
//             expand: false,
//         }
//     }

//     fn update(self, widgets: &mut Self::Widgets) {
//         self.update(&mut widgets.widget);
//     }
// }

pub fn expand<E: WidgetParams>(element: E) -> ListItem<E> {
    ListItem {
        element,
        expand: true,
    }
}

pub trait ListContentWidgets {
    fn all<H: WidgetHandler<C>>(&mut self, handler: &mut H);
}

pub struct EmptyListContent;

pub struct EmptyListContentWidgets;

impl ListContent<C> for EmptyListContent {
    type Widgets = EmptyListContentWidgets;

    fn build(self) -> Self::Widgets {
        EmptyListContentWidgets
    }

    fn update(self, _: &mut Self::Widgets) {}
}

impl ListContentWidgets<C> for EmptyListContentWidgets {
    fn all<H: WidgetHandler<C>>(&mut self, _: &mut H) {}
}

pub struct ListItem<E> {
    element: E,
    expand: bool,
}

pub fn item<E: WidgetParams>(element: E, expand: bool) -> ListItem<E> {
    ListItem { element, expand }
}

pub struct ListItemWidgets<W> {
    widget: W,
    expand: bool,
}

impl<C: Debug, W: Widget> ListContentWidgets<C> for ListItemWidgets<W> {
    fn all<H: WidgetHandler<C>>(&mut self, handler: &mut H) {
        handler.widget(&mut self.widget, self.expand);
    }
}

impl<E: WidgetParams> ListContent<C> for ListItem<E> {
    type Widgets = ListItemWidgets<E::Widget>;

    fn build(self) -> Self::Widgets {
        ListItemWidgets {
            widget: self.element.build(),
            expand: self.expand,
        }
    }

    fn update(self, widgets: &mut Self::Widgets) {
        self.element.update(&mut widgets.widget);
        widgets.expand = self.expand;
    }
}

pub struct Then<A, B> {
    a: A,
    b: B,
}

impl<C: Debug, A: ListContent<C>, B: ListContent<C>> ListContent<C> for Then<A, B> {
    type Widgets = ThenWidgets<A::Widgets, B::Widgets>;

    fn build(self) -> Self::Widgets {
        ThenWidgets {
            a: self.a.build(),
            b: self.b.build(),
        }
    }

    fn update(self, widgets: &mut Self::Widgets) {
        self.a.update(&mut widgets.a);
        self.b.update(&mut widgets.b);
    }
}

pub struct ThenWidgets<A, B> {
    a: A,
    b: B,
}

impl<C: Debug, A: ListContentWidgets<C>, B: ListContentWidgets<C>> ListContentWidgets<C>
    for ThenWidgets<A, B>
{
    fn all<H: WidgetHandler<C>>(&mut self, handler: &mut H) {
        self.a.all(handler);
        self.b.all(handler);
    }
}

pub struct IterListContent<E, I: Iterator<Item = (E, bool)>> {
    iter: I,
}

pub fn iter<E: WidgetParams, I: Iterator<Item = (E, bool)>>(
    iter: I,
) -> IterListContent<E, I> {
    IterListContent { iter }
}

impl<E: WidgetParams, I: Iterator<Item = (E, bool)>> ListContent<C>
    for IterListContent<E, I>
{
    type Widgets = IterListContentWidgets<E::Widget>;

    fn build(self) -> Self::Widgets {
        IterListContentWidgets {
            widgets: self
                .iter
                .map(|(element, expand)| (element.build(), expand))
                .collect(),
        }
    }

    fn update(mut self, widgets: &mut Self::Widgets) {
        let mut iter_done = false;

        for i in 0..widgets.widgets.len() {
            if let Some((element, expand)) = self.iter.next() {
                element.update(&mut widgets.widgets[i].0);
                widgets.widgets[i].1 = expand;
            } else {
                widgets.widgets.truncate(i);
                iter_done = true;

                break;
            }
        }

        if !iter_done {
            // maybe we should take the upper bound into account?
            widgets.widgets.reserve(self.iter.size_hint().0);

            for (element, expand) in self.iter {
                widgets.widgets.push((element.build(), expand));
            }
        }
    }
}

pub struct IterListContentWidgets<W> {
    widgets: Vec<(W, bool)>,
}

impl<C: Debug, W: Widget> ListContentWidgets<C> for IterListContentWidgets<W> {
    fn all<H: WidgetHandler<C>>(&mut self, handler: &mut H) {
        for (widget, expand) in &mut self.widgets {
            handler.widget(widget, *expand);
        }
    }
}
