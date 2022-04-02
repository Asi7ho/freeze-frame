use iced::{
    pure::widget::{button, Button},
    Color, Length,
};

use crate::FreezeFrameMessage;

pub struct WButtonColor<'a> {
    pub widget: Button<'a, FreezeFrameMessage>,
}

impl<'a> WButtonColor<'a> {
    pub fn new(message: FreezeFrameMessage, size: u16, style: WButtonColorStyle) -> Self {
        let widget = Button::new("")
            .on_press(message)
            .height(Length::Units(size))
            .width(Length::Units(size))
            .style(style);
        Self { widget }
    }
}

pub struct WButtonColorStyle {
    pub background: Color,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
}

impl button::StyleSheet for WButtonColorStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Default::default(),
            background: Some(self.background.into()),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color,
            text_color: Color::TRANSPARENT,
        }
    }
}
