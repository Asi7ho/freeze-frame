use iced::{
    alignment::Horizontal,
    theme,
    widget::{Column, Container, PickList, Row, Slider, Text},
    Element, Length,
};

use crate::{
    styles::{RightBarStyle, SliderStyle},
    tools::filters::{BrushFilter, GeometryForm},
    FreezeFrameMessage,
};

use super::{MainViewMessage, PropertyMessage};

#[derive(Debug, Default)]
pub struct PropertyState {
    pub filter: BrushFilter,
    pub brush_slider_value: f32,
    pub eraser_slider_value: f32,
    pub geometry_form: Option<GeometryForm>,
}

pub fn view(property_state: &PropertyState) -> Element<FreezeFrameMessage> {
    let heading = Text::new("Properties")
        .size(22)
        .width(Length::Fixed(225.0))
        .horizontal_alignment(Horizontal::Center);
    let properties = match property_state.filter {
        BrushFilter::Brush => brush_properties(&property_state.brush_slider_value),
        BrushFilter::Eraser => brush_properties(&property_state.eraser_slider_value),
        BrushFilter::Geometry => geometry_properties(&property_state.geometry_form),
        _ => Column::new(),
    };

    let container = Container::new(Column::new().push(heading).push(properties).spacing(10))
        .height(Length::Fill)
        .width(Length::Fixed(225.0))
        .padding(10)
        .style(theme::Container::Custom(Box::new(RightBarStyle)));

    container.into()
}

fn brush_properties(slider_value: &f32) -> Column<FreezeFrameMessage> {
    let size_text = Text::new("Size: ");
    let size_slider = Slider::new(1.0..=50.0, *slider_value, |v| {
        FreezeFrameMessage::MainView(MainViewMessage::Property(PropertyMessage::Slide(v)))
    })
    .style(theme::Slider::Custom(Box::new(SliderStyle)));
    let size_ind = Text::new(slider_value.to_string());
    let size = Row::new()
        .push(size_text)
        .push(size_slider)
        .push(size_ind)
        .spacing(15);
    let properties = Column::new()
        .push(size)
        .padding(10)
        .width(Length::Fixed(225.0));

    properties
}

fn geometry_properties(geometry_form: &Option<GeometryForm>) -> Column<FreezeFrameMessage> {
    let form_text = Text::new("Form: ");
    let form_picklist = PickList::new(&GeometryForm::ALL[..], *geometry_form, |form| {
        FreezeFrameMessage::MainView(MainViewMessage::Property(
            PropertyMessage::ChangeGeometryForm(form),
        ))
    });
    let geometry_list = Row::new().push(form_text).push(form_picklist).spacing(15);
    let properties = Column::new()
        .push(geometry_list)
        .padding(10)
        .width(Length::Fixed(225.0));

    properties
}
