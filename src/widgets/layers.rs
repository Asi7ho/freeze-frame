use iced::{
    alignment::Horizontal,
    pure::{
        widget::{Column, Container, Text},
        Element,
    },
    Color, Length,
};

use crate::message::FreezeFrameMessage;

use super::style::{WContainerState, WContainerStyle};

#[derive(Debug, Default)]
pub struct LayerState {
    pub level: i8,
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
        .style(WContainerStyle {
            state: WContainerState::RightBar,
        });

    return container.into();
}
