use iced::{
    alignment::Horizontal,
    pure::{
        widget::{container, Column, Container, Row, Text},
        Element,
    },
    Color, Length,
};

use crate::FreezeFrameMessage;

use crate::widgets::components::slider::{WSlider, WSliderStyle};
use crate::widgets::header::BrushFilter;

use super::components::text_input::{WTextInput, WTextInputStyle};

#[derive(Debug, Default)]
pub struct PropertyState {
    pub filter: BrushFilter,
    pub resolution: (f32, f32),
    pub brush_slider_value: f32,
    pub eraser_slider_value: f32,
}

#[derive(Debug, Clone)]
pub enum PropertyMessage {
    SliderChanged(f32),
    ResolutionXChanged(String),
    ResolutionYChanged(String),
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
        _ => Column::new(),
    };

    let container = Container::new(Column::new().push(heading).push(properties).spacing(10))
        .height(Length::Fill)
        .width(Length::Units(225))
        .padding(10)
        .style(PropertyStyle);

    return container.into();
}

fn canvas_properties(resolution: &(f32, f32)) -> Column<FreezeFrameMessage> {
    let resolution_x_text = Text::new("Width: ").color(Color::WHITE);
    let resolution_y_text = Text::new("Height: ").color(Color::WHITE);

    let input_x = WTextInput::new(
        &resolution.0.to_string(),
        &resolution.0.to_string(),
        |x| FreezeFrameMessage::PropertyInteraction(PropertyMessage::ResolutionXChanged(x)),
        WTextInputStyle {
            background: Color::from_rgb8(34, 34, 34),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            placeholder_color: Color::WHITE,
            value_color: Color::WHITE,
            selection_color: Color::from_rgb8(64, 64, 64),
        },
    );
    let input_y = WTextInput::new(
        &resolution.1.to_string(),
        &resolution.1.to_string(),
        |y| FreezeFrameMessage::PropertyInteraction(PropertyMessage::ResolutionYChanged(y)),
        WTextInputStyle {
            background: Color::from_rgb8(34, 34, 34),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            placeholder_color: Color::WHITE,
            value_color: Color::WHITE,
            selection_color: Color::from_rgb8(64, 64, 64),
        },
    );

    let resolution_x = Row::new()
        .push(resolution_x_text)
        .push(input_x.widget)
        .spacing(15);

    let resolution_y = Row::new()
        .push(resolution_y_text)
        .push(input_y.widget)
        .spacing(15);

    let resolution = Column::new()
        .push(resolution_x)
        .push(resolution_y)
        .spacing(10)
        .padding(10)
        .width(Length::Units(225));

    return resolution;
}

fn brush_properties(slider_value: &f32) -> Column<FreezeFrameMessage> {
    let size_text = Text::new("Size: ").color(Color::WHITE);
    let size_slider = WSlider::new(
        1.0..=50.0,
        *slider_value,
        |v| FreezeFrameMessage::PropertyInteraction(PropertyMessage::SliderChanged(v)),
        WSliderStyle {
            rail_colors: (
                Color::from_rgb8(187, 182, 197),
                Color::from_rgb8(187, 182, 197),
            ),
            handle_radius: 8.0,
            handle_color: Color::from_rgb8(187, 182, 197),
        },
    );
    let size_ind = Text::new(slider_value.to_string()).color(Color::WHITE);
    let size = Row::new()
        .push(size_text)
        .push(size_slider.widget)
        .push(size_ind)
        .spacing(15);
    let properties = Column::new()
        .push(size)
        .padding(10)
        .width(Length::Units(225));
    return properties;
}

pub struct PropertyStyle;

impl container::StyleSheet for PropertyStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: None,
            background: Some(Color::from_rgb8(25, 25, 25).into()),
            border_radius: 0.0,
            border_width: 0.0,
            ..container::Style::default()
        }
    }
}
