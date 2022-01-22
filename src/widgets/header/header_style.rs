use iced::{button, container, text_input, Color};

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

pub enum HeaderButtonStyle {
    HeaderButtonSelectedStyle,
    HeaderButtonDefaultStyle,
}

impl button::StyleSheet for HeaderButtonStyle {
    fn active(&self) -> button::Style {
        match self {
            HeaderButtonStyle::HeaderButtonSelectedStyle => button::Style {
                shadow_offset: Default::default(),
                background: Some(Color::from_rgba8(187, 182, 197, 0.15).into()),
                border_radius: 10.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                text_color: Color::TRANSPARENT,
            },
            HeaderButtonStyle::HeaderButtonDefaultStyle => button::Style {
                shadow_offset: Default::default(),
                background: None,
                border_radius: 10.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                text_color: Color::TRANSPARENT,
            },
        }
    }
}

pub struct HeaderColorButtonStyle {
    pub color: Color,
    pub selected: bool,
}

impl button::StyleSheet for HeaderColorButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Default::default(),
            background: Some(self.color.into()),
            border_radius: 10.0,
            border_width: if self.selected { 3.0 } else { 0.0 },
            border_color: Color::from_rgba8(187, 182, 197, 0.15),
            text_color: Color::TRANSPARENT,
        }
    }
}
