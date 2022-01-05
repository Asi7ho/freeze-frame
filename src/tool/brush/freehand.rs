use druid::{kurbo::BezPath, Color, Point};

pub struct Freehand {
    pub path: BezPath,
    pub color: Color,
    pub size: f64,
}

impl Freehand {
    pub fn new(size: f64, color: Color) -> Self {
        let path = BezPath::new();
        return Self { path, color, size };
    }

    pub fn start_draw(&mut self, point: Point) {
        self.path.move_to(point);
    }

    pub fn draw(&mut self, point: Point) {
        self.path.line_to(point);
    }
}
