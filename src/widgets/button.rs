use std::fmt::Debug;

use druid_shell::kurbo::{Point, Rect, Size};
use piet_common::Piet;

use super::drawables::{checkmark_elem, fixed_rect_elem};
use super::or::OrElem;
use super::*;
use crate::*;

pub struct Button<D> {
    pub variant: WidgetVariant,
    pub drawable: D,
    pub layout: Size,
}

#[derive(Copy, Clone, Debug)]
pub enum ButtonEvent {
    Click,
}
pub struct ButtonElem<E, H> {
    element: E,
    on_click: H,
}

impl<'a, E: WidgetParams, H: Fn() + 'a> WidgetParams for ButtonElem<E, H> {
    type Widget = Button<E::Widget>;
}

impl<D: Widget, H: FnMut()> Widget for Button<D> {
    type Params = ButtonElem<D::Params, H>;
    type Event = ButtonEvent;

    fn build(
        params: &mut Self::Params,
        max_size: [Option<f64>; 2],
        renderer: &mut Piet,
        theme: &Theme,
    ) -> Self {
        // let rect_theme = theme.rect.get(self.variant, true);
        let mut drawable = params.element.build(
            max_size,
            renderer,
            theme,
        );

        let layout = measure_rect(&mut drawable, max_size, renderer, theme);

        Button {
            variant: WidgetVariant::Normal,
            drawable,
            layout,
        }
    }

    fn update(
        &mut self,
        params: &mut Self::Params,
        max_size: [Option<f64>; 2],
        renderer: &mut Piet,
        theme: &Theme,
    ) {
        self.drawable.update(
            params.element,
            max_size,
            renderer,
            theme,
        );

        // TODO: These fields are kinda redundant.
        self.layout = measure_rect(&mut self.drawable, max_size, renderer, theme);
        self.on_click = self.on_click;
    }

    fn min_size(&self) -> Size {
        self.layout
    }

    fn extra_layers(&self) -> u8 {
        0
    }

    fn render(
        &mut self,
        rect: Rect,
        renderer: &mut Piet,
        theme: &Theme,
        input_state: &InputState,
        layer: u8,
        focus: bool,
    ) {
        render_rect(
            false,
            true,
            &mut self.drawable,
            rect,
            renderer,
            theme,
            input_state,
            layer,
            focus,
        );
    }

    fn handle_cursor_input(
        &mut self,
        rect: Rect,
        cursor_pos: Point,
        _cursor_layer: u8,
        input: CursorInput,
        _input_state: &InputState,
        _theme: &Theme,
        _: bool,
    ) -> (InputReturn, Option<ButtonEvent>) {
        // let rect_theme = theme.rect.get(self.variant, true);

        (
            Default::default(),
            match input {
                CursorInput::Up(..) => {
                    if rect.contains(cursor_pos) {
                        (self.on_click)();
                        Some(ButtonEvent::Click)
                    } else {
                        None
                    }
                }
                _ => None,
            },
        )
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
//         max_size: [Option<f64>; 2],
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
//     ) -> (InputReturn, Option<Self::Event>) {
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

pub fn button_elem<'a, E: WidgetParams>(
    element: E,
    on_click: impl Fn() + 'a,
) -> impl WidgetParams {
    ButtonElem { element, on_click }
}

pub fn text_button_elem<'a>(
    text: &'static str,
    on_click: impl Fn() + 'a,
) -> impl WidgetParams + 'a {
    button_elem(super::drawables::text_elem(text), on_click)
}

pub fn checkbox_elem(checked: bool, on_click: impl Fn()) -> impl WidgetParams<'static> {
    button_elem(
        if checked {
            // Sadly rust doesn't seem to be able to infer this.
            OrElem/*::<_, _, NoneWidget, NoneWidget>*/::A(checkmark_elem(Size::new(12., 12.)))
        } else {
            OrElem::B(fixed_rect_elem(Size::new(12., 12.)))
        },
        on_click,
    )
}

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
