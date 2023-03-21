use iced::{widget::button, Color, Theme};

pub struct IconStyle {
    pub selected: bool,
}

impl button::StyleSheet for IconStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let color = if self.selected {
            Color::from_rgb8(25, 25, 25)
        } else {
            Color::TRANSPARENT
        };

        button::Appearance {
            background: Some(color.into()),
            border_radius: 10.0,
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
            background: Some(self.color.into()),
            border_radius: 8.0,
            ..button::Appearance::default()
        }
    }
}
