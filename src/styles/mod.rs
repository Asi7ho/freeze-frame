use iced::{theme, Color};

pub mod button;
pub mod panel;
pub mod slider;
pub mod text_input;

pub use button::{ColorButtonStyle, IconStyle};
pub use panel::{HeaderStyle, RightBarStyle, TimeLineStyle};
pub use slider::SliderStyle;
pub use text_input::SceneTitleStyle;

// Default background
pub const DEFAULT_PALETTE: theme::Palette = theme::Palette {
    background: Color::from_rgb(34 as f32 / 255.0, 34 as f32 / 255.0, 34 as f32 / 255.0),
    text: Color::WHITE,
    primary: Color::from_rgba(
        187 as f32 / 255.0,
        182 as f32 / 255.0,
        197 as f32 / 255.0,
        0.15,
    ),
    success: Color::from_rgba(
        187 as f32 / 255.0,
        182 as f32 / 255.0,
        197 as f32 / 255.0,
        0.15,
    ),
    danger: Color::from_rgba(
        187 as f32 / 255.0,
        182 as f32 / 255.0,
        197 as f32 / 255.0,
        0.15,
    ),
};
