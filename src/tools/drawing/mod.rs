use iced::Color;

use crate::tools::filters::BrushFilter;

pub mod drawable;
pub mod strokes;

pub use drawable::Drawable;
pub use strokes::Strokes;

// Brush
#[derive(Debug, Copy, Clone, Default)]
pub struct BrushComponent {
    pub brush: BrushFilter,
    pub size: f32,
    pub color: Color,
}
