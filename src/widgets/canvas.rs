use iced::{canvas, Canvas, Color, Element, Length, Point};

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
    pub pending: Pending,
    pub is_drawing: bool,
    pub brush_color: Color,
    pub brush_size: f32,
    pub brush_filter: BrushFilter,
    pub grid_filer: GridFilter,
}

impl CanvasState {
    pub fn view<'a>(&'a mut self, strokes: &'a [Strokes]) -> Element<'a, Strokes> {
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

#[derive(Debug, Clone, Copy)]
pub enum Pending {
    Freehand {
        from: Option<Point>,
        to: Option<Point>,
    },
}

impl Default for Pending {
    fn default() -> Self {
        Pending::Freehand {
            from: None,
            to: None,
        }
    }
}
