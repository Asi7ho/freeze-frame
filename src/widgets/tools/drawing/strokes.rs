use iced::{
    canvas::{Frame, Path, Stroke},
    Color, Point,
};

#[derive(Debug, Clone, Copy)]
pub struct Strokes {
    pub from: Option<Point>,
    pub to: Option<Point>,
    pub color: Option<Color>,
}

impl Strokes {
    pub fn draw_all(strokes: &[Strokes], frame: &mut Frame) {
        for stroke in strokes {
            if stroke.from.is_some() && stroke.to.is_some() {
                let freehand_stroke = Path::line(stroke.from.unwrap(), stroke.to.unwrap());
                frame.stroke(
                    &freehand_stroke,
                    Stroke::default()
                        .with_color(stroke.color.unwrap())
                        .with_width(2.0),
                );
            }
        }
    }
}
