use iced::{
    pure::{
        widget::{Column, Container},
        Element,
    },
    Length,
};

use crate::{
    styles::{ContainerState, ContainerStyle},
    FreezeFrameMessage,
};

#[derive(Debug, Default)]
pub struct TimelineState {}

pub fn view(_timeline_state: &TimelineState) -> Element<FreezeFrameMessage> {
    let property = Container::new(Column::new())
        .height(Length::Units(125))
        .width(Length::Fill)
        .style(ContainerStyle {
            state: ContainerState::TimeLine,
        });

    property.into()
}
