// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

mod canvas;
mod data;
mod tool;
mod view;

use data::AppState;
use druid::{AppLauncher, Color, LocalizedString, WindowDesc};
use view::build_ui;

pub fn main() {
    let data = AppState::new(800.0, 450.0, Color::WHITE);
    let window = WindowDesc::new(build_ui)
        .title(LocalizedString::new("Freeze Frame"))
        .window_size((1400.0, 800.0));
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}
