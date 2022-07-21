use std::ops::RangeInclusive;

use iced::{
    pure::{
        widget::{Button, Column, Row, Slider, TextInput},
        Element,
    },
    Color, Length,
};

use crate::message::{FreezeFrameMessage, HeaderMessage};

use super::style::{WButtonState, WButtonStyle, WSliderStyle, WTextInputStyle};

// Button (Color)
pub struct WButtonColor<'a> {
    pub widget: Button<'a, FreezeFrameMessage>,
}

impl<'a> WButtonColor<'a> {
    pub fn new(message: FreezeFrameMessage, size: u16, style: WButtonStyle) -> Self {
        let widget = Button::new("")
            .on_press(message)
            .height(Length::Units(size))
            .width(Length::Units(size))
            .style(style);
        Self { widget }
    }
}

// Button (Icon)
pub struct WButtonIcon<'a> {
    pub widget: Button<'a, FreezeFrameMessage>,
}

impl<'a> WButtonIcon<'a> {
    pub fn new<E>(content: E, message: FreezeFrameMessage, style: WButtonStyle) -> Self
    where
        E: Into<Element<'a, FreezeFrameMessage>>,
    {
        let widget = Button::new(content).on_press(message).style(style);
        Self { widget }
    }
}

// Color Palette
pub struct WColorPalette<'a> {
    pub widget: Column<'a, FreezeFrameMessage>,
}

impl<'a> WColorPalette<'a> {
    pub fn new(colors: Vec<Color>, chunk_size: usize, selected_color_id: (usize, usize)) -> Self {
        let widget = Column::with_children(
            colors
                .as_slice()
                .chunks(chunk_size)
                .collect::<Vec<_>>()
                .into_iter()
                .enumerate()
                .map(|(n_row, item)| {
                    Row::with_children(
                        item.into_iter()
                            .enumerate()
                            .map(|(n_col, color)| {
                                let color_style = WButtonStyle {
                                    state: WButtonState::ColorButton(*color),
                                };

                                let mut size = 20;
                                if (n_row, n_col) == selected_color_id {
                                    size = 25;
                                }

                                let message = FreezeFrameMessage::HeaderInteraction(
                                    HeaderMessage::ChangeColor((n_row, n_col)),
                                );

                                WButtonColor::new(message, size, color_style)
                                    .widget
                                    .padding(10)
                                    .into()
                            })
                            .collect(),
                    )
                    .spacing(5)
                    .into()
                })
                .collect(),
        );

        Self { widget }
    }
}

// Slider
pub struct WSlider<'a> {
    pub widget: Slider<'a, f32, FreezeFrameMessage>,
}

impl<'a> WSlider<'a> {
    pub fn new<F>(range: RangeInclusive<f32>, value: f32, message: F, style: WSliderStyle) -> Self
    where
        F: 'static + Fn(f32) -> FreezeFrameMessage,
    {
        let widget = Slider::new(range, value, message).style(style);
        Self { widget }
    }
}

// Text Input
pub struct WTextInput<'a> {
    pub widget: TextInput<'a, FreezeFrameMessage>,
}

impl<'a> WTextInput<'a> {
    pub fn new<F>(placeholder: &str, value: &str, message: F, style: WTextInputStyle) -> Self
    where
        F: 'static + Fn(String) -> FreezeFrameMessage,
    {
        let widget = TextInput::new(placeholder, value, message).style(style);
        Self { widget }
    }
}
