mod configs;
mod styles;
mod tools;
mod utils;
mod views;
mod widgets;

use iced::{
    executor,
    pure::{Application, Element},
    window, Color, Command, Settings,
};

use views::main_view::{self, MainView, MainViewMessage};

// Launch desktop app
fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            min_size: configs::window::MIN_SIZE,
            ..window::Settings::default()
        },
        // antialiasing: true,
        ..Settings::default()
    };

    FreezeFrame::run(settings)
}

#[derive(Default)]
pub struct FreezeFrame {
    main_view: MainView,
}

#[derive(Debug, Clone)]
pub enum FreezeFrameMessage {
    MainViewInteraction(MainViewMessage),
    Ignore,
}

impl Application for FreezeFrame {
    type Executor = executor::Default;
    type Message = FreezeFrameMessage;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<FreezeFrameMessage>) {
        (
            Self {
                main_view: MainView::default(),
                ..FreezeFrame::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Freeze Frame")
    }

    fn background_color(&self) -> Color {
        Color::from_rgb8(34, 34, 34)
    }

    fn update(&mut self, message: FreezeFrameMessage) -> Command<FreezeFrameMessage> {
        match message {
            FreezeFrameMessage::MainViewInteraction(m) => main_view::update(&mut self.main_view, m),
            FreezeFrameMessage::Ignore => (),
        }

        Command::none()
    }

    fn view(&self) -> Element<FreezeFrameMessage> {
        main_view::ui(&self.main_view)
    }
}
