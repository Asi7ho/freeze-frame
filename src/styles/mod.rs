use iced::{theme, Color};

pub mod button;
pub mod panel;
pub mod slider;
pub mod text_input;

pub use button::{ColorButtonStyle, IconStyle};
pub use panel::{HeaderStyle, RightBarStyle, TimeLineStyle};
pub use slider::SliderStyle;
pub use text_input::SceneTitleStyle;

macro_rules! color {
    ($red:expr, $green:expr, $blue:expr) => {
        Color::from_rgba(
            $red as f32 / 255.0,
            $green as f32 / 255.0,
            $blue as f32 / 255.0,
            0.0,
        )
    };
    ($red:expr, $green:expr, $blue:expr, $opacity:expr) => {
        Color::from_rgba(
            $red as f32 / 255.0,
            $green as f32 / 255.0,
            $blue as f32 / 255.0,
            $opacity as f32 / 255.0,
        )
    };
}

// Default background
pub const DEFAULT_PALETTE: theme::Palette = theme::Palette {
    background: BACKGROUND,
    text: TEXT_COLOR,
    primary: BUTTON_BACKGROUND,
    success: BUTTON_BACKGROUND,
    danger: BUTTON_BACKGROUND,
};

// General colors
const TEXT_COLOR: Color = Color::WHITE;
// const SVG: Color = color!(187, 182, 193);
const BACKGROUND: Color = color!(34, 34, 34);
const DARK_BACKGROUND: Color = color!(25, 25, 25);
const TRANSPARENT: Color = Color::TRANSPARENT;
const SELECTION: Color = color!(64, 64, 64);

// Button colors
const BUTTON_BACKGROUND: Color = color!(187, 182, 197, 0.15);
const BUTTON_BORDER: Color = color!(187, 182, 197, 0.35);
const BORDER_RADIUS: f32 = 10.0;

// Rails colors
const RAILS: (Color, Color) = (color!(187, 182, 197), color!(187, 182, 197));
const HANDLE: Color = color!(187, 182, 197);
