use iced::{
    canvas::{self, event, Cursor, Event, Frame, Path},
    mouse, Color, Point, Rectangle, Size,
};

use crate::widgets::canvas::{CanvasState, Pending};

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
            match event {
                Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        self.state.is_drawing = false;
                        match self.state.pending {
                            Pending::StrokePending { stroke: _ } => {
                                self.state.pending = Pending::StrokePending {
                                    stroke: Strokes {
                                        from: None,
                                        to: None,
                                        color: None,
                                    },
                                };
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            };
            return (event::Status::Ignored, None);
        };

        match event {
            Event::Mouse(mouse_event) => {
                let message = match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        self.state.is_drawing = true;
                        match self.state.pending {
                            Pending::StrokePending { mut stroke } => {
                                let stroke_to_send = match (stroke.from, stroke.to) {
                                    (None, None) => {
                                        stroke.from = Some(cursor_position);
                                        self.state.pending = Pending::StrokePending { stroke };
                                        None
                                    }
                                    _ => None,
                                };
                                stroke_to_send
                            }
                        }
                    }
                    mouse::Event::CursorMoved { position: _ } => {
                        if self.state.is_drawing {
                            match self.state.pending {
                                Pending::StrokePending { mut stroke } => {
                                    let stroke_to_send = match (stroke.from, stroke.to) {
                                        (None, None) => {
                                            stroke.from = Some(cursor_position);
                                            self.state.pending = Pending::StrokePending { stroke };
                                            None
                                        }
                                        (Some(_), None) => {
                                            stroke.to = Some(cursor_position);
                                            stroke.color = Some(self.state.brush_color);
                                            let send = Some(stroke);

                                            stroke.from = stroke.to;
                                            stroke.to = None;
                                            stroke.color = None;
                                            self.state.pending = Pending::StrokePending { stroke };

                                            send
                                        }
                                        _ => None,
                                    };
                                    stroke_to_send
                                }
                            }
                        } else {
                            None
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        self.state.is_drawing = false;
                        match self.state.pending {
                            Pending::StrokePending { stroke: _ } => {
                                self.state.pending = Pending::StrokePending {
                                    stroke: Strokes {
                                        from: None,
                                        to: None,
                                        color: None,
                                    },
                                };
                                None
                            }
                        }
                    }
                    _ => None,
                };
                (event::Status::Captured, message)
            }
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

    fn draw(&self, bounds: iced::Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
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

        let strokes_content = self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
            Strokes::draw_all(self.strokes, frame);
        });
        // let strokes_content = Strokes::draw_all(self.strokes, bounds);
        contents.extend(vec![strokes_content]);

        contents
    }
}
