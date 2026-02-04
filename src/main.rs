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
        .window(window::Settings {
            size: Size::new(500.0, 350.0),
            min_size: Some(Size::new(400.0, 300.0)),
            transparent: true,
            decorations: true,
            ..Default::default()
        })
        .run()
}
