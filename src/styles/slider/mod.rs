use iced::{
    widget::slider::{self, Handle, HandleShape},
    Color, Theme,
};

pub struct SliderStyle;

impl slider::StyleSheet for SliderStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail_colors: (
                Color::from_rgb8(187, 182, 197),
                Color::from_rgb8(187, 182, 197),
            ),
            handle: Handle {
                shape: HandleShape::Circle { radius: 8.0 },
                color: Color::from_rgb8(187, 182, 197),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
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
