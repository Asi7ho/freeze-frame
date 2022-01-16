use iced::{container, text_input, Color};

pub struct HeaderStyle;

impl container::StyleSheet for HeaderStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::WHITE),
            background: Some(Color::from_rgb8(34, 34, 34).into()),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

impl text_input::StyleSheet for HeaderStyle {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Color::from_rgb8(34, 34, 34).into(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: Color::from_rgb8(34, 34, 34).into(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::WHITE
    }

    fn value_color(&self) -> Color {
        Color::WHITE
    }

    fn selection_color(&self) -> Color {
        Color::from_rgb8(64, 64, 64)
    }
}
