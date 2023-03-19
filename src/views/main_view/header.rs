use iced::{
    theme,
    widget::{scrollable, svg, Button, Container, Row, Scrollable, Svg, TextInput},
    Alignment, Color, Element, Length,
};

use crate::{
    styles::{HeaderStyle, IconStyle, SceneTitleStyle},
    tools::filters::{BrushFilter, UiControlFilter},
    utils::{
        colors,
        svg::{
            ADD, BOTTOM_ARROW, BRUSH, ERASER, FILL, GEOMETRY, GRID, ICON_SIZE, POINTER, TEXT, TRASH,
        },
    },
    widgets::ColorPalette,
    FreezeFrameMessage,
};

use super::{HeaderMessage, MainViewMessage};

#[derive(Debug)]
pub struct HeaderState {
    pub scene_title_input: String,
    pub brush_filter: BrushFilter,
    pub brush_color_id: usize,
    pub color_palette: Vec<Color>,
    pub color_scroll_offset: scrollable::RelativeOffset,
    pub ui_control_filter: UiControlFilter,
}

impl Default for HeaderState {
    fn default() -> Self {
        Self {
            scene_title_input: String::default(),
            brush_filter: BrushFilter::default(),
            brush_color_id: 0,
            color_palette: vec![
                Color::BLACK,
                colors::generate_color(),
                colors::generate_color(),
                colors::generate_color(),
                colors::generate_color(),
            ],
            color_scroll_offset: scrollable::RelativeOffset::START,
            ui_control_filter: UiControlFilter::default(),
        }
    }
}

pub fn view(header_state: &HeaderState) -> Element<FreezeFrameMessage> {
    let header = Container::new(
        Row::new()
            .spacing(50)
            .push(create_scene_tile(header_state))
            .push(create_brush_tools(header_state))
            .push(create_color_tools(header_state))
            .push(create_ui_control_tool(header_state))
            .align_items(Alignment::Center),
    )
    .height(Length::Fixed(45.0))
    .width(Length::Fill)
    .style(theme::Container::Custom(Box::new(HeaderStyle)));

    header.into()
}

fn create_scene_tile(header_state: &HeaderState) -> TextInput<FreezeFrameMessage> {
    let scene_title = &header_state.scene_title_input;
    let message = |s| {
        FreezeFrameMessage::MainView(MainViewMessage::Header(HeaderMessage::ChangeSceneTitle(s)))
    };

    TextInput::new("Scene Title", scene_title, message)
        .style(theme::TextInput::Custom(Box::new(SceneTitleStyle)))
        .size(26)
        .padding(10)
        .width(Length::Fixed(250.0))
}

fn create_icon(icon_byte: &'static [u8]) -> Svg<iced::Renderer> {
    let handle = svg::Handle::from_memory(icon_byte);

    Svg::new(handle)
        .height(Length::Fixed(ICON_SIZE))
        .width(Length::Fixed(ICON_SIZE))
}

fn create_brush_tools(header_state: &HeaderState) -> Container<FreezeFrameMessage> {
    let button = |icon_byte, filter, current_filter| {
        let icon = create_icon(icon_byte);

        Button::new(icon)
            .on_press(FreezeFrameMessage::MainView(MainViewMessage::Header(
                HeaderMessage::ChangeBrushControls(filter),
            )))
            .style(theme::Button::Custom(Box::new(IconStyle {
                selected: filter == current_filter,
            })))
            .padding(10)
    };

    Container::new(
        Row::new()
            .spacing(5)
            .push(button(
                POINTER,
                BrushFilter::Pointer,
                header_state.brush_filter,
            ))
            .push(button(BRUSH, BrushFilter::Brush, header_state.brush_filter))
            .push(button(
                ERASER,
                BrushFilter::Eraser,
                header_state.brush_filter,
            ))
            .push(button(
                GEOMETRY,
                BrushFilter::Geometry,
                header_state.brush_filter,
            ))
            .push(button(TEXT, BrushFilter::Text, header_state.brush_filter))
            .push(button(FILL, BrushFilter::Fill, header_state.brush_filter))
            .align_items(Alignment::Center),
    )
}

fn create_color_palette(header_state: &HeaderState) -> ColorPalette {
    let message =
        |n| FreezeFrameMessage::MainView(MainViewMessage::Header(HeaderMessage::ChangeColor(n)));

    ColorPalette::new(
        header_state.color_palette.clone(),
        header_state.brush_color_id,
        message,
    )
}

fn create_color_tools(header_state: &HeaderState) -> Container<FreezeFrameMessage> {
    let button = |icon_byte, message| {
        let icon = create_icon(icon_byte);

        Button::new(icon)
            .on_press(message)
            .style(theme::Button::Custom(Box::new(IconStyle {
                selected: false,
            })))
            .padding(10)
    };

    let color_palette = create_color_palette(header_state);
    let arrow_message =
        FreezeFrameMessage::MainView(MainViewMessage::Header(HeaderMessage::ChangePalette));
    let scroll_message =
        move |s| FreezeFrameMessage::MainView(MainViewMessage::Header(HeaderMessage::Scroll(s)));
    let add_message =
        FreezeFrameMessage::MainView(MainViewMessage::Header(HeaderMessage::AddColor));

    Container::new(
        Row::new()
            .spacing(5)
            .push(button(BOTTOM_ARROW, arrow_message))
            .push(
                Scrollable::new(color_palette.widget.spacing(8).padding(5))
                    .horizontal_scroll(scrollable::Properties::new().width(1).scroller_width(1))
                    .on_scroll(scroll_message),
            )
            .padding(5)
            .push(button(ADD, add_message))
            .width(Length::Fixed(225.0))
            .align_items(Alignment::Center),
    )
}

fn create_ui_control_tool(header_state: &HeaderState) -> Container<FreezeFrameMessage> {
    let button = |icon_byte, filter, current_filter| {
        let icon = create_icon(icon_byte);

        let message = FreezeFrameMessage::MainView(MainViewMessage::Header(
            HeaderMessage::SelectGridTool(filter),
        ));

        Button::new(icon)
            .on_press(message)
            .style(theme::Button::Custom(Box::new(IconStyle {
                selected: filter == current_filter,
            })))
            .padding(10)
    };

    Container::new(
        Row::new()
            .spacing(5)
            .push(button(
                GRID,
                UiControlFilter::Grid,
                header_state.ui_control_filter,
            ))
            .push(button(
                TRASH,
                UiControlFilter::Trash,
                header_state.ui_control_filter,
            ))
            .align_items(Alignment::Center),
    )
}
