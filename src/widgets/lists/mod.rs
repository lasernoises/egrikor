use std::fmt::Debug;

use crate::*;

pub mod col;
pub mod row;
pub mod iter;

#[macro_export]
macro_rules! flex_item {
    ($params_var:ident: $params:ty => $build:expr) => {{
        struct Item;

        impl crate::widgets::lists::FlexItemBuild for Item {
            type Params = $params;
            type Widget<'a> = impl Widget + 'a;
            type State = <Self::Widget<'static> as Widget>::State;

            fn build<'a>(&self, $params_var: &'a mut $params) -> Self::Widget<'a> {
                $build
            }
        }

        FlexItem {
            build: Item,
            expand: true,
        }
    }};
}

#[macro_export]
macro_rules! flex_content {
    // () => {
    //     $crate::widgets::lists::EmptyFlexContent
    // };
    ($params_var:ident: $params:ty => [$($build:expr),* $(,)?]) => {{
        use $crate::widgets::lists::FlexContent;
        $crate::widgets::lists::EmptyFlexContent $(.then(
            crate::flex_item!($params_var: $params => $build)
        ))*
    }};
}

pub trait FlexContent<P> {
    type State: FlexContentState;

    fn all<H: FlexContentHandler>(
        &mut self,
        params: &mut P,
        state: &mut Self::State,
        handler: &mut H,
    );

    fn then<O: FlexContent<P>>(self, other: O) -> Then<Self, O>
    where
        Self: Sized,
    {
        Then { a: self, b: other }
    }
}

pub trait FlexContentHandler {
    fn widget<W: Widget>(&mut self, widget: &mut W, state: &mut Option<W::State>, expand: bool);
}

pub trait FlexContentState {
    fn new() -> Self;
}

pub trait FlexItemBuild {
    type Params;
    type State;
    type Widget<'a>: Widget<State = Self::State> + 'a;

    fn build<'a>(&self, params: &'a mut Self::Params) -> Self::Widget<'a>;
}

pub struct FlexItem<B> {
    pub build: B,
    pub expand: bool,
}

pub struct FlexItemState<S> {
    state: Option<S>,
}

impl<P, B: FlexItemBuild<Params = P>> FlexContent<P> for FlexItem<B> {
    type State = FlexItemState<B::State>;

    fn all<H: FlexContentHandler>(
        &mut self,
        params: &mut P,
        state: &mut Self::State,
        handler: &mut H,
    ) {
        let mut widget = self.build.build(params);
        handler.widget(&mut widget, &mut state.state, self.expand);
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

impl<P> FlexContent<P> for EmptyFlexContent {
    type State = EmptyFlexContentState;

    fn all<H: FlexContentHandler>(&mut self, _: &mut P, _: &mut Self::State, _: &mut H) {}
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

impl<P, A: FlexContent<P>, B: FlexContent<P>> FlexContent<P> for Then<A, B> {
    type State = ThenState<A::State, B::State>;

    fn all<H: FlexContentHandler>(
        &mut self,
        params: &mut P,
        state: &mut Self::State,
        handler: &mut H,
    ) {
        self.a.all(params, &mut state.a, handler);
        self.b.all(params, &mut state.b, handler);
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
