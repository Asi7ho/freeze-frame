use iced::Color;

use crate::tools::filters::BrushFilter;

pub mod painting;
pub mod strokes;

pub use painting::Painting;
pub use strokes::Strokes;

use super::filters::GeometryForm;

// Brush
#[derive(Debug, Copy, Clone, Default)]
pub struct BrushComponent {
    pub brush: BrushFilter,
    pub geometry_form: Option<GeometryForm>,
    pub size: f32,
    pub color: Color,
}
