#![windows_subsystem = "windows"]
mod app;
mod components;
mod config;
mod download;
mod message;
mod settings;
mod theme;
mod widgets;

use app::App;
use iced::{window, Size};

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("yt-dlgui")
        .theme(App::theme)
        .subscription(App::subscription)
        .window(window::Settings {
            size: Size::new(500.0, 380.0),
            min_size: Some(Size::new(400.0, 330.0)),
            decorations: false,
            ..Default::default()
        })
        .run()
}
