use iced::{
    mouse,
    widget::canvas::{
        self,
        event::{self, Event},
        Cursor, Frame, Path,
    },
    Color, Point, Rectangle, Size, Theme,
};

use crate::{tools::filters::BrushFilter, views::main_view::canvas::CanvasState};

use super::{
    strokes::{draw_all, Line},
    BrushComponent, Strokes,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Interaction {
    #[default]
    None,
    Drawing {
        line: Line,
    },
    Erasing {
        line: Line,
    },
    Geometry {
        line: Line,
    },
}

pub struct Painting<'a> {
    pub state: &'a CanvasState,
    pub strokes: &'a [Strokes],
}

impl<'a> canvas::Program<Strokes> for Painting<'a> {
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
            if let Event::Mouse(mouse_event) = event {
                if mouse_event == mouse::Event::ButtonReleased(mouse::Button::Left) {
                    *interaction = Interaction::None;
                }
            }
            return (event::Status::Ignored, None);
        };

        log::info!("Canvas event {:?}", event);
        log::info!(
            "Canvas interaction {:?} with brush {:?}",
            interaction,
            self.state.brush_component
        );
        match event {
            Event::Mouse(mouse_event) => {
                let message = match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        // Update interaction
                        // This is equivalent to "I'm starting drawing"
                        update_interaction(interaction, self.state.brush_component);

                        // Update first point
                        start_drawing(interaction, cursor_position)
                    }

                    // Update drawing
                    mouse::Event::CursorMoved { position: _ } => {
                        update_drawing(interaction, self.state.brush_component, cursor_position)
                    }

                    // Stop drawing
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        finish_drawing(interaction, self.state.brush_component, cursor_position)
                    }
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
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut contents = Vec::new();

        // Foreground canvas
        let mut frame = Frame::new(bounds.size());
        let top_left = Point::new(
            frame.center().x - 750.0 / 2.0,
            frame.center().y - 435.0 / 2.0,
        );
        let size = Size::new(750.0, 435.0);
        let foreground_canvas = Path::rectangle(top_left, size);
        frame.fill(&foreground_canvas, Color::WHITE);
        contents.push(frame.into_geometry());

        let strokes_content = self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
            draw_all(self.strokes, frame);
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
                Interaction::Drawing { line: _ } => mouse::Interaction::Crosshair,
                Interaction::Erasing { line: _ } => mouse::Interaction::Crosshair,
                _ => mouse::Interaction::default(),
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

fn update_interaction(interaction: &mut Interaction, brush_component: BrushComponent) {
    if brush_component.brush == BrushFilter::Brush {
        *interaction = Interaction::Drawing {
            line: Line::default(),
        };
    } else if brush_component.brush == BrushFilter::Eraser {
        *interaction = Interaction::Erasing {
            line: Line::default(),
        };
    } else if brush_component.brush == BrushFilter::Geometry {
        *interaction = Interaction::Geometry {
            line: Line::default(),
        };
    } else {
        *interaction = Interaction::None;
    }
}

fn start_drawing(interaction: &mut Interaction, cursor_position: Point) -> Option<Strokes> {
    match interaction {
        // Drawing with a freehand brush
        Interaction::Drawing { line: _ } => {
            *interaction = Interaction::Drawing {
                line: Line::new(Some(cursor_position), None),
            };
        }

        // Erasing with a freehand brush
        Interaction::Erasing { line: _ } => {
            *interaction = Interaction::Erasing {
                line: Line::new(Some(cursor_position), None),
            };
        }

        // Drawing a geometry form
        Interaction::Geometry { line: _ } => {
            *interaction = Interaction::Geometry {
                line: Line::new(Some(cursor_position), None),
            };
        }
        _ => (),
    };

    None
}

fn update_drawing(
    interaction: &mut Interaction,
    brush_component: BrushComponent,
    cursor_position: Point,
) -> Option<Strokes> {
    let message = match interaction {
        Interaction::Drawing { line } => match (line.from, line.to) {
            (Some(_), None) => {
                let message = Some(Strokes {
                    brush_component: BrushComponent {
                        brush: BrushFilter::Brush,
                        size: brush_component.size,
                        color: brush_component.color,
                        geometry_form: None,
                    },
                    line: Line::new(line.from, Some(cursor_position)),
                });

                let line = Line::new(Some(cursor_position), None);
                *interaction = Interaction::Drawing { line };

                message
            }
            _ => None,
        },

        Interaction::Erasing { line } => match (line.from, line.to) {
            (Some(_), None) => {
                let message = Some(Strokes {
                    brush_component: BrushComponent {
                        brush: BrushFilter::Eraser,
                        size: brush_component.size,
                        color: Color::WHITE,
                        geometry_form: None,
                    },
                    line: Line::new(line.from, Some(cursor_position)),
                });

                let line = Line::new(Some(cursor_position), None);
                *interaction = Interaction::Erasing { line };

                message
            }
            _ => None,
        },
        _ => None,
    };

    message
}

fn finish_drawing(
    interaction: &mut Interaction,
    brush_component: BrushComponent,
    cursor_position: Point,
) -> Option<Strokes> {
    let message = match interaction {
        Interaction::Geometry { line } => match (line.from, line.to) {
            (Some(_), None) => {
                let message = Some(Strokes {
                    brush_component: BrushComponent {
                        brush: BrushFilter::Geometry,
                        size: brush_component.size,
                        color: brush_component.color,
                        geometry_form: brush_component.geometry_form,
                    },
                    line: Line::new(line.from, Some(cursor_position)),
                });

                message
            }
            _ => None,
        },

        _ => None,
    };

    *interaction = Interaction::None;
    message
}
