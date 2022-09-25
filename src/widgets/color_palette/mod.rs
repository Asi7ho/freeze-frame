use iced::{
    pure::widget::{Button, Column, Row},
    Color, Length,
};

use crate::{
    styles::{ButtonState, ButtonStyle},
    FreezeFrameMessage,
};

// Color Palette
pub struct ColorPalette<'a> {
    pub widget: Column<'a, FreezeFrameMessage>,
}

impl<'a> ColorPalette<'a> {
    pub fn new<F>(
        colors: Vec<Color>,
        chunk_size: usize,
        selected_color_id: (usize, usize),
        message: F,
    ) -> Self
    where
        F: 'a + Fn(usize, usize) -> FreezeFrameMessage,
    {
        let widget = Column::with_children(
            colors
                .as_slice()
                .chunks(chunk_size)
                .collect::<Vec<_>>()
                .into_iter()
                .enumerate()
                .map(|(n_row, item)| {
                    Row::with_children(
                        item.iter()
                            .enumerate()
                            .map(|(n_col, color)| {
                                let mut size = 20;
                                if (n_row, n_col) == selected_color_id {
                                    size = 25;
                                }

                                Button::new("")
                                    .on_press(message(n_row, n_col))
                                    .height(Length::Units(size))
                                    .width(Length::Units(size))
                                    .style(ButtonStyle {
                                        state: ButtonState::ColorButton(*color),
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
