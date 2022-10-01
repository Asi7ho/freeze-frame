use iced::{
    pure::{
        widget::{canvas, Canvas, Container},
        Element,
    },
    Length,
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
        .width(Length::Units(750.0 as u16))
        .height(Length::Units(435.0 as u16));

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
