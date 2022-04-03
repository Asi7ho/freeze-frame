use iced::{
    alignment::Horizontal,
    pure::{
        widget::{container, Column, Container, Text},
        Element,
    },
    Color, Length,
};

use crate::FreezeFrameMessage;

#[derive(Debug, Default)]
pub struct LayerState {
    pub level: i8,
}

#[derive(Debug, Clone)]
pub enum LayerMessage {
    LayerChanged(usize),
}

pub fn view(layer_state: &LayerState) -> Element<FreezeFrameMessage> {
    let heading = Text::new("Layers")
        .size(22)
        .color(Color::WHITE)
        .width(Length::Units(225))
        .horizontal_alignment(Horizontal::Center);

    let container = Container::new(Column::new().push(heading).spacing(10))
        .height(Length::Fill)
        .width(Length::Units(225))
        .padding(10)
        .style(LayerStyle);

    return container.into();
}

pub struct LayerStyle;

impl container::StyleSheet for LayerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: None,
            background: Some(Color::from_rgb8(25, 25, 25).into()),
            border_radius: 0.0,
            border_width: 0.0,
            ..container::Style::default()
        }
    }
}
