use iced::{text_input, TextInput};

use crate::FreezeFrameMessage;

pub struct SceneTitle<'a> {
    pub scene_title: TextInput<'a, FreezeFrameMessage>,
}

impl<'a> SceneTitle<'a> {
    pub fn new<F>(value: &str, state: &'a mut text_input::State, on_change: F) -> Self
    where
        F: 'static + Fn(String) -> FreezeFrameMessage,
    {
        return Self {
            scene_title: TextInput::new(state, "Scene title", value, move |s| on_change(s)),
        };
    }
}
