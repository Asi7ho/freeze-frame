use iced::{button, svg, Align, Button, Container, Element, Length, Row, Svg};

use crate::{
    widgets::header::{HeaderButtonStyle, HeaderMessage},
    FreezeFrameMessage, InteractionMessage,
};

use crate::utils::svg::{BRUSH, ERASER, FILL, GEOMETRY, POINTER};

const ICON_SIZE: u16 = 16;

pub struct BrushTool<'a> {
    pub brush_button: Button<'a, FreezeFrameMessage>,
    pub active: bool,
}

impl<'a> BrushTool<'a> {
    pub fn new<E>(content: E, state: &'a mut button::State) -> Self
    where
        E: Into<Element<'a, FreezeFrameMessage>>,
    {
        Self {
            brush_button: Button::new(state, content),
            active: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrushFilter {
    Pointer,
    Brush,
    Eraser,
    Fill,
    Geometry,
}

impl Default for BrushFilter {
    fn default() -> Self {
        BrushFilter::Pointer
    }
}

#[derive(Debug, Default, Clone)]
pub struct BrushControls {
    pub scene_pointer_state: button::State,
    pub scene_brush_state: button::State,
    pub scene_eraser_state: button::State,
    pub scene_geometry_state: button::State,
    pub scene_fill_state: button::State,
}

impl BrushControls {
    pub fn view(&mut self, current_brush: BrushFilter) -> Element<FreezeFrameMessage> {
        let BrushControls {
            scene_pointer_state,
            scene_brush_state,
            scene_eraser_state,
            scene_geometry_state,
            scene_fill_state,
        } = self;

        let filter_button = |state, icon_byte, filter, current_filter| {
            let handle = svg::Handle::from_memory(icon_byte);
            let icon = Svg::new(handle)
                .height(Length::Units(ICON_SIZE))
                .width(Length::Units(ICON_SIZE));
            let button = BrushTool::new(icon, state)
                .brush_button
                .style(if filter == current_filter {
                    HeaderButtonStyle::HeaderButtonSelectedStyle
                } else {
                    HeaderButtonStyle::HeaderButtonDefaultStyle
                })
                .padding(10);

            button.on_press(FreezeFrameMessage::Interaction(
                InteractionMessage::HeaderInteraction(HeaderMessage::BrushControlsChange(filter)),
            ))
        };

        let brush_tools = Container::new(
            Row::new()
                .spacing(5)
                .push(filter_button(
                    scene_pointer_state,
                    POINTER,
                    BrushFilter::Pointer,
                    current_brush,
                ))
                .push(filter_button(
                    scene_brush_state,
                    BRUSH,
                    BrushFilter::Brush,
                    current_brush,
                ))
                .push(filter_button(
                    scene_eraser_state,
                    ERASER,
                    BrushFilter::Eraser,
                    current_brush,
                ))
                .push(filter_button(
                    scene_geometry_state,
                    GEOMETRY,
                    BrushFilter::Geometry,
                    current_brush,
                ))
                .push(filter_button(
                    scene_fill_state,
                    FILL,
                    BrushFilter::Fill,
                    current_brush,
                ))
                .align_items(Align::Center),
        );

        return brush_tools.into();
    }
}
