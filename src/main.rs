use iced::{window, Element, Sandbox, Settings};
use widgets::header::{extra_tools::ExtraFilter, HeaderState};

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
}

#[derive(Debug, Clone)]

pub enum InteractionMessage {
    HeaderInteraction(widgets::header::HeaderMessage),
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
                        widgets::header::HeaderMessage::AddColor(color) => {
                            self.header_state.color_palette.colors.push(color)
                        }
                        widgets::header::HeaderMessage::ChangeColor(id) => {
                            self.header_state.brush_color_id = id
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
                InteractionMessage::Ignore => (),
            },
        }
    }

    fn view(&mut self) -> Element<FreezeFrameMessage> {
        return widgets::header::header::view(&mut self.header_state);
    }
}
