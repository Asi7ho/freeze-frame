use iced::{canvas, Canvas, Element, Length};

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
