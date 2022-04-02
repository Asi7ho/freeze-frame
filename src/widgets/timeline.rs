use iced::{
    pure::{
        widget::{container, Column, Container},
        Element,
    },
    Length,
};

use crate::FreezeFrameMessage;

#[derive(Debug, Default)]
pub struct TimelineState {}

pub fn view(_timeline_state: &TimelineState) -> Element<FreezeFrameMessage> {
    let property = Container::new(Column::new())
        .height(Length::Units(125))
        .width(Length::Fill)
        .style(TimelineStyle);

    return property.into();
}

pub struct TimelineStyle;

impl container::StyleSheet for TimelineStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: None,
            border_radius: 0.0,
            border_width: 0.0,
            ..container::Style::default()
        }
    }
}
