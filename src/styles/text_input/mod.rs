use iced::{widget::text_input, Color, Theme};

pub struct SceneTitleStyle;

impl text_input::StyleSheet for SceneTitleStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::from_rgb8(34, 34, 34).into(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            icon_color: Color::TRANSPARENT,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        self.focused(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgba8(255, 255, 255, 0.3)
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        Color::WHITE
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb8(64, 64, 64)
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgba8(54, 54, 54, 0.8)
    }
}
