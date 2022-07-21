use iced::{
    pure::widget::{Button, Column, Row},
    Color, Length,
};

use crate::message::{FreezeFrameMessage, HeaderMessage};

use super::style::{WButtonState, WButtonStyle};

// Color Palette
pub struct ColorPalette<'a> {
    pub widget: Column<'a, FreezeFrameMessage>,
}

impl<'a> ColorPalette<'a> {
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
                                let mut size = 20;
                                if (n_row, n_col) == selected_color_id {
                                    size = 25;
                                }

                                let message = FreezeFrameMessage::HeaderInteraction(
                                    HeaderMessage::ChangeColor((n_row, n_col)),
                                );

                                Button::new("")
                                    .on_press(message)
                                    .height(Length::Units(size))
                                    .width(Length::Units(size))
                                    .style(WButtonStyle {
                                        state: WButtonState::ColorButton(*color),
                                    })
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
