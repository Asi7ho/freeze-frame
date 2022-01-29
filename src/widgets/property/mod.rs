use iced::{Column, Container, Element, Length};

pub mod property_style;
pub use property_style::PropertyStyle;

use crate::FreezeFrameMessage;

#[derive(Debug, Default)]
pub struct PropertyState {}

pub fn view(property_state: &mut PropertyState) -> Element<FreezeFrameMessage> {
    let property = Container::new(Column::new())
        .height(Length::Fill)
        .width(Length::Units(225))
        .style(PropertyStyle);

    return property.into();
}
