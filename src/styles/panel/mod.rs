use iced::{widget::container, Color, Theme};

pub struct HeaderStyle;

impl container::StyleSheet for HeaderStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::WHITE),
            ..container::Appearance::default()
        }
    }
}

pub struct RightBarStyle;

impl container::StyleSheet for RightBarStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Color::from_rgb8(25, 25, 25).into()),
            ..container::Appearance::default()
        }
    }
}

pub struct TimeLineStyle;

impl container::StyleSheet for TimeLineStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            ..container::Appearance::default()
        }
    }
}
