use iced::{Align, Container, Element, Row};

use crate::widgets::header::brush_tools::{BrushControls, BrushFilter};
use crate::widgets::header::scene_title::TitleControls;
use crate::FreezeFrameMessage;

use crate::widgets::header::HeaderStyle;

#[derive(Debug, Default)]
pub struct HeaderState {
    pub scene_title_input: String,
    pub scene_title_controls: TitleControls,
    pub brush_filter: BrushFilter,
    pub brush_controls: BrushControls,
}

#[derive(Debug, Clone)]
pub enum HeaderMessage {
    SceneTitleChange(String),
    BrushControlsChange(BrushFilter),
}

pub fn view(header_state: &mut HeaderState) -> Element<FreezeFrameMessage> {
    // Scene title
    let scene_title = header_state
        .scene_title_controls
        .view(header_state.scene_title_input.clone());

    // Brush tools
    let brush_tools = header_state.brush_controls.view(header_state.brush_filter);

    // Color palette

    // Extra tools

    let header = Container::new(
        Row::new()
            .push(scene_title)
            .push(brush_tools)
            .align_items(Align::Center),
    )
    .style(HeaderStyle);

    return header.into();
}
