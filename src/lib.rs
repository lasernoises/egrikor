#![feature(type_alias_impl_trait)]

use std::fmt::Debug;

use druid_shell::{
    kurbo::{Point, Rect, Size},
    Application, KeyEvent, Modifiers, MouseEvent, Region, WinHandler, WindowBuilder, WindowHandle,
};
use piet_common::{Color, Piet, RenderContext, PietText};
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

// type LayoutConstraint = LayoutConstraint;

#[derive(Copy, Clone, Debug)]
pub struct LayoutConstraint {
    pub x: Option<f64>,
    pub y: Option<f64>,
}

impl LayoutConstraint {
    pub const fn new(x: Option<f64>, y: Option<f64>) -> Self {
        Self { x, y }
    }
}

#[non_exhaustive]
pub struct LayoutCtx<'a> {
    pub text: &'a mut PietText,
    pub theme: &'a Theme,
}

#[non_exhaustive]
pub struct RenderCtx<'a, 'b> {
    pub piet: &'a mut Piet<'b>,
    pub theme: &'a Theme,
    pub input_state: &'a InputState,
}

pub trait WidgetState {
    fn new() -> Self;

    fn min_size(&self) -> Size;

    /// These are the extra layers (there is always one at least from the perspective of a widget).
    /// Layers are relative.
    /// A widget in layer 1 will not know that there's stuff below.
    /// Containers must return the max of their children here.
    fn extra_layers(&self) -> u8 {
        0
    }
}

pub trait Widget<E> {
    type State: WidgetState;

    fn layout(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        constraint: LayoutConstraint,
        ctx: &mut LayoutCtx,
    );

    /// Single-layer widgets can just ignore the `layer` parameter since `render` they should only
    /// be called for layers a widget actually has.
    fn render(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        rect: Rect,
        layer: u8,
        focus: bool,
        ctx: &mut RenderCtx,
    ) {
    }

    /// Calculate the layer on which an input at a given position needs to be handled. The layer is
    /// relative to the layer the current widget is on. `None` means the input doesn't hit the
    /// widget.
    fn test_input_pos_layer(
        &mut self,
        _state: &mut Self::State,
        env: &mut E,
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
        env: &mut E,
        rect: Rect,
        cursor_pos: Point,
        cursor_layer: u8,
        input: CursorInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
    ) -> InputReturn {
        Default::default()
    }

    /// Containers should only pass on `handle_input` calls to their children with the most
    /// extra layers, this behaves like dropdowns in x11.
    fn handle_keyboard_input(
        &mut self,
        state: &mut Self::State,
        env: &mut E,
        rect: Rect,
        input: &KeyboardInput,
        input_state: &InputState,
        theme: &Theme,
        focus: bool,
    ) {}
}

pub struct WindowHandler<W: Widget<Runtime>> {
    widget: W,
    state: W::State,
    input_state: InputState,
    size: Size,
    handle: Option<WindowHandle>,
    theme: Theme,
}

impl<W: Widget<Runtime>> WindowHandler<W> {
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

impl<W: Widget<Runtime>> WinHandler for WindowHandler<W>
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
        let mut ctx = LayoutCtx {
            text: piet.text(),
            theme: &self.theme,
        };

        let state = &mut self.state;

        self.widget.layout(
            state,
            &mut Runtime {},
            LayoutConstraint::new(Some(self.size.width), Some(self.size.height)),
            &mut ctx,
        );

        piet.clear(Color::Rgba32(0x00_00_00_FF));

        for i in 0..1 + state.extra_layers() {
            self.widget.render(
                state,
                &mut Runtime {},
                Rect::from_origin_size((0.0, 0.0), self.size),
                i,
                true,
                &mut RenderCtx {
                    piet,
                    theme: &self.theme,
                    input_state: &self.input_state,
                },
            );
        }
    }

    fn size(&mut self, size: Size) {
        self.size = size;
    }

    fn mouse_move(&mut self, event: &MouseEvent) {
        let state = &mut self.state;

        self.input_state.cursor_pos = Some(event.pos);
        self.input_state.mods = event.mods;
        let rect = Rect::from_origin_size((0., 0.), self.size);

        let layer = self.widget.test_input_pos_layer(state, &mut Runtime {}, rect, event.pos);

        if let Some(layer) = layer {
            self.widget.handle_cursor_input(
                state,
                &mut Runtime {},
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

    fn mouse_leave(&mut self) {
        self.input_state.cursor_pos = None;
        self.handle.as_ref().unwrap().request_anim_frame();
    }

    fn mouse_down(&mut self, event: &MouseEvent) {
        let state = &mut self.state;

        self.input_state.mouse_down = true;
        self.input_state.mods = event.mods;
        if let Some(button) = druid_shell_mouse_button_to_mouse_button(event.button) {
            let rect = Rect::from_origin_size((0., 0.), self.size);

            let layer = self.widget.test_input_pos_layer(state, &mut Runtime {}, rect, event.pos);

            if let Some(layer) = layer {
                self.widget.handle_cursor_input(
                    state,
                    &mut Runtime {},
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

    fn mouse_up(&mut self, event: &MouseEvent) {
        let state = &mut self.state;

        self.input_state.mouse_down = false;
        self.input_state.mods = event.mods;
        if let Some(button) = druid_shell_mouse_button_to_mouse_button(event.button) {
            let rect = Rect::from_origin_size((0., 0.), self.size);

            let layer = self.widget.test_input_pos_layer(state, &mut Runtime {}, rect, event.pos);

            if let Some(layer) = layer {
                self.widget.handle_cursor_input(
                    state,
                    &mut Runtime {},
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

    fn key_down(&mut self, event: KeyEvent) -> bool {
        let state = &mut self.state;

        self.input_state.mods = event.mods;
        self.widget.handle_keyboard_input(
            state,
            &mut Runtime {},
            Rect::from_origin_size((0., 0.), self.size),
            &KeyboardInput::KeyDown(event),
            &self.input_state,
            &self.theme,
            true,
        );

        self.handle.as_ref().unwrap().request_anim_frame();

        true
    }

    fn as_any(&mut self) -> &mut dyn core::any::Any {
        self
    }

    fn request_close(&mut self) {
        self.handle.as_ref().unwrap().close();
    }
}

#[non_exhaustive]
pub struct Runtime {}

pub fn run<W: Widget<Runtime> + 'static>(
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
        state: W::State::new(),
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
