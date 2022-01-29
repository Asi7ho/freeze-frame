use iced::{container, Color};

pub struct PropertyStyle;

impl container::StyleSheet for PropertyStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: None,
            background: Some(Color::from_rgb8(25, 25, 25).into()),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}
