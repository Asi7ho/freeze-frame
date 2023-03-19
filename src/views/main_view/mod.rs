use iced::{
    widget::{scrollable, Column, Container, Row},
    Element, Length,
};

use crate::{
    tools::{
        drawing::{BrushComponent, Strokes},
        filters::{BrushFilter, GeometryForm, UiControlFilter},
    },
    utils::colors,
    FreezeFrameMessage,
};

pub mod canvas;
pub mod header;
pub mod property;
pub mod timeline;

pub struct MainView {
    header_state: header::HeaderState,
    canvas_state: canvas::CanvasState,
    strokes: Vec<Strokes>,
    timeline_state: timeline::TimelineState,
    property_state: property::PropertyState,
}

impl Default for MainView {
    fn default() -> Self {
        let header_state = header::HeaderState {
            scene_title_input: String::from("Scene title"),
            ..header::HeaderState::default()
        };
        let canvas_state = canvas::CanvasState {
            brush_component: BrushComponent {
                size: 1.0,
                color: header_state.color_palette[0],
                geometry_form: Some(GeometryForm::default()),
                ..BrushComponent::default()
            },
            ..canvas::CanvasState::default()
        };
        let property_state = property::PropertyState {
            brush_slider_value: 1.0,
            eraser_slider_value: 1.0,
            geometry_form: Some(GeometryForm::default()),
            ..property::PropertyState::default()
        };

        Self {
            header_state,
            canvas_state,
            strokes: Vec::default(),
            timeline_state: timeline::TimelineState::default(),
            property_state,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MainViewMessage {
    Header(HeaderMessage),
    Canvas(CanvasMessage),
    Property(PropertyMessage),
}

//  Header
#[derive(Debug, Clone)]
pub enum HeaderMessage {
    ChangeSceneTitle(String),
    ChangeBrushControls(BrushFilter),
    SelectGridTool(UiControlFilter),
    ChangePalette,
    ChangeColor(usize),
    AddColor,
    Scroll(scrollable::RelativeOffset),
}

// Canvas
#[derive(Debug, Clone)]
pub enum CanvasMessage {
    AddStrokes(Strokes),
}

// Property
#[derive(Debug, Clone)]
pub enum PropertyMessage {
    Slide(f32),
    ChangeGeometryForm(GeometryForm),
}

pub fn update(state: &mut MainView, message: MainViewMessage) {
    match message {
        MainViewMessage::Header(m) => match m {
            HeaderMessage::ChangeSceneTitle(scene_title) => {
                state.header_state.scene_title_input = scene_title;
            }
            HeaderMessage::ChangeBrushControls(filter) => {
                state.header_state.brush_filter = filter;
                state.canvas_state.brush_component.brush = filter;
                state.property_state.filter = filter;

                if state.property_state.filter == BrushFilter::Brush {
                    state.canvas_state.brush_component.size =
                        state.property_state.brush_slider_value;
                } else if state.property_state.filter == BrushFilter::Eraser {
                    state.canvas_state.brush_component.size =
                        state.property_state.eraser_slider_value;
                }
            }
            HeaderMessage::SelectGridTool(tool) => {
                if tool == UiControlFilter::Trash {
                    state.canvas_state = canvas::CanvasState {
                        brush_component: state.canvas_state.brush_component,
                        ..canvas::CanvasState::default()
                    };
                    state.strokes.clear();
                    state.header_state.ui_control_filter = UiControlFilter::Ignore;
                } else if tool == state.header_state.ui_control_filter {
                    state.header_state.ui_control_filter = UiControlFilter::Ignore;
                } else {
                    state.header_state.ui_control_filter = tool;
                }
            }
            HeaderMessage::ChangePalette => (),
            HeaderMessage::ChangeColor(n) => {
                state.header_state.brush_color_id = n;
                state.canvas_state.brush_component.color = state.header_state.color_palette[n];
            }
            HeaderMessage::AddColor => {
                let color = colors::generate_color();
                state.header_state.color_palette.push(color);
            }
            HeaderMessage::Scroll(offset) => {
                state.header_state.color_scroll_offset = offset;
            }
        },
        MainViewMessage::Canvas(m) => match m {
            CanvasMessage::AddStrokes(stroke) => {
                state.strokes.push(stroke);
                state.canvas_state.request_redraw();
            }
        },
        MainViewMessage::Property(m) => match m {
            PropertyMessage::Slide(value) => {
                if state.property_state.filter == BrushFilter::Brush {
                    state.property_state.brush_slider_value = value;
                } else if state.property_state.filter == BrushFilter::Eraser {
                    state.property_state.eraser_slider_value = value;
                }
                state.canvas_state.brush_component.size = value;
            }
            PropertyMessage::ChangeGeometryForm(form) => {
                state.property_state.geometry_form = Some(form);
                state.canvas_state.brush_component.geometry_form = Some(form);
            }
        },
    };
}

pub fn ui(state: &MainView) -> Element<FreezeFrameMessage> {
    let header_view = header::view(&state.header_state);
    let canvas_view = state.canvas_state.view(&state.strokes).map(|stroke| {
        FreezeFrameMessage::MainView(MainViewMessage::Canvas(CanvasMessage::AddStrokes(stroke)))
    });
    let timeline_view = timeline::view(&state.timeline_state);
    let property_view = property::view(&state.property_state);

    let main_view = Column::new()
        .push(header_view)
        .push(canvas_view)
        .push(timeline_view)
        .height(Length::Fill)
        .width(Length::Fill);
    let right_bar_view = Column::new().push(property_view);

    return Container::new(Row::new().push(main_view).push(right_bar_view)).into();
}
