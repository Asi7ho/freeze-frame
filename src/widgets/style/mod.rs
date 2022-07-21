use iced::{
    pure::widget::{
        button, container,
        slider::{self, Handle, HandleShape},
        text_input,
    },
    Color,
};

// Button Style
#[derive(Clone, Copy)]
pub struct WButtonStyle {
    pub state: WButtonState,
}

#[derive(Clone, Copy)]
pub enum WButtonState {
    IconNotSelected,
    IconSelected,
    ColorButton(Color),
}

impl button::StyleSheet for WButtonStyle {
    fn active(&self) -> button::Style {
        match self.state {
            WButtonState::IconNotSelected => button::Style {
                background: Some(Color::TRANSPARENT.into()),
                border_radius: 10.0,
                ..button::Style::default()
            },
            WButtonState::IconSelected => button::Style {
                background: Some(Color::from_rgba8(187, 182, 197, 0.15).into()),
                border_radius: 10.0,
                ..button::Style::default()
            },
            WButtonState::ColorButton(color) => button::Style {
                background: Some(color.into()),
                border_radius: 8.0,
                border_width: 2.5,
                border_color: Color::from_rgba8(187, 182, 197, 0.35),
                ..button::Style::default()
            },
        }
    }
}

// Container Style
#[derive(Clone, Copy)]
pub struct WContainerStyle {
    pub state: WContainerState,
}

#[derive(Clone, Copy)]
pub enum WContainerState {
    Header,
    RightBar,
    TimeLine,
}

impl container::StyleSheet for WContainerStyle {
    fn style(&self) -> container::Style {
        match self.state {
            WContainerState::Header => container::Style {
                text_color: Some(Color::WHITE),
                ..container::Style::default()
            },
            WContainerState::RightBar => container::Style {
                background: Some(Color::from_rgb8(25, 25, 25).into()),
                ..container::Style::default()
            },
            WContainerState::TimeLine => container::Style {
                ..container::Style::default()
            },
        }
    }
}

// Slider Style
#[derive(Clone, Copy)]
pub struct WSliderStyle {
    pub state: WSliderState,
}

#[derive(Clone, Copy)]
pub enum WSliderState {
    Properties,
}

impl slider::StyleSheet for WSliderStyle {
    fn active(&self) -> slider::Style {
        match self.state {
            WSliderState::Properties => slider::Style {
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

// // Text Input Style
#[derive(Clone, Copy)]
pub struct WTextInputStyle {
    pub state: WTextInputState,
}

#[derive(Clone, Copy)]
pub enum WTextInputState {
    SceneTitle,
    Properties,
}

impl text_input::StyleSheet for WTextInputStyle {
    fn active(&self) -> text_input::Style {
        match self.state {
            WTextInputState::SceneTitle => text_input::Style {
                background: Color::from_rgb8(34, 34, 34).into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            WTextInputState::Properties => text_input::Style {
                background: Color::from_rgb8(34, 34, 34).into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn focused(&self) -> text_input::Style {
        self.active()
    }

    fn placeholder_color(&self) -> iced::Color {
        Color::from_rgba(1.0, 1.0, 1.0, 0.3)
    }

    fn value_color(&self) -> iced::Color {
        Color::WHITE
    }

    fn selection_color(&self) -> iced::Color {
        Color::from_rgb8(64, 64, 64)
    }
}
