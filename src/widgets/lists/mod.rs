use std::fmt::Debug;

use crate::*;

// pub mod col;
pub mod iter;
pub mod flex;

pub use flex::{row, col};

// pub mod iter;

// #[macro_export]
// macro_rules! item {
//     ($params_var:ident: $params:ty => $build:expr) => {{
//         struct Item;

//         impl crate::widgets::lists::FlexItemBuild for Item {
//             type Params = $params;
//             type Widget<'a> = impl Widget + 'a;
//             type State = <Self::Widget<'static> as Widget>::State;

//             fn build<'a>(&self, $params_var: &'a mut $params) -> Self::Widget<'a> {
//                 $build
//             }
//         }

//         Item
//     }};
// }

// #[macro_export]
// macro_rules! flex_item {
//     ($params_var:ident: $params:ty => $build:expr) => {{
//         $crate::widgets::lists::FlexItem {
//             build: item!($params_var: $params => $build),
//             expand: true,
//         }
//     }};
// }

#[macro_export]
macro_rules! flex_content {
    // () => {
    //     $crate::widgets::lists::EmptyFlexContent
    // };
    ($($build:expr),* $(,)?) => {{
        use $crate::widgets::lists::FlexContent;
        $crate::widgets::lists::EmptyFlexContent $(.then(
            // crate::flex_item!($params_var: $params => $build)
            $crate::widgets::lists::FlexItem {
                widget: $build,
                expand: true,
            }
        ))*
    }};
}

pub trait FlexContent<E> {
    type State: FlexContentState;

    fn all<H: FlexContentHandler<E>>(
        &mut self,
        state: &mut Self::State,
        handler: &mut H,
    );

    fn then<O: FlexContent<E>>(self, other: O) -> Then<Self, O>
    where
        Self: Sized,
    {
        Then { a: self, b: other }
    }
}

pub trait FlexContentHandler<E> {
    fn widget<W: Widget<E>>(&mut self, widget: &mut W, state: &mut Option<W::State>, expand: bool);
}

pub trait FlexContentState {
    fn new() -> Self;
}

// pub trait FlexItemBuild {
//     type Params;
//     type State;
//     type Widget<'a>: Widget<State = Self::State> + 'a;

//     fn build<'a>(&self, params: &'a mut Self::Params) -> Self::Widget<'a>;
// }

pub struct FlexItem<W> {
    pub widget: W,
    pub expand: bool,
}

pub struct FlexItemState<S> {
    state: Option<S>,
}

impl<E, W: Widget<E>> FlexContent<E> for FlexItem<W> {
    type State = FlexItemState<W::State>;

    fn all<H: FlexContentHandler<E>>(
        &mut self,
        state: &mut Self::State,
        handler: &mut H,
    ) {
        // let mut widget = self.build.build(params);
        handler.widget(&mut self.widget, &mut state.state, self.expand);
    }
}

impl<S> FlexContentState for FlexItemState<S> {
    fn new() -> Self {
        FlexItemState { state: None }
    }
}

// pub fn item<W: Widget>(widget: W, expand: bool) -> FlexItem<W> {
//     FlexItem { widget, expand }
// }

// pub fn expand<W: Widget>(widget: W) -> FlexItem<W> {
//     FlexItem {
//         widget,
//         expand: true,
//     }
// }

pub struct EmptyFlexContent;

pub struct EmptyFlexContentState;

impl<E> FlexContent<E> for EmptyFlexContent {
    type State = EmptyFlexContentState;

    fn all<H: FlexContentHandler<E>>(&mut self, _: &mut Self::State, _: &mut H) {}
}

impl FlexContentState for EmptyFlexContentState {
    fn new() -> Self {
        EmptyFlexContentState
    }
}

pub struct Then<A, B> {
    a: A,
    b: B,
}

pub struct ThenState<A, B> {
    a: A,
    b: B,
}

impl<E, A: FlexContent<E>, B: FlexContent<E>> FlexContent<E> for Then<A, B> {
    type State = ThenState<A::State, B::State>;

    fn all<H: FlexContentHandler<E>>(
        &mut self,
        state: &mut Self::State,
        handler: &mut H,
    ) {
        self.a.all(&mut state.a, handler);
        self.b.all(&mut state.b, handler);
    }
}

impl<A: FlexContentState, B: FlexContentState> FlexContentState for ThenState<A, B> {
    fn new() -> Self {
        ThenState {
            a: A::new(),
            b: B::new(),
        }
    }
}

// impl<C: Debug, A: ListContent, B: ListContent> ListContent for Then<A, B> {
//     type Widgets = ThenWidgets<A::Widgets, B::Widgets>;

//     fn build(self) -> Self::Widgets {
//         ThenWidgets {
//             a: self.a.build(),
//             b: self.b.build(),
//         }
//     }

//     fn update(self, widgets: &mut Self::Widgets) {
//         self.a.update(&mut widgets.a);
//         self.b.update(&mut widgets.b);
//     }
// }

// pub struct ThenWidgets<A, B> {
//     a: A,
//     b: B,
// }

// impl<C: Debug, A: ListContentWidgets, B: ListContentWidgets> ListContentWidgets
//     for ThenWidgets<A, B>
// {
//     fn all<H: WidgetHandler>(&mut self, handler: &mut H) {
//         self.a.all(handler);
//         self.b.all(handler);
//     }
// }

// pub struct IterListContent<E, I: Iterator<Item = (E, bool)>> {
//     iter: I,
// }

// pub fn iter<E: Widget, I: Iterator<Item = (E, bool)>>(
//     iter: I,
// ) -> IterListContent<E, I> {
//     IterListContent { iter }
// }

// impl<E: Widget, I: Iterator<Item = (E, bool)>> ListContent for IterListContent<E, I> {
//     type Widgets = IterListContentWidgets<E::Widget>;

//     fn build(self) -> Self::Widgets {
//         IterListContentWidgets {
//             widgets: self
//                 .iter
//                 .map(|(element, expand)| (element.build(), expand))
//                 .collect(),
//         }
//     }

//     fn update(mut self, widgets: &mut Self::Widgets) {
//         let mut iter_done = false;

//         for i in 0..widgets.widgets.len() {
//             if let Some((element, expand)) = self.iter.next() {
//                 element.update(&mut widgets.widgets[i].0);
//                 widgets.widgets[i].1 = expand;
//             } else {
//                 widgets.widgets.truncate(i);
//                 iter_done = true;

//                 break;
//             }
//         }

//         if !iter_done {
//             // maybe we should take the upper bound into account?
//             widgets.widgets.reserve(self.iter.size_hint().0);

//             for (element, expand) in self.iter {
//                 widgets.widgets.push((element.build(), expand));
//             }
//         }
//     }
// }

// pub struct IterListContentWidgets<S> {
//     widgets: Vec<(S, bool)>,
// }

// impl<C: Debug, S> ListContentWidgets for IterListContentWidgets<S> {
//     fn all<H: WidgetHandler>(&mut self, handler: &mut H) {
//         for (widget, expand) in &mut self.widgets {
//             handler.widget(widget, *expand);
//         }
//     }
// }
