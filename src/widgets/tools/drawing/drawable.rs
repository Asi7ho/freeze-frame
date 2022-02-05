use iced::{
    canvas::{self, event, Cursor, Frame, Path, Stroke},
    mouse, Color, Point, Rectangle, Size,
};

use crate::widgets::canvas::CanvasState;

use super::Strokes;

pub struct Drawable<'a> {
    pub state: &'a mut CanvasState,
    pub strokes: &'a [Strokes],
}

impl<'a> canvas::Program<Strokes> for Drawable<'a> {
    fn update(
        &mut self,
        event: canvas::Event,
        bounds: iced::Rectangle,
        cursor: canvas::Cursor,
    ) -> (event::Status, Option<Strokes>) {
        let cursor_position = if let Some(position) = cursor.position_in(&bounds) {
            position
        } else {
            return (event::Status::Ignored, None);
        };

        match event {
            _ => (event::Status::Ignored, None),
        }
    }

    fn mouse_interaction(&self, bounds: Rectangle, cursor: Cursor) -> mouse::Interaction {
        if cursor.is_over(&bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(&self, bounds: iced::Rectangle, cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
        let mut contents = Vec::new();

        // Background canvas
        let mut frame = Frame::new(bounds.size());
        let background_canvas = Path::rectangle(Point::ORIGIN, bounds.size());
        frame.fill(&background_canvas, Color::from_rgb8(34, 34, 34));
        contents.push(frame.into_geometry());

        // Foreground canvas
        let mut frame = Frame::new(bounds.size());
        let foreground_canvas = Path::rectangle(
            Point {
                x: frame.center().x - self.state.canvas_width / 2.0,
                y: frame.center().y - self.state.canvas_height / 2.0,
            },
            Size {
                width: self.state.canvas_width,
                height: self.state.canvas_height,
            },
        );
        frame.fill(&foreground_canvas, Color::WHITE);
        contents.push(frame.into_geometry());

        // let content = self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
        //     Strokes::draw_all(self.strokes, frame);

        //     frame.stroke(
        //         &Path::rectangle(Point::ORIGIN, frame.size()),
        //         Stroke::default(),
        //     );
        // });

        contents
    }
}
