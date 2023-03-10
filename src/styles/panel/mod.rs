use iced::{widget::container, Theme};

pub struct HeaderStyle;

impl container::StyleSheet for HeaderStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(super::TEXT_COLOR),
            ..container::Appearance::default()
        }
    }
}

pub struct RightBarStyle;

impl container::StyleSheet for RightBarStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(super::DARK_BACKGROUND.into()),
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
