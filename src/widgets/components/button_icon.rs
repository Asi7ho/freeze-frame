use iced::{button, Button, Color, Element};

use crate::FreezeFrameMessage;

pub struct WButtonIcon<'a> {
    pub widget: Button<'a, FreezeFrameMessage>,
}

impl<'a> WButtonIcon<'a> {
    pub fn new<E>(
        state: &'a mut button::State,
        content: E,
        message: FreezeFrameMessage,
        style: WButtonIconStyle,
    ) -> Self
    where
        E: Into<Element<'a, FreezeFrameMessage>>,
    {
        let widget = Button::new(state, content).on_press(message).style(style);
        Self { widget }
    }
}

pub struct WButtonIconStyle {
    pub background: Color,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
}

impl button::StyleSheet for WButtonIconStyle {
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
