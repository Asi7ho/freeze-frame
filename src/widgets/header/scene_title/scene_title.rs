use iced::{text_input, Element, Length, TextInput};

use crate::{
    widgets::header::{HeaderMessage, HeaderStyle},
    FreezeFrameMessage, InteractionMessage,
};

#[derive(Debug, Default, Clone)]
pub struct TitleControls {
    pub scene_title_state: text_input::State,
}

impl TitleControls {
    pub fn view(&mut self, current_title: String) -> Element<FreezeFrameMessage> {
        let TitleControls { scene_title_state } = self;

        let scene_title = TextInput::new(scene_title_state, "Scene title", &current_title, |s| {
            FreezeFrameMessage::Interaction(InteractionMessage::HeaderInteraction(
                HeaderMessage::SceneTitleChange(s),
            ))
        })
        .size(26)
        .padding(10)
        .width(Length::Fill)
        .style(HeaderStyle);

        return scene_title.into();
    }
}
