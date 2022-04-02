use super::button_color::{WButtonColor, WButtonColorStyle};

use iced::{
    pure::widget::{Column, Row},
    Color,
};

use crate::{widgets::header::HeaderMessage, FreezeFrameMessage};

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
                                let color_style = WButtonColorStyle {
                                    background: *color,
                                    border_radius: 8.0,
                                    border_width: 2.5,
                                    border_color: Color::from_rgba8(187, 182, 197, 0.35),
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
