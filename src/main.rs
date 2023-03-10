mod configs;
mod styles;
mod tools;
mod utils;
mod views;
mod widgets;

use iced::{executor, theme, window, Application, Color, Command, Element, Settings, Theme};

use views::main_view::{self, MainView, MainViewMessage};

// Launch desktop app
fn main() -> iced::Result {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("freeze_frame=debug"),
    )
    .init();

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
    theme: Theme,
    main_view: MainView,
}

#[derive(Debug, Clone)]
pub enum FreezeFrameMessage {
    MainView(MainViewMessage),
    Ignore,
}

impl Application for FreezeFrame {
    type Executor = executor::Default;
    type Message = FreezeFrameMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<FreezeFrameMessage>) {
        log::info!("Initialize new Freeze Frame instance");
        (
            Self {
                theme: Theme::custom(theme::Palette {
                    background: Color::from_rgb8(34, 34, 34),
                    text: Color::WHITE,
                    primary: Color::from_rgba8(187, 182, 197, 0.15),
                    success: Color::from_rgba8(187, 182, 197, 0.15),
                    danger: Color::from_rgba8(187, 182, 197, 0.15),
                }),
                main_view: MainView::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Freeze Frame")
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: FreezeFrameMessage) -> Command<FreezeFrameMessage> {
        log::info!("Received message {:?}", message);
        match message {
            FreezeFrameMessage::MainView(m) => main_view::update(&mut self.main_view, m),
            FreezeFrameMessage::Ignore => (),
        }

        Command::none()
    }

    fn view(&self) -> Element<FreezeFrameMessage> {
        main_view::ui(&self.main_view)
    }
}
