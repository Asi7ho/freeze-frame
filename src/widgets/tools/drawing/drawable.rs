use iced::pure::widget::canvas::{
    self,
    event::{self, Event},
    Cursor, Frame, Path,
};
use iced::{mouse, Color, Point, Rectangle, Size};

use crate::widgets::canvas::CanvasState;
use crate::widgets::header::BrushFilter;

use super::Strokes;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interaction {
    None,
    Drawing {
        from: Option<Point>,
        to: Option<Point>,
    },
    Erasing {
        from: Option<Point>,
        to: Option<Point>,
    },
    Geometry {
        from: Option<Point>,
        to: Option<Point>,
    },
}

impl Default for Interaction {
    fn default() -> Self {
        Self::None
    }
}

pub struct Drawable<'a> {
    pub state: &'a CanvasState,
    pub strokes: &'a [Strokes],
}

impl<'a> canvas::Program<Strokes> for Drawable<'a> {
    type State = Interaction;

    fn update(
        &self,
        interaction: &mut Interaction,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Strokes>) {
        let cursor_position = if let Some(position) = cursor.position_in(&bounds) {
            position
        } else {
            match event {
                Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        *interaction = Interaction::None;
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
                        // Update interaction
                        // This is equivalent to "I'm starting drawing"
                        if self.state.brush_filter == BrushFilter::Brush {
                            *interaction = Interaction::Drawing {
                                from: None,
                                to: None,
                            };
                        } else if self.state.brush_filter == BrushFilter::Eraser {
                            *interaction = Interaction::Erasing {
                                from: None,
                                to: None,
                            };
                        } else if self.state.brush_filter == BrushFilter::Geometry {
                            *interaction = Interaction::Geometry {
                                from: None,
                                to: None,
                            };
                        } else {
                            *interaction = Interaction::None;
                        }

                        // Update first point
                        match interaction {
                            // Drawing with a freehand brush
                            Interaction::Drawing { mut from, to } => match (from, *to) {
                                (None, None) => {
                                    from = Some(cursor_position);
                                    *interaction = Interaction::Drawing { from, to: *to };
                                    None
                                }
                                _ => None,
                            },

                            // Erasing with a freehand brush
                            Interaction::Erasing { mut from, to } => match (from, *to) {
                                (None, None) => {
                                    from = Some(cursor_position);
                                    *interaction = Interaction::Erasing { from, to: *to };
                                    None
                                }
                                _ => None,
                            },

                            // Drawing a geometry form
                            Interaction::Geometry { mut from, to } => match (from, *to) {
                                (None, None) => {
                                    from = Some(cursor_position);
                                    *interaction = Interaction::Geometry { from, to: *to };
                                    None
                                }
                                _ => None,
                            },
                            _ => None,
                        }
                    }

                    // Update drawing
                    mouse::Event::CursorMoved { position: _ } => match interaction {
                        // Drawing with a freehand brush
                        Interaction::Drawing { mut from, mut to } => match (from, to) {
                            (None, None) => {
                                from = Some(cursor_position);
                                *interaction = Interaction::Drawing { from, to };
                                None
                            }
                            (Some(_), None) => {
                                to = Some(cursor_position);

                                let message = Some(Strokes {
                                    brush: BrushFilter::Brush,
                                    from,
                                    to,
                                    color: self.state.brush_color,
                                    size: self.state.brush_size,
                                    geometry_form: None,
                                });

                                from = to;
                                to = None;
                                *interaction = Interaction::Drawing { from, to };

                                message
                            }
                            _ => None,
                        },

                        // Erasing with a freehand brush
                        Interaction::Erasing { mut from, mut to } => match (from, to) {
                            (None, None) => {
                                from = Some(cursor_position);
                                *interaction = Interaction::Erasing { from, to };
                                None
                            }
                            (Some(_), None) => {
                                to = Some(cursor_position);

                                let message = Some(Strokes {
                                    brush: BrushFilter::Eraser,
                                    from,
                                    to,
                                    color: Color::WHITE,
                                    size: self.state.brush_size,
                                    geometry_form: None,
                                });

                                from = to;
                                to = None;
                                *interaction = Interaction::Erasing { from, to };

                                message
                            }
                            _ => None,
                        },
                        _ => None,
                    },

                    // Stop drawing
                    mouse::Event::ButtonReleased(mouse::Button::Left) => match interaction {
                        Interaction::Geometry { from, to } => match (*from, *to) {
                            (Some(_), None) => {
                                *to = Some(cursor_position);

                                let message = Some(Strokes {
                                    brush: BrushFilter::Geometry,
                                    from: *from,
                                    to: *to,
                                    color: self.state.brush_color,
                                    size: self.state.brush_size,
                                    geometry_form: self.state.geometry_form,
                                });

                                *interaction = Interaction::None;

                                message
                            }
                            _ => {
                                *interaction = Interaction::None;
                                None
                            }
                        },

                        _ => {
                            *interaction = Interaction::None;
                            None
                        }
                    },
                    _ => None,
                };
                (event::Status::Captured, message)
            }
            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        _interaction: &Interaction,
        bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut contents = Vec::new();

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

        let strokes_content = self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
            Strokes::draw_all(self.strokes, frame);
        });
        contents.extend(vec![strokes_content]);

        contents
    }

    fn mouse_interaction(
        &self,
        interaction: &Interaction,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(&bounds) {
            match interaction {
                Interaction::Drawing { from: _, to: _ } => mouse::Interaction::Crosshair,
                Interaction::Erasing { from: _, to: _ } => mouse::Interaction::Crosshair,
                _ => mouse::Interaction::default(),
            }
        } else {
            mouse::Interaction::default()
        }
    }
}
