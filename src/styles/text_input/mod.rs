use iced::{widget::text_input, Color, Theme};

pub struct SceneTitleStyle;

impl text_input::StyleSheet for SceneTitleStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: super::BACKGROUND.into(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: super::TRANSPARENT,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        self.focused(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgba(1.0, 1.0, 1.0, 0.3)
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        super::TEXT_COLOR
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        super::SELECTION
    }
}
