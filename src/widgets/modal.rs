use iced::widget::{container, mouse_area, stack, Space};
use iced::{Element, Fill};

use crate::message::Message;
use crate::theme::{modal_backdrop_style, modal_container_style};

pub fn modal<'a>(
    base: impl Into<Element<'a, Message>>,
    modal_content: impl Into<Element<'a, Message>>,
    on_backdrop_click: Message,
) -> Element<'a, Message> {
    let backdrop = mouse_area(
        container(Space::new())
            .width(Fill)
            .height(Fill)
            .style(modal_backdrop_style),
    )
    .on_press(on_backdrop_click);

    let modal_box = container(modal_content)
        .max_width(600)
        .max_height(700)
        .style(modal_container_style);

    let modal_layer = container(modal_box)
        .width(Fill)
        .height(Fill)
        .center_x(Fill)
        .center_y(Fill);

    stack![base.into(), stack![backdrop, modal_layer]].into()
}
