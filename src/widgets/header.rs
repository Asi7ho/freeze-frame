use iced::{
    pure::{
        widget::{Button, Container, Row, Scrollable, TextInput},
        Element,
    },
    svg, Alignment, Color, Length, Svg,
};

use crate::{
    message::{FreezeFrameMessage, HeaderMessage},
    utils::svg::{
        ADD, BOTTOM_ARROW, BRUSH, ERASER, FILL, GEOMETRY, GRID, ICON_SIZE, POINTER, TEXT, TRASH,
    },
};

use super::{
    components::ColorPalette,
    style::{
        WButtonState, WButtonStyle, WContainerState, WContainerStyle, WTextInputState,
        WTextInputStyle,
    },
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BrushFilter {
    #[default]
    Pointer,
    Brush,
    Eraser,
    Geometry,
    Text,
    Fill,
}

#[derive(Debug, Clone)]
pub struct Colors {
    pub colors: Vec<Color>,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            colors: vec![Color::BLACK],
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ExtraFilter {
    Grid,
    Trash,
    #[default]
    Ignore,
}

#[derive(Debug, Default)]
pub struct HeaderState {
    pub scene_title_input: String,
    pub brush_filter: BrushFilter,
    pub brush_color_id: (usize, usize),
    pub color_palette: Colors,
    pub color_scroll_offset: f32,
    pub extra_filter: ExtraFilter,
}

pub fn view(header_state: &HeaderState) -> Element<FreezeFrameMessage> {
    let create_icon = |icon_byte| {
        let handle = svg::Handle::from_memory(icon_byte);
        Svg::new(handle)
            .height(Length::Units(ICON_SIZE))
            .width(Length::Units(ICON_SIZE))
    };

    // Scene title
    let scene_title = TextInput::new("Scene Title", &header_state.scene_title_input, |s| {
        FreezeFrameMessage::HeaderInteraction(HeaderMessage::SceneTitleChange(s))
    })
    .style(WTextInputStyle {
        state: WTextInputState::SceneTitle,
    })
    .size(26)
    .padding(10)
    .width(Length::Units(250));

    // Brush tools
    let controller_button = |icon_byte, filter, current_filter| {
        let icon = create_icon(icon_byte);
        let mut state = WButtonState::IconNotSelected;
        if filter == current_filter {
            state = WButtonState::IconSelected;
        }

        Button::new(icon)
            .on_press(FreezeFrameMessage::HeaderInteraction(
                HeaderMessage::BrushControlsChange(filter),
            ))
            .style(WButtonStyle { state })
            .padding(10)
    };

    let brush_tools = Container::new(
        Row::new()
            .spacing(5)
            .push(controller_button(
                POINTER,
                BrushFilter::Pointer,
                header_state.brush_filter,
            ))
            .push(controller_button(
                BRUSH,
                BrushFilter::Brush,
                header_state.brush_filter,
            ))
            .push(controller_button(
                ERASER,
                BrushFilter::Eraser,
                header_state.brush_filter,
            ))
            .push(controller_button(
                GEOMETRY,
                BrushFilter::Geometry,
                header_state.brush_filter,
            ))
            .push(controller_button(
                TEXT,
                BrushFilter::Text,
                header_state.brush_filter,
            ))
            .push(controller_button(
                FILL,
                BrushFilter::Fill,
                header_state.brush_filter,
            ))
            .align_items(Alignment::Center),
    );

    // Color palette
    let controllers_button = |icon_byte, message| {
        let icon = create_icon(icon_byte);

        Button::new(icon)
            .on_press(message)
            .style(WButtonStyle {
                state: WButtonState::IconNotSelected,
            })
            .padding(10)
    };

    let color_palette = ColorPalette::new(
        header_state.color_palette.colors.clone(),
        5,
        header_state.brush_color_id,
    );

    let color_tools = Container::new(
        Row::new()
            .spacing(5)
            .push(controllers_button(
                BOTTOM_ARROW,
                FreezeFrameMessage::HeaderInteraction(HeaderMessage::ChangePalette),
            ))
            .push(
                Scrollable::new(color_palette.widget.spacing(8)).on_scroll(move |offset| {
                    FreezeFrameMessage::HeaderInteraction(HeaderMessage::Scrolled(offset))
                }),
            )
            .push(controllers_button(
                ADD,
                FreezeFrameMessage::HeaderInteraction(HeaderMessage::AddColor),
            ))
            .width(Length::Units(225))
            .align_items(Alignment::Center),
    );

    // Grid tools
    let controller_button = |icon_byte, filter, current_filter| {
        let icon = create_icon(icon_byte);
        let mut state = WButtonState::IconNotSelected;
        if filter == current_filter {
            state = WButtonState::IconSelected;
        }

        Button::new(icon)
            .on_press(FreezeFrameMessage::HeaderInteraction(
                HeaderMessage::GridToolSelected(filter),
            ))
            .style(WButtonStyle { state })
            .padding(10)
    };

    let extra_tool = Container::new(
        Row::new()
            .spacing(5)
            .push(controller_button(
                GRID,
                ExtraFilter::Grid,
                header_state.extra_filter,
            ))
            .push(controller_button(
                TRASH,
                ExtraFilter::Trash,
                header_state.extra_filter,
            ))
            .align_items(Alignment::Center),
    );

    // Put everything together
    let header = Container::new(
        Row::new()
            .spacing(50)
            .push(scene_title)
            .push(brush_tools)
            .push(color_tools)
            .push(extra_tool)
            .align_items(Alignment::Center),
    )
    .height(Length::Units(45))
    .width(Length::Fill)
    .style(WContainerStyle {
        state: WContainerState::Header,
    });

    return header.into();
}
