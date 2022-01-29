use iced::{Alignment, Container, Element, Length, Row};

use crate::widgets::header::brush_tools::{BrushControls, BrushFilter};
use crate::widgets::header::color_palette::{ColorPalette, PaletteControls};
use crate::widgets::header::extra_tools::{ExtraControl, ExtraFilter};
use crate::widgets::header::scene_title::TitleControls;
use crate::FreezeFrameMessage;

use crate::widgets::header::HeaderStyle;

#[derive(Debug, Default)]
pub struct HeaderState {
    pub scene_title_input: String,
    pub scene_title_controls: TitleControls,
    pub brush_filter: BrushFilter,
    pub brush_controls: BrushControls,
    pub brush_color_id: (usize, usize),
    pub color_palette: ColorPalette,
    pub color_controls: PaletteControls,
    pub color_scroll_offset: f32,
    pub extra_filter: ExtraFilter,
    pub extra_control: ExtraControl,
}

#[derive(Debug, Clone)]
pub enum HeaderMessage {
    SceneTitleChange(String),
    BrushControlsChange(BrushFilter),
    ExtraToolSelected(ExtraFilter),
    ChangePalette,
    ChangeColor((usize, usize)),
    AddColor,
    Scrolled(f32),
}

pub fn view(header_state: &mut HeaderState) -> Element<FreezeFrameMessage> {
    // Scene title
    let scene_title = header_state
        .scene_title_controls
        .view(header_state.scene_title_input.clone());

    // Brush tools
    let brush_tools = header_state.brush_controls.view(header_state.brush_filter);

    // Color palette
    let color_tools = header_state.color_controls.view(
        header_state.color_palette.colors.clone(),
        header_state.brush_color_id,
    );

    // Extra tools
    let extra_tools = header_state.extra_control.view(header_state.extra_filter);

    let header = Container::new(
        Row::new()
            .spacing(50)
            .push(scene_title)
            .push(brush_tools)
            .push(color_tools)
            .push(extra_tools)
            .align_items(Alignment::Center),
    )
    .height(Length::Units(45))
    .width(Length::Fill)
    .style(HeaderStyle);

    return header.into();
}
