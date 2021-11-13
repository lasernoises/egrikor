/// The first byte is red, the second is green, the third is blue and the fourth is alpha.
/// These are easier to define like `0xFF7700FF` than `[u8; 4]`.
pub type Color = u32;

/// From some analysis of gtk3-widget-factory.
/// A list of widgets with mostly unique theme.
pub enum WidgetType {
    Button,
    Dropdown,
    Tab,
    ToggleButton,
    Checkbox,
    Radiobutton,
    ProgressBar,
}

#[derive(Copy, Clone, Debug)]
pub struct Theme {
    pub rect: WidgetTheme<RectTheme>,

    // for textboxes and stuff
    pub rect_outline: WidgetTheme<RectTheme>,

    pub text: WidgetTheme<TextTheme>,
}

#[derive(Copy, Clone, Debug)]
pub struct TextTheme {
    pub font: &'static str,
    pub size: u16,
}

#[derive(Copy, Clone, Debug)]
pub struct WidgetVariants<T> {
    pub normal: T,
    pub active: T,
    pub danger: T,
}

// pub struct WidgetState {
//     pub variant: WidgetVariant,
//     pub disabled: bool,
// }

#[derive(Copy, Clone, Debug)]
pub struct WidgetTheme<T> {
    pub enabled: WidgetVariants<T>,
    pub disabled: WidgetVariants<T>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum WidgetVariant {
    Normal,
    Active,
    Danger,
}

impl<T: Copy> WidgetTheme<T> {
    pub fn get(&self, variant: WidgetVariant, enabled: bool) -> T {
        let variant_themes = if enabled {
            &self.enabled
        } else {
            &self.disabled
        };

        match variant {
            WidgetVariant::Normal => variant_themes.normal,
            WidgetVariant::Active => variant_themes.active,
            WidgetVariant::Danger => variant_themes.danger,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BorderTheme {
    pub color: Color,
    pub radius: u64,
    pub padding: u64,
    pub margin: u64,
}

#[derive(Copy, Clone, Debug)]
pub enum RectShape {
    Square,
    Round(u64),
}

#[derive(Copy, Clone, Debug)]
pub struct RectTheme {
    pub background_color: (Color, Color),
    pub foreground_color: (Color, Color),
    // pub shape: RectShape,
    pub border_color: (Color, Color),
    pub border_width: u64,
    pub padding: u64,

    // not sure about this
    pub margin: u64,
}

impl RectTheme {
    pub fn full_border_width(&self) -> f64 {
        (self.padding + self.margin) as f64 + self.border_width as f64 / 2.0
    }

    pub fn point_in_rect(&self, pos: [f64; 2], size: [f64; 2], point: [f64; 2]) -> bool {
        let margin = self.margin as f64;

        point[0] >= pos[0] + margin
            && point[0] <= pos[0] + size[0] - margin
            && point[1] >= pos[1] + margin
            && point[1] <= pos[1] + size[1] - margin
    }
}

// #[derive(Clone, Debug)]
// pub struct Theme<F> {
//     pub default_rect_theme: RectTheme,
//     pub rect_themes: Vec<Vec<RectTheme>>,
//     pub default_text_theme: F,
//     pub text_themes: Vec<Vec<F>>,
// }

pub trait ThemeProvider {
    fn text_theme(&self, variant: WidgetVariant, enabled: bool) -> TextTheme;
}
