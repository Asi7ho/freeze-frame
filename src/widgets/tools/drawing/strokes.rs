use iced::{
    canvas::{Frame, Path, Stroke},
    Color, Point,
};

#[derive(Debug, Clone, Copy)]
pub struct Strokes {
    pub from: Option<Point>,
    pub to: Option<Point>,
    pub color: Option<Color>,
    pub size: Option<f32>,
}

impl Strokes {
    pub fn draw_all(strokes: &[Strokes], frame: &mut Frame) {
        for stroke in strokes {
            if stroke.from.is_some() && stroke.to.is_some() {
                let freehand_stroke = Path::line(stroke.from.unwrap(), stroke.to.unwrap());
                let size = if stroke.size.is_some() {
                    stroke.size.unwrap()
                } else {
                    1.0
                };
                frame.stroke(
                    &freehand_stroke,
                    Stroke::default()
                        .with_color(stroke.color.unwrap())
                        .with_width(size),
                );
            }
        }
    }
}
