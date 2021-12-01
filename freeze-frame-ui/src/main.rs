use iced::{
    executor, Align, Application, Clipboard, Column, Command, Element, Length, Settings, Text,
};

fn main() -> iced::Result {
    FreezeFrameApp::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Default)]
struct FreezeFrameApp {}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Application for FreezeFrameApp {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        return (Self::default(), Command::none());
    }

    fn title(&self) -> String {
        String::from("Freeze Frame")
    }

    fn update(&mut self, _message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        return Command::none();
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(
                Text::new(String::from("Hello, world!"))
                    .width(Length::Shrink)
                    .size(50),
            )
            .into()
    }
}
