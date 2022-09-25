use iced::{
    alignment::Horizontal,
    pure::{
        widget::{Column, Container, PickList, Row, Slider, Text, TextInput},
        Element,
    },
    Color, Length,
};

use crate::{
    styles::{
        ContainerState, ContainerStyle, SliderState, SliderStyle, TextInputState, TextInputStyle,
    },
    tools::filters::{BrushFilter, GeometryForm},
    FreezeFrameMessage,
};

use super::{MainViewMessage, PropertyMessage};

#[derive(Debug, Default)]
pub struct PropertyState {
    pub filter: BrushFilter,
    pub resolution: (f32, f32),
    pub brush_slider_value: f32,
    pub eraser_slider_value: f32,
    pub geometry_form: Option<GeometryForm>,
}

pub fn view(property_state: &PropertyState) -> Element<FreezeFrameMessage> {
    let heading = Text::new("Properties")
        .size(22)
        .color(Color::WHITE)
        .width(Length::Units(225))
        .horizontal_alignment(Horizontal::Center);
    let properties = match property_state.filter {
        BrushFilter::Pointer => canvas_properties(&property_state.resolution),
        BrushFilter::Brush => brush_properties(&property_state.brush_slider_value),
        BrushFilter::Eraser => brush_properties(&property_state.eraser_slider_value),
        BrushFilter::Geometry => geometry_properties(&property_state.geometry_form),
        _ => Column::new(),
    };

    let container = Container::new(Column::new().push(heading).push(properties).spacing(10))
        .height(Length::Fill)
        .width(Length::Units(225))
        .padding(10)
        .style(ContainerStyle {
            state: ContainerState::RightBar,
        });

    container.into()
}

fn canvas_properties(resolution: &(f32, f32)) -> Column<FreezeFrameMessage> {
    let resolution_x_text = Text::new("Width: ").color(Color::WHITE);
    let resolution_y_text = Text::new("Height: ").color(Color::WHITE);

    let input_x = TextInput::new(&resolution.0.to_string(), &resolution.0.to_string(), |x| {
        FreezeFrameMessage::MainView(MainViewMessage::Property(
            PropertyMessage::ChangeResolutionX(x),
        ))
    })
    .style(TextInputStyle {
        state: TextInputState::Properties,
    });
    let input_y = TextInput::new(&resolution.1.to_string(), &resolution.1.to_string(), |y| {
        FreezeFrameMessage::MainView(MainViewMessage::Property(
            PropertyMessage::ChangeResolutionY(y),
        ))
    })
    .style(TextInputStyle {
        state: TextInputState::Properties,
    });

    let resolution_x = Row::new().push(resolution_x_text).push(input_x).spacing(15);

    let resolution_y = Row::new().push(resolution_y_text).push(input_y).spacing(15);

    let resolution = Column::new()
        .push(resolution_x)
        .push(resolution_y)
        .spacing(10)
        .padding(10)
        .width(Length::Units(225));

    resolution
}

fn brush_properties(slider_value: &f32) -> Column<FreezeFrameMessage> {
    let size_text = Text::new("Size: ").color(Color::WHITE);
    let size_slider = Slider::new(1.0..=50.0, *slider_value, |v| {
        FreezeFrameMessage::MainView(MainViewMessage::Property(PropertyMessage::Slide(v)))
    })
    .style(SliderStyle {
        state: SliderState::Properties,
    });
    let size_ind = Text::new(slider_value.to_string()).color(Color::WHITE);
    let size = Row::new()
        .push(size_text)
        .push(size_slider)
        .push(size_ind)
        .spacing(15);
    let properties = Column::new()
        .push(size)
        .padding(10)
        .width(Length::Units(225));

    properties
}

fn geometry_properties(geometry_form: &Option<GeometryForm>) -> Column<FreezeFrameMessage> {
    let form_text = Text::new("Form: ").color(Color::WHITE);
    let form_picklist = PickList::new(&GeometryForm::ALL[..], *geometry_form, |form| {
        FreezeFrameMessage::MainView(MainViewMessage::Property(
            PropertyMessage::ChangeGeometryForm(form),
        ))
    });
    let geometry_list = Row::new().push(form_text).push(form_picklist).spacing(15);
    let properties = Column::new()
        .push(geometry_list)
        .padding(10)
        .width(Length::Units(225));

    properties
}
