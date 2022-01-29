use iced::{container, Color};

pub struct CanvasStyle;

impl container::StyleSheet for CanvasStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: None,
            background: Some(Color::from_rgb8(34, 34, 34).into()),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}
