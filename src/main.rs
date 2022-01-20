use iced::{window, Element, Sandbox, Settings};
use widgets::header::HeaderState;

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
