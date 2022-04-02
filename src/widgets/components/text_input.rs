use iced::{
    pure::widget::{text_input, TextInput},
    Color,
};

use crate::FreezeFrameMessage;

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

pub struct WTextInputStyle {
    pub background: Color,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
    pub placeholder_color: Color,
    pub value_color: Color,
    pub selection_color: Color,
}

impl text_input::StyleSheet for WTextInputStyle {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: self.background.into(),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color,
        }
    }

    fn focused(&self) -> text_input::Style {
        self.active()
    }

    fn placeholder_color(&self) -> Color {
        self.placeholder_color
    }

    fn value_color(&self) -> Color {
        self.value_color
    }

    fn selection_color(&self) -> Color {
        self.selection_color
    }
}
