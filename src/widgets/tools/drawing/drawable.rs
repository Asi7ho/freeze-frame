use iced::{
    canvas::{self, event, Cursor, Event, Frame, Path},
    mouse, Color, Point, Rectangle, Size,
};

use crate::widgets::canvas::{CanvasState, Pending};
use crate::widgets::header::BrushFilter;

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
                        self.state.pending = Pending::default();
                    }
                    _ => (),
                },
                _ => (),
            };
            return (event::Status::Ignored, None);
        };

        match self.state.brush_filter {
            BrushFilter::Brush | BrushFilter::Eraser => match event {
                Event::Mouse(mouse_event) => {
                    let message = match mouse_event {
                        mouse::Event::ButtonPressed(mouse::Button::Left) => {
                            self.state.is_drawing = true;
                            match self.state.pending {
                                Pending::Freehand { mut from, to } => match (from, to) {
                                    (None, None) => {
                                        from = Some(cursor_position);
                                        self.state.pending = Pending::Freehand { from, to };
                                        None
                                    }
                                    _ => None,
                                },
                            }
                        }
                        mouse::Event::CursorMoved { position: _ } => {
                            if self.state.is_drawing {
                                match self.state.pending {
                                    Pending::Freehand { mut from, mut to } => match (from, to) {
                                        (None, None) => {
                                            from = Some(cursor_position);
                                            self.state.pending = Pending::Freehand { from, to };
                                            None
                                        }
                                        (Some(_), None) => {
                                            to = Some(cursor_position);

                                            let message = Some(Strokes {
                                                brush: self.state.brush_filter,
                                                from,
                                                to,
                                                color: self.state.brush_color,
                                                size: self.state.brush_size,
                                            });

                                            from = to;
                                            to = None;
                                            self.state.pending = Pending::Freehand { from, to };

                                            message
                                        }
                                        _ => None,
                                    },
                                }
                            } else {
                                None
                            }
                        }
                        mouse::Event::ButtonReleased(mouse::Button::Left) => {
                            self.state.is_drawing = false;
                            self.state.pending = Pending::default();
                            None
                        }
                        _ => None,
                    };
                    (event::Status::Captured, message)
                }
                _ => (event::Status::Ignored, None),
            },
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
        let top_left = Point {
            x: frame.center().x - self.state.canvas_width / 2.0,
            y: frame.center().y - self.state.canvas_height / 2.0,
        };
        let size = Size {
            width: self.state.canvas_width,
            height: self.state.canvas_height,
        };
        let foreground_canvas = Path::rectangle(top_left, size);
        frame.fill(&foreground_canvas, Color::WHITE);
        contents.push(frame.into_geometry());

        let strokes_ = cut_strokes(self.strokes, top_left, size);

        let strokes_content = self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
            Strokes::draw_all(&strokes_, frame);
        });
        // let strokes_content = Strokes::draw_all(self.strokes, bounds);
        contents.extend(vec![strokes_content]);

        contents
    }
}

fn cut_strokes(strokes: &[Strokes], top_left: Point, size: Size) -> Vec<Strokes> {
    let rectangle = iced::Rectangle::new(top_left, size);
    let mut new_strokes = Vec::new();
    for stroke in strokes {
        if stroke.from.is_some() && stroke.to.is_some() {
            let from = stroke.from.unwrap();
            let to = stroke.to.unwrap();
            if rectangle.contains(from) && !rectangle.contains(to) {
                let mut stroke_inner = stroke.clone();
                let mut stroke_outer = stroke.clone();
                let contact_point = compute_contact_point(from, to, top_left, size);
                stroke_inner.to = Some(contact_point);
                stroke_outer.from = Some(contact_point);
                if stroke.brush == BrushFilter::Eraser {
                    stroke_inner.color = Color::WHITE;
                    stroke_outer.color = Color::from_rgb8(34, 34, 34);
                }
                new_strokes.push(stroke_inner);
                new_strokes.push(stroke_outer);
            } else if !rectangle.contains(from) && rectangle.contains(to) {
                let mut stroke_inner = stroke.clone();
                let mut stroke_outer = stroke.clone();
                let contact_point = compute_contact_point(to, from, top_left, size);

                stroke_inner.from = Some(contact_point);
                stroke_outer.to = Some(contact_point);
                if stroke.brush == BrushFilter::Eraser {
                    stroke_inner.color = Color::WHITE;
                    stroke_outer.color = Color::from_rgb8(34, 34, 34);
                }
                new_strokes.push(stroke_inner);
                new_strokes.push(stroke_outer);
            } else {
                let mut stroke_ = stroke.clone();
                let from = stroke_.from.unwrap();
                let to = stroke_.to.unwrap();
                if rectangle.contains(from) && rectangle.contains(to) {
                    if stroke.brush == BrushFilter::Eraser {
                        stroke_.color = Color::WHITE;
                    }
                } else {
                    if stroke.brush == BrushFilter::Eraser {
                        stroke_.color = Color::from_rgb8(34, 34, 34);
                    }
                }
                new_strokes.push(stroke_);
            }
        }
    }
    return new_strokes;
}

fn compute_contact_point(from: Point, to: Point, top_left: Point, size: Size) -> Point {
    let mut contact_point = Point { x: 0.0, y: 0.0 };
    if top_left.x <= from.x && top_left.x > to.x {
        contact_point = Point {
            x: top_left.x,
            y: (to.y - from.y) * top_left.x / (to.x - from.x) + from.y
                - (to.y - from.y) * from.x / (to.x - from.x),
        };
    } else if from.x <= top_left.x + size.width && to.x > top_left.x + size.width {
        contact_point = Point {
            x: top_left.x + size.width,
            y: (to.y - from.y) * (top_left.x + size.width) / (to.x - from.x) + from.y
                - (to.y - from.y) * from.x / (to.x - from.x),
        };
    } else if top_left.y <= from.y && top_left.y > to.y {
        contact_point = Point {
            x: (to.x - from.x) * top_left.y / (to.y - from.y) + from.x
                - (to.x - from.x) * from.y / (to.y - from.y),
            y: top_left.y,
        };
    } else if from.y <= top_left.y + size.height && to.y > top_left.y + size.height {
        contact_point = Point {
            x: (to.x - from.x) * (top_left.y + size.height) / (to.y - from.y) + from.x
                - (to.x - from.x) * from.y / (to.y - from.y),
            y: top_left.y + size.height,
        };
    }
    return contact_point;
}
