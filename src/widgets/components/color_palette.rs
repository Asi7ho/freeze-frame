use super::button_color::{WButtonColor, WButtonColorStyle};
use iced::{button, Color, Column, Row};

use crate::{widgets::header::HeaderMessage, FreezeFrameMessage};

pub struct WColorPalette<'a> {
    pub widget: Column<'a, FreezeFrameMessage>,
}

impl<'a> WColorPalette<'a> {
    pub fn new(
        color_states: &'a mut Vec<button::State>,
        colors: Vec<Color>,
        chunk_size: usize,
        selected_color_id: (usize, usize),
    ) -> Self {
        color_states.resize_with(colors.len(), Default::default);
        let states = color_states
            .as_mut_slice()
            .chunks_mut(chunk_size)
            .collect::<Vec<_>>();

        let widget = Column::with_children(
            colors
                .as_slice()
                .chunks(chunk_size)
                .collect::<Vec<_>>()
                .into_iter()
                .zip(states)
                .enumerate()
                .map(|(n_row, item)| {
                    Row::with_children(
                        item.0
                            .into_iter()
                            .zip(item.1)
                            .enumerate()
                            .map(|(n_col, (color, state))| {
                                let color_style = WButtonColorStyle {
                                    background: *color,
                                    border_radius: 8.0,
                                    border_width: 2.5,
                                    border_color: Color::from_rgba8(187, 182, 197, 0.35),
                                };

                                let mut size = 21;
                                if (n_row, n_col) == selected_color_id {
                                    size = 26;
                                }

                                let message = FreezeFrameMessage::HeaderInteraction(
                                    HeaderMessage::ChangeColor((n_row, n_col)),
                                );

                                WButtonColor::new(state, message, size, color_style)
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
