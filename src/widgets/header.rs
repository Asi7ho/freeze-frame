use iced::{
    button, container, scrollable, svg, text_input, Alignment, Color, Container, Element, Length,
    Row, Scrollable, Svg,
};

use crate::widgets::components::button_icon::{WButtonIcon, WButtonIconStyle};
use crate::widgets::components::color_palette::WColorPalette;
use crate::widgets::components::text_input::{WTextInput, WTextInputStyle};
use crate::FreezeFrameMessage;

use crate::utils::svg::{
    ADD, BOTTOM_ARROW, BRUSH, ERASER, FILL, GEOMETRY, GRID, ICON_SIZE, POINTER, TEXT,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrushFilter {
    Pointer,
    Brush,
    Eraser,
    Geometry,
    Text,
    Fill,
}

impl Default for BrushFilter {
    fn default() -> Self {
        BrushFilter::Pointer
    }
}

#[derive(Debug, Default, Clone)]
pub struct BrushStates {
    pub pointer_state: button::State,
    pub brush_state: button::State,
    pub eraser_state: button::State,
    pub geometry_state: button::State,
    pub text_state: button::State,
    pub fill_state: button::State,
}

#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub colors: Vec<Color>,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            colors: vec![Color::BLACK],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridFilter {
    Grid,
    Ignore,
}

impl Default for GridFilter {
    fn default() -> Self {
        GridFilter::Ignore
    }
}

#[derive(Debug, Default)]
pub struct HeaderState {
    pub scene_title_input: String,
    pub scene_title_state: text_input::State,
    pub brush_filter: BrushFilter,
    pub brush_states: BrushStates,
    pub brush_color_id: (usize, usize),
    pub choose_palette_state: button::State,
    pub color_palette: ColorPalette,
    pub color_states: Vec<button::State>,
    pub scrollbar_state: scrollable::State,
    pub color_scroll_offset: f32,
    pub add_color_state: button::State,
    pub grid_filter: GridFilter,
    pub grid_state: button::State,
}

#[derive(Debug, Clone)]
pub enum HeaderMessage {
    SceneTitleChange(String),
    BrushControlsChange(BrushFilter),
    GridToolSelected(GridFilter),
    ChangePalette,
    ChangeColor((usize, usize)),
    AddColor,
    Scrolled(f32),
}

pub fn view(header_state: &mut HeaderState) -> Element<FreezeFrameMessage> {
    let create_icon = |icon_byte| {
        let handle = svg::Handle::from_memory(icon_byte);
        let icon = Svg::new(handle)
            .height(Length::Units(ICON_SIZE))
            .width(Length::Units(ICON_SIZE));

        icon
    };

    let get_default_button_style = || WButtonIconStyle {
        background: Color::TRANSPARENT,
        border_radius: 10.0,
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
    };

    // Scene title
    let scene_title = WTextInput::new(
        &mut header_state.scene_title_state,
        "Scene Title",
        &header_state.scene_title_input,
        |s| FreezeFrameMessage::HeaderInteraction(HeaderMessage::SceneTitleChange(s)),
        WTextInputStyle {
            background: Color::from_rgb8(34, 34, 34),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            placeholder_color: Color::WHITE,
            value_color: Color::WHITE,
            selection_color: Color::from_rgb8(64, 64, 64),
        },
    )
    .widget
    .size(26)
    .padding(10)
    .width(Length::Units(250));

    // Brush tools
    let controller_button = |state, icon_byte, filter, current_filter| {
        let icon = create_icon(icon_byte);
        let mut style = get_default_button_style();
        if filter == current_filter {
            style.background = Color::from_rgba8(187, 182, 197, 0.15);
        }

        let button_icon = WButtonIcon::new(
            state,
            icon,
            FreezeFrameMessage::HeaderInteraction(HeaderMessage::BrushControlsChange(filter)),
            style,
        )
        .widget
        .padding(10);

        button_icon
    };

    let brush_tools = Container::new(
        Row::new()
            .spacing(5)
            .push(controller_button(
                &mut header_state.brush_states.pointer_state,
                POINTER,
                BrushFilter::Pointer,
                header_state.brush_filter,
            ))
            .push(controller_button(
                &mut header_state.brush_states.brush_state,
                BRUSH,
                BrushFilter::Brush,
                header_state.brush_filter,
            ))
            .push(controller_button(
                &mut header_state.brush_states.eraser_state,
                ERASER,
                BrushFilter::Eraser,
                header_state.brush_filter,
            ))
            .push(controller_button(
                &mut header_state.brush_states.geometry_state,
                GEOMETRY,
                BrushFilter::Geometry,
                header_state.brush_filter,
            ))
            .push(controller_button(
                &mut header_state.brush_states.text_state,
                TEXT,
                BrushFilter::Text,
                header_state.brush_filter,
            ))
            .push(controller_button(
                &mut header_state.brush_states.fill_state,
                FILL,
                BrushFilter::Fill,
                header_state.brush_filter,
            ))
            .align_items(Alignment::Center),
    );

    // Color palette
    let controllers_button = |state, icon_byte, message| {
        let icon = create_icon(icon_byte);
        let style = get_default_button_style();

        let button_icon = WButtonIcon::new(state, icon, message, style)
            .widget
            .padding(10);

        button_icon
    };

    let color_palette = WColorPalette::new(
        &mut header_state.color_states,
        header_state.color_palette.colors.clone(),
        5,
        header_state.brush_color_id,
    );

    let color_tools = Container::new(
        Row::new()
            .spacing(5)
            .push(controllers_button(
                &mut header_state.choose_palette_state,
                BOTTOM_ARROW,
                FreezeFrameMessage::HeaderInteraction(HeaderMessage::ChangePalette),
            ))
            .push(
                Scrollable::new(&mut header_state.scrollbar_state)
                    .push(color_palette.widget.spacing(8))
                    .on_scroll(move |offset| {
                        FreezeFrameMessage::HeaderInteraction(HeaderMessage::Scrolled(offset))
                    })
                    .max_height(45)
                    .max_width(150)
                    .padding(8)
                    .spacing(5),
            )
            .push(controllers_button(
                &mut header_state.add_color_state,
                ADD,
                FreezeFrameMessage::HeaderInteraction(HeaderMessage::AddColor),
            ))
            .width(Length::Units(225)),
    );

    // Grid tools
    let controller_button = |state, icon_byte, filter, selected| {
        let icon = create_icon(icon_byte);
        let mut style = get_default_button_style();
        if filter == selected {
            style.background = Color::from_rgba8(187, 182, 197, 0.15);
        }

        let button_icon = WButtonIcon::new(
            state,
            icon,
            FreezeFrameMessage::HeaderInteraction(HeaderMessage::GridToolSelected(filter)),
            style,
        )
        .widget
        .padding(10);

        button_icon
    };

    let grid_tool = Container::new(
        Row::new()
            .spacing(5)
            .push(controller_button(
                &mut header_state.grid_state,
                GRID,
                GridFilter::Grid,
                header_state.grid_filter,
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
            .push(grid_tool)
            .align_items(Alignment::Center),
    )
    .height(Length::Units(45))
    .width(Length::Fill)
    .style(HeaderStyle);

    return header.into();
}

pub struct HeaderStyle;

impl container::StyleSheet for HeaderStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::WHITE),
            background: Some(Color::from_rgb8(34, 34, 34).into()),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}
