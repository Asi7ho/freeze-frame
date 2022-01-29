use iced::{Column, Container, Element, Length};

pub mod timeline_style;
pub use timeline_style::TimelineStyle;

use crate::FreezeFrameMessage;

#[derive(Debug, Default)]
pub struct TimelineState {}

pub fn view(timeline_state: &mut TimelineState) -> Element<FreezeFrameMessage> {
    let property = Container::new(Column::new())
        .height(Length::Units(125))
        .width(Length::Fill)
        .style(TimelineStyle);

    return property.into();
}
