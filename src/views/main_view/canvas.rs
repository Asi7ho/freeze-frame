use iced::{
    pure::{
        widget::{canvas, Canvas, Container},
        Element,
    },
    Length,
};

use crate::tools::{
    drawing::{BrushComponent, Drawable, Strokes},
    filters::{GeometryForm, UiControlFilter},
};

#[derive(Debug, Default)]
pub struct CanvasState {
    pub canvas_width: f32,
    pub canvas_height: f32,
    pub cache: canvas::Cache,
    pub brush_component: BrushComponent,
    pub geometry_form: Option<GeometryForm>,
    pub extra_filter: UiControlFilter,
}

impl CanvasState {
    pub fn view<'a>(&'a self, strokes: &'a [Strokes]) -> Element<'a, Strokes> {
        let canvas = Canvas::new(Drawable {
            state: self,
            strokes,
        })
        .width(Length::Units(self.canvas_width as u16))
        .height(Length::Units(self.canvas_height as u16));

        let container = Container::new(canvas)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y();

        container.into()
    }

    pub fn request_redraw(&mut self) {
        log::info!("Clear canvas -> Canvas = {:?}", self);
        self.cache.clear()
    }
}
