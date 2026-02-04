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
    let app_icon = include_bytes!("../resources/zenforic.ico");
    iced::application(App::new, App::update, App::view)
        .title("yt-dlgui")
        .theme(App::theme)
        .subscription(App::subscription)
        .window(window::Settings {
            size: Size::new(500.0, 380.0),
            min_size: Some(Size::new(400.0, 330.0)),
            decorations: false,
            icon: Some(iced::window::icon::from_file_data(app_icon, None).unwrap()),
            ..Default::default()
        })
        .run()
}
