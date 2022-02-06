use iced::{
    canvas::{Frame, Path, Stroke},
    Point,
};

#[derive(Debug, Clone, Copy)]
pub struct Strokes {
    pub from: Option<Point>,
    pub to: Option<Point>,
}

impl Strokes {
    pub fn draw_all(strokes: &[Strokes], frame: &mut Frame) {
        let freehand_strokes = Path::new(|p| {
            for stroke in strokes {
                if stroke.from.is_some() && stroke.to.is_some() {
                    p.move_to(stroke.from.unwrap());
                    p.line_to(stroke.to.unwrap())
                }
            }
        });

        frame.stroke(&freehand_strokes, Stroke::default().with_width(2.0));
    }
}
