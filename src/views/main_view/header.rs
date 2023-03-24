use iced::{
    theme,
    widget::{button, container, row, scrollable, svg, text_input, Svg},
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
    let content = row![
        create_scene_tile(header_state),
        create_brush_tools(header_state),
        create_color_tools(header_state),
        create_ui_control_tool(header_state)
    ]
    .spacing(50)
    .align_items(Alignment::Center);

    container(content)
        .height(Length::Fixed(45.0))
        .width(Length::Fill)
        .style(theme::Container::Custom(Box::new(HeaderStyle)))
        .into()
}

fn _message(message: HeaderMessage) -> FreezeFrameMessage {
    FreezeFrameMessage::MainView(MainViewMessage::Header(message))
}

fn create_scene_tile(header_state: &HeaderState) -> Element<FreezeFrameMessage> {
    Element::from(
        text_input("Scene Title", &header_state.scene_title_input, |s| {
            _message(HeaderMessage::ChangeSceneTitle(s))
        })
        .style(theme::TextInput::Custom(Box::new(SceneTitleStyle)))
        .size(26)
        .padding(10)
        .width(Length::Fixed(250.0)),
    )
}

fn create_icon(icon_byte: &'static [u8]) -> Svg<iced::Renderer> {
    let handle = svg::Handle::from_memory(icon_byte);

    svg(handle)
        .height(Length::Fixed(ICON_SIZE))
        .width(Length::Fixed(ICON_SIZE))
}

fn create_brush_tools(header_state: &HeaderState) -> Element<FreezeFrameMessage> {
    let current_filter = header_state.brush_filter;

    let create_brush_button = |icon_byte, filter| {
        let icon = create_icon(icon_byte);
        let selected = filter == current_filter;
        let message = |f| _message(HeaderMessage::ChangeBrushControls(f));

        button(icon)
            .on_press(message(filter))
            .style(theme::Button::Custom(Box::new(IconStyle { selected })))
            .padding(10)
    };

    let content = row![
        create_brush_button(POINTER, BrushFilter::Pointer),
        create_brush_button(BRUSH, BrushFilter::Brush),
        create_brush_button(ERASER, BrushFilter::Eraser),
        create_brush_button(GEOMETRY, BrushFilter::Geometry),
        create_brush_button(TEXT, BrushFilter::Text),
        create_brush_button(FILL, BrushFilter::Fill)
    ]
    .align_items(Alignment::Center);

    Element::from(container(content))
}

fn create_color_palette(header_state: &HeaderState) -> Element<FreezeFrameMessage> {
    ColorPalette::new(
        header_state.color_palette.clone(),
        header_state.brush_color_id,
        |n| _message(HeaderMessage::ChangeColor(n)),
    )
    .widget
    .spacing(8)
    .padding(5)
    .into()
}

fn create_color_tools(header_state: &HeaderState) -> Element<FreezeFrameMessage> {
    let create_control_button = |icon_byte, message| {
        let icon = create_icon(icon_byte);

        button(icon)
            .on_press(message)
            .style(theme::Button::Custom(Box::new(IconStyle {
                selected: false,
            })))
            .padding(10)
    };

    let content = row![
        create_control_button(BOTTOM_ARROW, _message(HeaderMessage::ChangePalette)),
        scrollable(create_color_palette(header_state))
            .horizontal_scroll(scrollable::Properties::new().width(1).scroller_width(1))
            .on_scroll(|s| _message(HeaderMessage::Scroll(s))),
        create_control_button(ADD, _message(HeaderMessage::AddColor))
    ]
    .spacing(5)
    .padding(5)
    .width(Length::Fixed(225.0))
    .align_items(Alignment::Center);

    Element::from(container(content))
}

fn create_ui_control_tool(header_state: &HeaderState) -> Element<FreezeFrameMessage> {
    let current_filter = header_state.ui_control_filter;

    let create_ui_button = |icon_byte, filter| {
        let icon = create_icon(icon_byte);
        let message = _message(HeaderMessage::SelectGridTool(filter));

        button(icon)
            .on_press(message)
            .style(theme::Button::Custom(Box::new(IconStyle {
                selected: filter == current_filter,
            })))
            .padding(10)
    };

    let content = row![
        create_ui_button(GRID, UiControlFilter::Grid),
        create_ui_button(TRASH, UiControlFilter::Trash)
    ]
    .align_items(Alignment::Center);

    Element::from(container(content))
}
