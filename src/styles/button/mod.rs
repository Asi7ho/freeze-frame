use iced::{widget::button, Color, Theme};

pub struct IconStyle {
    pub selected: bool,
}

impl button::StyleSheet for IconStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let color = if self.selected {
            super::BUTTON_BACKGROUND
        } else {
            super::TRANSPARENT
        };

        button::Appearance {
            background: Some(color.into()),
            border_radius: super::BORDER_RADIUS,
            ..button::Appearance::default()
        }
    }
}

pub struct ColorButtonStyle {
    pub color: Color,
}

impl button::StyleSheet for ColorButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(self.color)),
            border_radius: 8.0,
            ..button::Appearance::default()
        }
    }
}
