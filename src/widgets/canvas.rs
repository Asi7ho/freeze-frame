use iced::{
    pure::{
        widget::{canvas, Canvas},
        Element,
    },
    Color, Length,
};

use super::header::{BrushFilter, GridFilter};
pub use super::tools::drawing::{Drawable, Strokes};

#[derive(Debug, Clone)]
pub enum CanvasMessage {
    AddStrokes(Strokes),
    Clear,
}

#[derive(Debug, Default)]
pub struct CanvasState {
    pub canvas_width: f32,
    pub canvas_height: f32,
    pub cache: canvas::Cache,
    pub brush_color: Color,
    pub brush_size: f32,
    pub brush_filter: BrushFilter,
    pub grid_filer: GridFilter,
}

impl CanvasState {
    pub fn view<'a>(&'a self, strokes: &'a [Strokes]) -> Element<'a, Strokes> {
        Canvas::new(Drawable {
            state: self,
            strokes,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear()
    }
}
