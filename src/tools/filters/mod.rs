#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BrushFilter {
    #[default]
    Pointer,
    Brush,
    Eraser,
    Geometry,
    Text,
    Fill,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum UiControlFilter {
    Grid,
    Trash,
    #[default]
    Ignore,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GeometryForm {
    #[default]
    Rectangle,
    Circle,
}

impl GeometryForm {
    pub const ALL: [GeometryForm; 2] = [GeometryForm::Rectangle, GeometryForm::Circle];
}

impl std::fmt::Display for GeometryForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GeometryForm::Rectangle => "Rectangle",
                GeometryForm::Circle => "Circle",
            }
        )
    }
}
