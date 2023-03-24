use iced::{
    widget::canvas::{Frame, LineCap, LineJoin, Path, Stroke},
    Point, Size,
};

use crate::tools::filters::{BrushFilter, GeometryForm};

use super::BrushComponent;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Line {
    pub from: Option<Point>,
    pub to: Option<Point>,
}

impl Line {
    pub fn new(from: Option<Point>, to: Option<Point>) -> Self {
        Self { from, to }
    }

    fn is_line(self) -> bool {
        return self.from.is_some() && self.to.is_some();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Strokes {
    pub brush_component: BrushComponent,
    pub line: Line,
}

pub fn draw_all(strokes: &[Strokes], frame: &mut Frame) {
    for stroke in strokes {
        match stroke.brush_component.brush {
            BrushFilter::Brush | BrushFilter::Eraser => draw_freehand(stroke, frame),
            BrushFilter::Geometry => draw_geometry(stroke, frame),
            _ => (),
        }
    }
}

fn draw_freehand(stroke: &Strokes, frame: &mut Frame) {
    if stroke.line.is_line() {
        if let (Some(from), Some(to)) = (stroke.line.from, stroke.line.to) {
            frame.stroke(
                &Path::line(from, to),
                Stroke::default()
                    .with_line_cap(LineCap::Round)
                    .with_color(stroke.brush_component.color)
                    .with_width(stroke.brush_component.size),
            );
        }
    }
}

fn draw_geometry(stroke: &Strokes, frame: &mut Frame) {
    if stroke.line.is_line() {
        if let (Some(from), Some(to)) = (stroke.line.from, stroke.line.to) {
            if let Some(geometry) = stroke.brush_component.geometry_form {
                let geometry_stroke = match geometry {
                    GeometryForm::Rectangle => {
                        let size = Size::new(to.x - from.x, to.y - from.y);
                        Path::rectangle(from, size)
                    }
                    GeometryForm::Circle => {
                        let radius = to.distance(from);
                        Path::circle(from, radius)
                    }
                };

                frame.stroke(
                    &geometry_stroke,
                    Stroke::default()
                        .with_line_join(LineJoin::Round)
                        .with_color(stroke.brush_component.color)
                        .with_width(stroke.brush_component.size),
                );
            }
        }
    }
}
