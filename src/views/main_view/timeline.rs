use iced::{
    theme,
    widget::{Column, Container},
    Element, Length,
};

use crate::{styles::TimeLineStyle, FreezeFrameMessage};

#[derive(Debug, Default)]
pub struct TimelineState {}

pub fn view(_timeline_state: &TimelineState) -> Element<FreezeFrameMessage> {
    let property = Container::new(Column::new())
        .height(Length::Fixed(125.0))
        .width(Length::Fill)
        .style(theme::Container::Custom(Box::new(TimeLineStyle)));

    property.into()
}
