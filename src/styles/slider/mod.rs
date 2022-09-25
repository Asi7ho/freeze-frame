use iced::{
    pure::widget::slider::{self, Handle, HandleShape},
    Color,
};

// Slider Style
#[derive(Clone, Copy)]
pub struct SliderStyle {
    pub state: SliderState,
}

#[derive(Clone, Copy)]
pub enum SliderState {
    Properties,
}

impl slider::StyleSheet for SliderStyle {
    fn active(&self) -> slider::Style {
        match self.state {
            SliderState::Properties => slider::Style {
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
            },
        }
    }

    fn hovered(&self) -> slider::Style {
        self.active()
    }

    fn dragging(&self) -> slider::Style {
        self.active()
    }
}
