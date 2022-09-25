use iced::{pure::widget::container, Color};

// Container Style
#[derive(Clone, Copy)]
pub struct ContainerStyle {
    pub state: ContainerState,
}

#[derive(Clone, Copy)]
pub enum ContainerState {
    Header,
    RightBar,
    TimeLine,
}

impl container::StyleSheet for ContainerStyle {
    fn style(&self) -> container::Style {
        match self.state {
            ContainerState::Header => container::Style {
                text_color: Some(Color::WHITE),
                ..container::Style::default()
            },
            ContainerState::RightBar => container::Style {
                background: Some(Color::from_rgb8(25, 25, 25).into()),
                ..container::Style::default()
            },
            ContainerState::TimeLine => container::Style {
                ..container::Style::default()
            },
        }
    }
}
