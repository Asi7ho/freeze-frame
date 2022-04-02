use std::ops::RangeInclusive;

use iced::{
    pure::widget::{slider, Slider},
    slider::Handle,
    Color,
};

use crate::FreezeFrameMessage;

pub struct WSlider<'a> {
    pub widget: Slider<'a, f32, FreezeFrameMessage>,
}

impl<'a> WSlider<'a> {
    pub fn new<F>(range: RangeInclusive<f32>, value: f32, message: F, style: WSliderStyle) -> Self
    where
        F: 'static + Fn(f32) -> FreezeFrameMessage,
    {
        let widget = Slider::new(range, value, message).style(style);
        Self { widget }
    }
}

pub struct WSliderStyle {
    pub rail_colors: (Color, Color),
    pub handle_radius: f32,
    pub handle_color: Color,
}

impl slider::StyleSheet for WSliderStyle {
    fn active(&self) -> slider::Style {
        slider::Style {
            rail_colors: self.rail_colors,
            handle: Handle {
                shape: iced::slider::HandleShape::Circle {
                    radius: self.handle_radius,
                },
                color: self.handle_color,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> iced::slider::Style {
        self.active()
    }

    fn dragging(&self) -> iced::slider::Style {
        self.active()
    }
}
