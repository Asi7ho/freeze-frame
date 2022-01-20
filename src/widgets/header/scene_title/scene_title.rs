use iced::{text_input, Element, Length, TextInput};

use crate::{
    widgets::header::{HeaderMessage, HeaderStyle},
    FreezeFrameMessage, InteractionMessage,
};

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

#[derive(Debug, Default, Clone)]
pub struct TitleControls {
    pub scene_title_state: text_input::State,
}

impl TitleControls {
    pub fn view(&mut self, current_title: String) -> Element<FreezeFrameMessage> {
        let TitleControls { scene_title_state } = self;

        let scene_title = SceneTitle::new(&current_title, scene_title_state, |s| {
            FreezeFrameMessage::Interaction(InteractionMessage::HeaderInteraction(
                HeaderMessage::SceneTitleChange(s),
            ))
        })
        .scene_title
        .size(26)
        .padding(10)
        .width(Length::Fill)
        .style(HeaderStyle);

        return scene_title.into();
    }
}
