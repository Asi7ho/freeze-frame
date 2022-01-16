use iced::{text_input, Container, Element, Length, Row};

use crate::widgets::header::scene_title::SceneTitle;
use crate::{FreezeFrameMessage, InteractionMessage};

use crate::widgets::header::HeaderStyle;

#[derive(Debug, Default)]
pub struct HeaderState {
    pub scene_title_input: String,
    pub scene_title_state: text_input::State,
}

#[derive(Debug, Clone)]
pub enum HeaderMessage {
    SceneTitleChange(String),
}

pub fn view(header_state: &mut HeaderState) -> Element<FreezeFrameMessage> {
    let scene_title = SceneTitle::new(
        &header_state.scene_title_input,
        &mut header_state.scene_title_state,
        |s| {
            FreezeFrameMessage::Interaction(InteractionMessage::HeaderInteraction(
                HeaderMessage::SceneTitleChange(s),
            ))
        },
    )
    .scene_title
    .size(26)
    .padding(10)
    .width(Length::Fill)
    .style(HeaderStyle);

    let header = Container::new(Row::new().push(scene_title)).style(HeaderStyle);

    return header.into();
}
