use iced::widget::{button, container, mouse_area, row, text, Space};
use iced::{Alignment, Element, Fill};

use crate::message::Message;
use crate::theme::{close_button_style, title_bar_button_style, title_bar_style};

pub fn title_bar<'a>() -> Element<'a, Message> {
    let title = text("yt-dlgui").size(14);

    let minimize_btn = button(text("\u{2212}").size(16))
        .on_press(Message::WindowMinimize)
        .padding([4, 12])
        .style(title_bar_button_style);

    let close_btn = button(text("\u{2715}").size(14))
        .on_press(Message::WindowClose)
        .padding([4, 12])
        .style(close_button_style);

    let controls = row![minimize_btn, close_btn].spacing(2);

    let bar_content = row![title, Space::new().width(Fill), controls]
        .align_y(Alignment::Center)
        .padding([8, 12]);

    let bar = container(bar_content)
        .width(Fill)
        .style(title_bar_style);

    mouse_area(bar)
        .on_press(Message::WindowDrag)
        .into()
}
