use iced::{
    widget::{canvas, Canvas, Container},
    Element, Length,
};

use crate::tools::{
    drawing::{BrushComponent, Painting, Strokes},
    filters::UiControlFilter,
};

#[derive(Debug, Default)]
pub struct CanvasState {
    pub cache: canvas::Cache,
    pub brush_component: BrushComponent,
    pub ui_control_filter: UiControlFilter,
}

impl CanvasState {
    pub fn view<'a>(&'a self, strokes: &'a [Strokes]) -> Element<'a, Strokes> {
        let canvas = Canvas::new(Painting {
            state: self,
            strokes,
        })
        .width(Length::Fixed(750.0))
        .height(Length::Fixed(435.0));

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
