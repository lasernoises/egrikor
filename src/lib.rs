#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use std::fmt::Debug;

use druid_shell::{
    kurbo::{Point, Rect, Size},
    Application, KeyEvent, Modifiers, MouseEvent, Region, WinHandler, WindowBuilder, WindowHandle,
};
use piet_common::{Color, Piet, RenderContext};
use widgets::{RectTheme, TextTheme, Theme, WidgetTheme, WidgetVariants};

pub mod text;
pub mod theme;
pub mod widgets;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CursorInput {
    Down(MouseButton),
    Up(MouseButton),
    Move,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyboardInput {
    KeyDown(druid_shell::KeyEvent),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Primary,
    Secondary,
    Middle,
    X1,
    X2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MouseButtons {
    pub primary: bool,
    pub secondary: bool,
    pub middle: bool,
    pub x1: bool,
    pub x2: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct InputState {
    pub cursor_pos: Option<Point>,
    pub mouse_down: bool,
    pub mods: Modifiers,
    // here will be more things like the state of something like ctrl and shift buttons maybe
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            cursor_pos: None,
            mouse_down: false,
            mods: Default::default(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct InputReturn {
    pub demand_focus: bool,
}

impl Default for InputReturn {
    fn default() -> Self {
        InputReturn {
            demand_focus: false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Context {
    pub window_size: [f64; 2],
}

type LayoutConstraint = [Option<f64>; 2];

pub trait Widget {
    type State;
    type Event: std::fmt::Debug;

    fn build(
        &mut self,
        constraint: LayoutConstraint,
        renderer: &mut Piet,
        theme: &Theme,
    ) -> Self::State;

    fn update(
        &mut self,
        state: &mut Self::State,
        constraint: LayoutConstraint,
        renderer: &mut Piet,
        theme: &Theme,
    );

    // This will always be called after measure.
    fn min_size(&self, state: &Self::State) -> Size;

    /// These are the extra layers (there is always one at least from the perspective of a widget).
    /// Layers are relative a widget in layer 1 will not know that there's stuff below.
    /// Containers must return the max of their children here.
    fn extra_layers(&self, state: &Self::State) -> u8 {
        0
    }

    /// Single-layer widgets can just ignore the `layer` parameter since `render` they should only
    /// be called for layers a widget actually has.
    fn render(
        &mut self,
        _state: &mut Self::State,
        _rect: Rect,
        _renderer: &mut Piet,
        _theme: &Theme,
        _input_state: &InputState,
        _layer: u8,
        _focus: bool,
    ) {
    }

    /// Calculate the layer on which an input at a given position needs to be handled. The layer is
    /// relative to the layer the current widget is on. `None` means the input doesn't hit the
    /// widget.
    fn test_input_pos_layer(
        &mut self,
        _state: &mut Self::State,
        rect: Rect,
        input_pos: Point,
    ) -> Option<u8> {
        if rect.contains(input_pos) {
            Some(0)
        } else {
            None
        }
    }

    fn handle_cursor_input(
        &mut self,
        state: &mut Self::State,
        _rect: Rect,
        _cursor_pos: Point,
        _cursor_layer: u8,
        _input: CursorInput,
        _input_state: &InputState,
        _theme: &Theme,
        _focus: bool,
    ) -> (InputReturn, Option<Self::Event>) {
        Default::default()
    }

    /// Containers should only pass on `handle_input` calls to their children with the most
    /// extra layers, this behaves like dropdowns in x11.
    fn handle_keyboard_input(
        &mut self,
        state: &mut Self::State,
        _rect: Rect,
        _input: &KeyboardInput,
        _input_state: &InputState,
        _theme: &Theme,
        _focus: bool,
    ) -> Option<Self::Event> {
        None
    }
}

pub struct WindowHandler<W: Widget> {
    widget: W,
    state: Option<W::State>,
    input_state: InputState,
    size: Size,
    handle: Option<WindowHandle>,
    theme: Theme,
}

impl<W: Widget> WindowHandler<W> {
    fn rect(&self) -> Rect {
        Rect::from_origin_size((0., 0.), self.size)
    }
}

fn druid_shell_mouse_button_to_mouse_button(
    mouse_button: druid_shell::MouseButton,
) -> Option<MouseButton> {
    use druid_shell::MouseButton as Db;

    match mouse_button {
        Db::None => None,
        Db::Left => Some(MouseButton::Primary),
        Db::Right => Some(MouseButton::Secondary),
        Db::Middle => Some(MouseButton::Middle),
        Db::X1 => Some(MouseButton::X1),
        Db::X2 => Some(MouseButton::X2),
    }
}

impl<W: Widget> WinHandler for WindowHandler<W>
where
    Self: 'static,
{
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = Some(handle.clone());
    }

    fn prepare_paint(&mut self) {
        self.handle.as_ref().unwrap().invalidate();
    }

    fn paint(&mut self, piet: &mut Piet, _: &Region) {
        let state = if let Some(ref mut state) = self.state {
            self.widget.update(
                state,
                [Some(self.size.width), Some(self.size.height)],
                piet,
                &self.theme,
            );
            state
        } else {
            self.state = Some(self.widget.build(
                [Some(self.size.width), Some(self.size.height)],
                piet,
                &self.theme,
            ));

            // I can't think of a way to write this that doesn't require unwrap.
            // Maybe there could be a function that assigns you a `Some` and returns you a ref.
            self.state.as_mut().unwrap()
        };

        piet.clear(Color::Rgba32(0x00_00_00_FF));

        for i in 0..1 + self.widget.extra_layers(&state) {
            self.widget.render(
                state,
                Rect::from_origin_size((0.0, 0.0), self.size),
                piet,
                &self.theme,
                &self.input_state,
                i,
                true,
            );
        }
    }

    fn size(&mut self, size: Size) {
        self.size = size;
    }

    fn mouse_move(&mut self, event: &MouseEvent) {
        if let Some(ref mut state) = self.state {

            self.input_state.cursor_pos = Some(event.pos);
            self.input_state.mods = event.mods;
            let rect = Rect::from_origin_size((0., 0.), self.size);

            let layer = self.widget.test_input_pos_layer(state, rect, event.pos);

            if let Some(layer) = layer {
                self.widget.handle_cursor_input(
                    state,
                    rect,
                    event.pos,
                    layer,
                    CursorInput::Move,
                    &self.input_state,
                    &self.theme,
                    true,
                );
            }

            self.handle.as_ref().unwrap().request_anim_frame();
        }
    }

    fn mouse_leave(&mut self) {
        self.input_state.cursor_pos = None;
        self.handle.as_ref().unwrap().request_anim_frame();
    }

    fn mouse_down(&mut self, event: &MouseEvent) {
        if let Some(ref mut state) = self.state {
            self.input_state.mouse_down = true;
            self.input_state.mods = event.mods;
            if let Some(button) = druid_shell_mouse_button_to_mouse_button(event.button) {
                let rect = Rect::from_origin_size((0., 0.), self.size);

                let layer = self.widget.test_input_pos_layer(state, rect, event.pos);

                if let Some(layer) = layer {
                    self.widget.handle_cursor_input(
                        state,
                        rect,
                        event.pos,
                        layer,
                        CursorInput::Down(button),
                        &self.input_state,
                        &self.theme,
                        true,
                    );
                }
            }
            self.handle.as_ref().unwrap().request_anim_frame();
        }
    }

    fn mouse_up(&mut self, event: &MouseEvent) {
        if let Some(ref mut state) = self.state {
            self.input_state.mouse_down = false;
            self.input_state.mods = event.mods;
            if let Some(button) = druid_shell_mouse_button_to_mouse_button(event.button) {
                let rect = Rect::from_origin_size((0., 0.), self.size);

                let layer = self.widget.test_input_pos_layer(state, rect, event.pos);

                if let Some(layer) = layer {
                    self.widget.handle_cursor_input(
                        state,
                        rect,
                        event.pos,
                        layer,
                        CursorInput::Up(button),
                        &self.input_state,
                        &self.theme,
                        true,
                    );
                }
            }
            self.handle.as_ref().unwrap().request_anim_frame();
        }
    }

    fn key_down(&mut self, event: KeyEvent) -> bool {
        if let Some(ref mut state) = self.state {
            self.input_state.mods = event.mods;
            self.widget.handle_keyboard_input(
                state,
                Rect::from_origin_size((0., 0.), self.size),
                &KeyboardInput::KeyDown(event),
                &self.input_state,
                &self.theme,
                true,
            );

            self.handle.as_ref().unwrap().request_anim_frame();
        }

        true
    }

    fn as_any(&mut self) -> &mut dyn core::any::Any {
        self
    }

    fn request_close(&mut self) {
        self.handle.as_ref().unwrap().close();
    }
}

pub fn run<W: Widget + 'static>(
    title: &str,
    widget: W,
) {
    let app = Application::new().unwrap();
    let mut builder = WindowBuilder::new(app.clone());
    builder.set_title(title);

    let rect_theme = RectTheme {
        background_color: (0x00_77_FF_FF, 0x00_77_FF_FF),
        foreground_color: (0xFF_FF_FF_FF, 0xFF_FF_FF_FF),

        // shape: RectShape::Square,
        border_color: (0x00_00_00_FF, 0x00_00_00_FF),
        border_width: 1,
        padding: 16,
        margin: 4,
    };

    let text_theme = TextTheme {
        font: "Arial",
        size: 16,
    };

    builder.set_handler(Box::new(WindowHandler {
        widget,
        state: None,
        input_state: Default::default(),
        size: Size::new(0., 0.),
        handle: None,
        theme: Theme {
            rect: WidgetTheme {
                enabled: WidgetVariants {
                    normal: rect_theme,
                    active: RectTheme {
                        background_color: (0x33_AA_FF_FF, 0x33_AA_FF_FF),
                        ..rect_theme
                    },
                    danger: RectTheme {
                        background_color: (0xFF_77_00_FF, 0xFF_77_00_FF),
                        ..rect_theme
                    },
                },
                disabled: WidgetVariants {
                    normal: rect_theme,
                    active: rect_theme,
                    danger: rect_theme,
                },
            },
            rect_outline: WidgetTheme {
                enabled: WidgetVariants {
                    normal: rect_theme,
                    active: RectTheme {
                        background_color: (0x33_AA_FF_FF, 0x33_AA_FF_FF),
                        ..rect_theme
                    },
                    danger: RectTheme {
                        background_color: (0xFF_77_00_FF, 0xFF_77_00_FF),
                        ..rect_theme
                    },
                },
                disabled: WidgetVariants {
                    normal: rect_theme,
                    active: rect_theme,
                    danger: rect_theme,
                },
            },
            text: WidgetTheme {
                enabled: WidgetVariants {
                    normal: text_theme,
                    active: text_theme,
                    danger: text_theme,
                },
                disabled: WidgetVariants {
                    normal: text_theme,
                    active: text_theme,
                    danger: text_theme,
                },
            },
        },
    }));
    let window = builder.build().unwrap();
    window.show();
    app.run(None);
}
