use std::fmt::Debug;

use druid_shell::kurbo::{Point, Rect, Size};
use piet_common::Piet;

// use super::drawables::{checkmark_elem, fixed_rect_elem};
// use super::or::OrElem;
use super::*;
use crate::*;

pub fn text_button<E>(text: &'static str, on_click: impl FnMut(&mut E)) -> impl Widget<E> {
    Button {
        widget: drawables::text(text),
        on_click,
    }
}

pub struct ButtonState<S> {
    pub variant: WidgetVariant,
    pub state: S,
    pub layout: Size,
}

pub struct Button<E, H> {
    pub widget: E,
    pub on_click: H,
}

// impl<'a, E: WidgetParams, H: Fn() + 'a> WidgetParams for ButtonElem<E, H> {
//     type Widget = Button<E::Widget>;
// }

impl<E, D: Widget<E>, H: FnMut(&mut E)> Widget<E> for Button<D, H> {
    // type Params = Button<D::Params, H>;
    type State = ButtonState<D::State>;

    fn build(
        &mut self,
        env: &mut E,
        constraint: LayoutConstraint,
        ctx: &mut LayoutCtx,
    ) -> Self::State {
        // let rect_theme = theme.rect.get(self.variant, true);
        let mut state = self.widget.build(
            env,
            LayoutConstraint {
                x: constraint.x.map(|w| 0f64.max(w - PADDING)),
                y: constraint.y.map(|h| 0f64.max(h - PADDING)),
            },
            ctx,
        );

        let layout = self.widget.min_size(&state) + Size::new(PADDING * 2., PADDING * 2.);

        ButtonState {
            variant: WidgetVariant::Normal,
            state,
            layout,
        }
    }

    fn update(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        constraint: LayoutConstraint,
        ctx: &mut LayoutCtx,
    ) {
        self.widget.update(
            &mut state.state,
            env,
            LayoutConstraint {
                x: constraint.x.map(|w| 0f64.max(w - PADDING)),
                y: constraint.y.map(|h| 0f64.max(h - PADDING)),
            },
            ctx,
        );

        // TODO: These fields are kinda redundant.
        state.layout = self.widget.min_size(&mut state.state) + Size::new(PADDING * 2., PADDING * 2.);
        // self.on_click = self.on_click;
    }

    fn min_size(&self, state: &Self::State) -> Size {
        state.layout
    }

    fn extra_layers(&self, state: &Self::State) -> u8 {
        0
    }

    fn render(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        rect: Rect,
        layer: u8,
        focus: bool,
        ctx: &mut RenderCtx,
    ) {
        // self.widget.render(
        //     &mut state.state,
        //     rect.inset(-PADDING),
        //     renderer,
        //     theme,
        //     input_state,
        //     layer,
        //     focus,
        // );
        render_rect(
            false,
            true,
            &mut self.widget,
            &mut state.state,
            env,
            rect,
            layer,
            focus,
            ctx,
        );
    }

    fn handle_cursor_input(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        rect: Rect,
        cursor_pos: Point,
        _cursor_layer: u8,
        input: CursorInput,
        _input_state: &InputState,
        _theme: &Theme,
        _: bool,
    ) -> InputReturn {
        // let rect_theme = theme.rect.get(self.variant, true);

        match input {
            CursorInput::Up(..) => {
                if rect.contains(cursor_pos) {
                    (self.on_click)(env);
                }
            }
            _ => (),
        }
        Default::default()
    }
}

// #[derive(Copy, Clone, Debug, Default)]
// pub struct TextButtonLayout(pub ButtonLayout, pub TextWidgetLayout);
//
// pub fn text_button<'a, C: FnMut()>(
//     text: &'static str,
//     layout: &'a mut TextButtonLayout,
//     on_click: &'a mut C,
// ) -> Button<'a, TextWidget<'a>, C> {
//     Button {
//         variant: WidgetVariant::Normal,
//         drawable: TextWidget {
//             text,
//             variant: WidgetVariant::Normal,
//             layout: &mut layout.1,
//         },
//         layout: &mut layout.0,
//         on_click,
//     }
// }
//
// pub struct TextButtonWidget<H> {
//     text: &'static str,
//     layout: TextButtonLayout,
//     on_click: H,
// }
//
// impl<C: Debug, H: Fn(&mut C)> Widget for TextButtonWidget<H> {
//     type Event = ();
//
//     fn measure(
//         &mut self,
//         max_size: LayoutConstraint,
//         renderer: &mut Piet,
//         theme: &Theme,
//     ) -> Size {
//         let h = &mut || ();
//         let mut w = text_button(self.text, &mut self.layout, h);
//         <Button<_, _> as crate::widget::Drawable<C>>::measure(&mut w, max_size, renderer, theme)
//     }
//
//     fn min_size(&mut self) -> Size {
//         let h = &mut || ();
//         let mut w = text_button(self.text, &mut self.layout, h);
//         <Button<_, _> as crate::widget::Drawable<C>>::min_size(&mut w)
//     }
//
//     fn extra_layers(&mut self) -> u8 {
//         let h = &mut || ();
//         let mut w = text_button(self.text, &mut self.layout, h);
//         <Button<_, _> as crate::widget::Drawable<C>>::extra_layers(&mut w)
//     }
//
//     fn render(
//         &mut self,
//         rect: Rect,
//         renderer: &mut Piet,
//         theme: &Theme,
//         input_state: &InputState,
//         layer: u8,
//         focus: bool,
//     ) {
//         let h = &mut || ();
//         let mut w = text_button(self.text, &mut self.layout, h);
//         <Button<_, _> as Widget>::render(&mut w, rect, renderer, theme, input_state, layer, focus)
//     }
//
//     fn handle_cursor_input(
//         &mut self,
//         rect: Rect,
//         cursor_pos: Point,
//         cursor_layer: u8,
//         input: CursorInput,
//         input_state: &InputState,
//         theme: &Theme,
//         focus: bool,
//         context: &mut C,
//     ) -> InputReturn {
//         if rect.contains(cursor_pos) {
//             let on_click = &mut self.on_click;
//             let h = &mut || {
//                 on_click(context);
//             };
//             let mut w = text_button(self.text, &mut self.layout, h);
//             w.handle_cursor_input(rect, cursor_pos, cursor_layer, input, input_state, theme, focus, &mut ());
//         }
//
//         Default::default()
//     }
// }

// pub fn button_elem<'a, E: WidgetParams>(
//     element: E,
//     on_click: impl Fn() + 'a,
// ) -> impl WidgetParams {
//     ButtonElem { element, on_click }
// }

// pub fn text_button_elem<'a>(
//     text: &'static str,
//     on_click: impl Fn() + 'a,
// ) -> impl WidgetParams + 'a {
//     button_elem(super::drawables::text_elem(text), on_click)
// }

// pub fn checkbox_elem(checked: bool, on_click: impl Fn()) -> impl WidgetParams<'static> {
//     button_elem(
//         if checked {
//             // Sadly rust doesn't seem to be able to infer this.
//             OrElem/*::<_, _, NoneWidget, NoneWidget>*/::A(checkmark_elem(Size::new(12., 12.)))
//         } else {
//             OrElem::B(fixed_rect_elem(Size::new(12., 12.)))
//         },
//         on_click,
//     )
// }

// pub fn text_button_elem(
//     text: &'static str,
//     on_click: impl Fn(&mut C),
// ) -> impl WidgetParams {
//     pub struct TextButtonElement<H> {
//         text: &'static str,
//         on_click: H,
//     }

//     impl<C: Debug, H: Fn(&mut C)> WidgetParams for TextButtonElement<H> {
//         type Widget = TextButtonWidget<H>;

//         fn build(self) -> Self::Widget {
//             TextButtonWidget {
//                 text: self.text,
//                 layout: Default::default(),
//                 on_click: self.on_click,
//             }
//         }

//         fn update(self, widget: &mut Self::Widget) {
//             widget.text = self.text;
//         }
//     }

//     TextButtonElement {
//         text,
//         on_click,
//     }
// }

// pub struct Checkbox<'a, C: FnMut()> {
//     pub checked: bool,
//     pub layout: &'a mut ButtonLayout,
//     pub on_click: &'a mut C,
// }

// impl<'a, C: Debug, H: FnMut()> Component<C> for Checkbox<'a, H> {
//     type Event = ();

//     fn component<I: WidgetHandler<C>, E>(&mut self, handler: I, _: E) {
//         if self.checked {
//             handler.widget(&mut Button {
//                 variant: WidgetVariant::Normal,
//                 drawable: Checkmark(Size::new(16., 16.)),
//                 layout: self.layout,
//                 on_click: self.on_click,
//             });
//         } else {
//             handler.widget(&mut Button {
//                 variant: WidgetVariant::Normal,
//                 drawable: FixedRect(Size::new(16., 16.)),
//                 layout: self.layout,
//                 on_click: self.on_click,
//             });
//         }
//     }
// }
