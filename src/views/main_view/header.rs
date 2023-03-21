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

fn create_scene_tile(header_state: &HeaderState) -> Element<FreezeFrameMessage> {
    let scene_title = &header_state.scene_title_input;
    let message = |s| {
        FreezeFrameMessage::MainView(MainViewMessage::Header(HeaderMessage::ChangeSceneTitle(s)))
    };

    text_input("Scene Title", scene_title, message)
        .style(theme::TextInput::Custom(Box::new(SceneTitleStyle)))
        .size(26)
        .padding(10)
        .width(Length::Fixed(250.0))
        .into()
}

fn create_icon(icon_byte: &'static [u8]) -> Svg<iced::Renderer> {
    let handle = svg::Handle::from_memory(icon_byte);

    svg(handle)
        .height(Length::Fixed(ICON_SIZE))
        .width(Length::Fixed(ICON_SIZE))
}

fn create_brush_tools(header_state: &HeaderState) -> Element<FreezeFrameMessage> {
    let button = |icon_byte, filter, current_filter| {
        let icon = create_icon(icon_byte);
        let selected = filter == current_filter;
        let message = |f| {
            FreezeFrameMessage::MainView(MainViewMessage::Header(
                HeaderMessage::ChangeBrushControls(f),
            ))
        };

        button(icon)
            .on_press(message(filter))
            .style(theme::Button::Custom(Box::new(IconStyle { selected })))
            .padding(10)
    };

    let content = row![
        button(POINTER, BrushFilter::Pointer, header_state.brush_filter,),
        button(BRUSH, BrushFilter::Brush, header_state.brush_filter),
        button(ERASER, BrushFilter::Eraser, header_state.brush_filter),
        button(GEOMETRY, BrushFilter::Geometry, header_state.brush_filter),
        button(TEXT, BrushFilter::Text, header_state.brush_filter),
        button(FILL, BrushFilter::Fill, header_state.brush_filter)
    ]
    .align_items(Alignment::Center);

    container(content).into()
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

fn create_color_tools(header_state: &HeaderState) -> Element<FreezeFrameMessage> {
    let get_buttton = |icon_byte, message| {
        let icon = create_icon(icon_byte);

        button(icon)
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

    let content = row![
        get_buttton(BOTTOM_ARROW, arrow_message),
        scrollable(color_palette.widget.spacing(8).padding(5))
            .horizontal_scroll(scrollable::Properties::new().width(1).scroller_width(1))
            .on_scroll(scroll_message),
        get_buttton(ADD, add_message)
    ]
    .spacing(5)
    .padding(5)
    .width(Length::Fixed(225.0))
    .align_items(Alignment::Center);

    container(content).into()
}

fn create_ui_control_tool(header_state: &HeaderState) -> Element<FreezeFrameMessage> {
    let get_button = |icon_byte, filter, current_filter| {
        let icon = create_icon(icon_byte);

        let message = FreezeFrameMessage::MainView(MainViewMessage::Header(
            HeaderMessage::SelectGridTool(filter),
        ));

        button(icon)
            .on_press(message)
            .style(theme::Button::Custom(Box::new(IconStyle {
                selected: filter == current_filter,
            })))
            .padding(10)
    };

    let content = row![
        get_button(GRID, UiControlFilter::Grid, header_state.ui_control_filter,),
        get_button(
            TRASH,
            UiControlFilter::Trash,
            header_state.ui_control_filter,
        )
    ]
    .align_items(Alignment::Center);

    container(content).into()
}
