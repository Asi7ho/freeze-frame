use druid::{Color, Data, Lens};

#[derive(Data, Lens, Clone)]
pub struct AppState {
    pub canvas_width: f64,
    pub canvas_height: f64,
    pub canvas_background_color: Color,
}

impl AppState {
    pub fn new(canvas_width: f64, canvas_height: f64, canvas_background_color: Color) -> Self {
        return Self {
            canvas_width,
            canvas_height,
            canvas_background_color,
        };
    }
}
