use iced::{
    theme,
    widget::{button, Row},
    Color, Length,
};

use crate::{styles::ColorButtonStyle, FreezeFrameMessage};

// Color Palette
pub struct ColorPalette<'a> {
    pub widget: Row<'a, FreezeFrameMessage>,
}

impl<'a> ColorPalette<'a> {
    pub fn new<F>(colors: Vec<Color>, selected_color_id: usize, message: F) -> Self
    where
        F: 'a + Fn(usize) -> FreezeFrameMessage,
    {
        log::info!("Color palette size: {:?}", colors.len());

        let palette = colors
            .into_iter()
            .enumerate()
            .map(|(n, color)| {
                let mut size = 20.0;
                if n == selected_color_id {
                    size = 25.0;
                }

                button("")
                    .on_press(message(n))
                    .height(Length::Fixed(size))
                    .width(Length::Fixed(size))
                    .style(theme::Button::Custom(Box::new(ColorButtonStyle { color })))
                    .padding(10)
                    .into()
            })
            .collect();

        let widget = Row::with_children(palette)
            .spacing(5)
            .width(Length::Fill)
            .into();

        Self { widget }
    }
}
