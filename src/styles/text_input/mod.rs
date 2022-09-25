use iced::{pure::widget::text_input, Color};

// // Text Input Style
#[derive(Clone, Copy)]
pub struct TextInputStyle {
    pub state: TextInputState,
}

#[derive(Clone, Copy)]
pub enum TextInputState {
    SceneTitle,
    Properties,
}

impl text_input::StyleSheet for TextInputStyle {
    fn active(&self) -> text_input::Style {
        match self.state {
            TextInputState::SceneTitle => text_input::Style {
                background: Color::from_rgb8(34, 34, 34).into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            TextInputState::Properties => text_input::Style {
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
