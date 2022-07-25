use iced::{
    pure::{
        widget::{Column, Container},
        Element,
    },
    Length,
};

use crate::message::FreezeFrameMessage;

use super::style::{WContainerState, WContainerStyle};

#[derive(Debug, Default)]
pub struct TimelineState {}

pub fn view(_timeline_state: &TimelineState) -> Element<FreezeFrameMessage> {
    let property = Container::new(Column::new())
        .height(Length::Units(125))
        .width(Length::Fill)
        .style(WContainerStyle {
            state: WContainerState::TimeLine,
        });

    property.into()
}
