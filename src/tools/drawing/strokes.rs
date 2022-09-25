use iced::{
    canvas::{Frame, LineCap, LineJoin, Path, Stroke},
    Point, Size,
};

use crate::tools::filters::{BrushFilter, GeometryForm};

use super::BrushComponent;

#[derive(Debug, Clone, Copy)]
pub struct Strokes {
    pub brush_component: BrushComponent,
    pub from: Option<Point>,
    pub to: Option<Point>,
    pub geometry_form: Option<GeometryForm>,
}

impl Strokes {
    pub fn draw_all(strokes: &[Strokes], frame: &mut Frame) {
        for stroke in strokes {
            match stroke.brush_component.brush {
                BrushFilter::Brush | BrushFilter::Eraser => {
                    if stroke.from.is_some() && stroke.to.is_some() {
                        let freehand_stroke = Path::line(stroke.from.unwrap(), stroke.to.unwrap());

                        frame.stroke(
                            &freehand_stroke,
                            Stroke::default()
                                .with_line_cap(LineCap::Round)
                                .with_color(stroke.brush_component.color)
                                .with_width(stroke.brush_component.size),
                        );
                    }
                }
                BrushFilter::Geometry => {
                    if stroke.from.is_some() && stroke.to.is_some() {
                        let geometry_stroke = match stroke.geometry_form {
                            Some(g) => match g {
                                GeometryForm::Rectangle => {
                                    let size = Size {
                                        width: stroke.to.unwrap().x - stroke.from.unwrap().x,
                                        height: stroke.to.unwrap().y - stroke.from.unwrap().y,
                                    };
                                    Path::rectangle(stroke.from.unwrap(), size)
                                }
                                GeometryForm::Circle => {
                                    let radius = stroke.to.unwrap().distance(stroke.from.unwrap());
                                    Path::circle(stroke.from.unwrap(), radius)
                                }
                            },
                            None => return,
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
                _ => (),
            }
        }
    }
}
