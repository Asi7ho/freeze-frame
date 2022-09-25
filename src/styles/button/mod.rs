use iced::{pure::widget::button, Color};

// Button Style
#[derive(Clone, Copy)]
pub struct ButtonStyle {
    pub state: ButtonState,
}

#[derive(Clone, Copy)]
pub enum ButtonState {
    IconNotSelected,
    IconSelected,
    ColorButton(Color),
}

impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        match self.state {
            ButtonState::IconNotSelected => button::Style {
                background: Some(Color::TRANSPARENT.into()),
                border_radius: 10.0,
                ..button::Style::default()
            },
            ButtonState::IconSelected => button::Style {
                background: Some(Color::from_rgba8(187, 182, 197, 0.15).into()),
                border_radius: 10.0,
                ..button::Style::default()
            },
            ButtonState::ColorButton(color) => button::Style {
                background: Some(color.into()),
                border_radius: 8.0,
                border_width: 2.5,
                border_color: Color::from_rgba8(187, 182, 197, 0.35),
                ..button::Style::default()
            },
        }
    }
}
