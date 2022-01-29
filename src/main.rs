use iced::{window, Color, Column, Container, Element, Length, Row, Sandbox, Settings};
use widgets::{
    canvas::{CanvasState, Strokes},
    header::{extra_tools::ExtraFilter, HeaderState},
    property::PropertyState,
    timeline::TimelineState,
};

use rand::{distributions::Uniform, prelude::Distribution};

mod utils;
mod widgets;

// Launch desktop app
fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            min_size: Some((1330, 700)),
            ..window::Settings::default()
        },
        antialiasing: true,
        ..Settings::default()
    };

    FreezeFrame::run(settings)
}

#[derive(Default)]
pub struct FreezeFrame {
    header_state: HeaderState,
    canvas_state: CanvasState,
    strokes: Vec<Strokes>,
    timeline_state: TimelineState,
    property_state: PropertyState,
}

#[derive(Debug, Clone)]

pub enum InteractionMessage {
    HeaderInteraction(widgets::header::HeaderMessage),
    AddStrokes(Strokes),
    Clear,
    Ignore,
}

#[derive(Debug, Clone)]
pub enum FreezeFrameMessage {
    Interaction(InteractionMessage),
}

impl Sandbox for FreezeFrame {
    type Message = FreezeFrameMessage;

    fn new() -> Self {
        Self {
            header_state: HeaderState {
                scene_title_input: String::from("Scene title"),
                ..HeaderState::default()
            },
            canvas_state: CanvasState {
                canvas_width: 750.0,
                canvas_height: 435.0,
                ..CanvasState::default()
            },
            ..FreezeFrame::default()
        }
    }

    fn title(&self) -> String {
        String::from("Freeze Frame")
    }

    fn update(&mut self, message: FreezeFrameMessage) {
        match message {
            FreezeFrameMessage::Interaction(interaction) => match interaction {
                InteractionMessage::HeaderInteraction(header_interaction) => {
                    match header_interaction {
                        widgets::header::HeaderMessage::SceneTitleChange(scene_title) => {
                            self.header_state.scene_title_input = scene_title;
                        }
                        widgets::header::HeaderMessage::BrushControlsChange(filter) => {
                            self.header_state.brush_filter = filter
                        }
                        widgets::header::HeaderMessage::ChangePalette => (),
                        widgets::header::HeaderMessage::AddColor => {
                            let step = Uniform::new(0, 256);
                            let mut rng = rand::thread_rng();
                            let red = step.sample(&mut rng) as u8;
                            let green = step.sample(&mut rng) as u8;
                            let blue = step.sample(&mut rng) as u8;

                            self.header_state
                                .color_palette
                                .colors
                                .push(Color::from_rgb8(red, green, blue))
                        }
                        widgets::header::HeaderMessage::ChangeColor(id_row_col) => {
                            self.header_state.brush_color_id = id_row_col
                        }
                        widgets::header::HeaderMessage::ExtraToolSelected(extra_tool) => {
                            if extra_tool == self.header_state.extra_filter {
                                self.header_state.extra_filter = ExtraFilter::Ignore;
                            } else {
                                self.header_state.extra_filter = extra_tool
                            }
                        }
                        widgets::header::HeaderMessage::Scrolled(offset) => {
                            self.header_state.color_scroll_offset = offset
                        }
                    }
                }
                InteractionMessage::AddStrokes(_) => todo!(),
                InteractionMessage::Clear => todo!(),
                InteractionMessage::Ignore => (),
            },
        }
    }

    fn view(&mut self) -> Element<FreezeFrameMessage> {
        let header_view = widgets::header::header::view(&mut self.header_state);
        let canvas_view = self
            .canvas_state
            .view(&self.strokes)
            .map(|stroke| FreezeFrameMessage::Interaction(InteractionMessage::AddStrokes(stroke)));
        let timeline_view = widgets::timeline::view(&mut self.timeline_state);
        let property_view = widgets::property::view(&mut self.property_state);

        let main_view = Column::new()
            .push(header_view)
            .push(canvas_view)
            .push(timeline_view)
            .height(Length::Fill)
            .width(Length::Fill);
        let right_bar_view = Column::new().push(property_view);

        return Container::new(Row::new().push(main_view).push(right_bar_view)).into();
    }
}
