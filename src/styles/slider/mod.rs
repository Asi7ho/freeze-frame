use iced::{
    widget::slider::{self, Handle, HandleShape},
    Theme,
};

pub struct SliderStyle;

impl slider::StyleSheet for SliderStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail_colors: super::RAILS,
            handle: Handle {
                shape: HandleShape::Circle { radius: 8.0 },
                color: super::HANDLE,
                border_width: 0.0,
                border_color: super::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> slider::Appearance {
        self.active(style)
    }

    fn dragging(&self, style: &Self::Style) -> slider::Appearance {
        self.active(style)
    }
}
