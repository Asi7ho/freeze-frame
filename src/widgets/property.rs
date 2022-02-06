use iced::{container, Color, Column, Container, Element, Length};

use crate::FreezeFrameMessage;

#[derive(Debug, Default)]
pub struct PropertyState {}

pub fn view(_property_state: &mut PropertyState) -> Element<FreezeFrameMessage> {
    let property = Container::new(Column::new())
        .height(Length::Fill)
        .width(Length::Units(225))
        .style(PropertyStyle);

    return property.into();
}

pub struct PropertyStyle;

impl container::StyleSheet for PropertyStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: None,
            background: Some(Color::from_rgb8(25, 25, 25).into()),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}
