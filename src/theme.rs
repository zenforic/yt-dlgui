use iced::widget::{button, container, pick_list, progress_bar, scrollable, text_input, toggler, rule};
use iced::{Background, Border, Color, Theme};

pub fn custom_theme() -> Theme {
    Theme::custom(
        "yt-dlgui Dark".to_string(),
        iced::theme::Palette {
            background: Color::from_rgba(0.10, 0.10, 0.12, 0.95),
            text: Color::from_rgb(0.90, 0.90, 0.90),
            primary: Color::from_rgb(0.85, 0.20, 0.25),
            success: Color::from_rgb(0.30, 0.70, 0.35),
            danger: Color::from_rgb(0.90, 0.25, 0.25),
            warning: Color::from_rgb(0.95, 0.75, 0.20),
        },
    )
}

pub fn main_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.10, 0.10, 0.12, 0.95))),
        text_color: Some(Color::from_rgb(0.90, 0.90, 0.90)),
        ..Default::default()
    }
}

pub fn window_container_style(_theme: &Theme, opacity: f32) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.10, 0.10, 0.12, 0.92 * opacity))),
        text_color: Some(Color::from_rgba(0.90, 0.90, 0.90, opacity)),
        border: Border {
            color: Color::from_rgba(0.85, 0.20, 0.25, 0.6 * opacity),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn title_bar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.08, 0.08, 0.10, 1.0))),
        text_color: Some(Color::from_rgb(0.90, 0.90, 0.90)),
        border: Border {
            color: Color::from_rgba(0.25, 0.25, 0.28, 0.5),
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn title_bar_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: Color::from_rgb(0.70, 0.70, 0.70),
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.1))),
            text_color: Color::from_rgb(0.90, 0.90, 0.90),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.05))),
            ..base
        },
        button::Status::Disabled => base,
    }
}

pub fn close_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: Color::from_rgb(0.70, 0.70, 0.70),
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.85, 0.20, 0.25))),
            text_color: Color::WHITE,
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.70, 0.15, 0.20))),
            text_color: Color::WHITE,
            ..base
        },
        button::Status::Disabled => base,
    }
}

pub fn modal_backdrop_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.6))),
        ..Default::default()
    }
}

pub fn modal_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.15, 0.15, 0.17, 1.0))),
        text_color: Some(Color::from_rgb(0.90, 0.90, 0.90)),
        border: Border {
            color: Color::from_rgb(0.25, 0.25, 0.28),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn section_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.12, 0.12, 0.14, 1.0))),
        border: Border {
            color: Color::from_rgb(0.20, 0.20, 0.22),
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    }
}

pub fn primary_button_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.palette();
    let base = button::Style {
        background: Some(Background::Color(palette.primary)),
        text_color: Color::WHITE,
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.95, 0.30, 0.35))),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.75, 0.15, 0.20))),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.40, 0.40, 0.42))),
            text_color: Color::from_rgb(0.60, 0.60, 0.60),
            ..base
        },
    }
}

pub fn secondary_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(Color::from_rgb(0.25, 0.25, 0.28))),
        text_color: Color::from_rgb(0.90, 0.90, 0.90),
        border: Border {
            color: Color::from_rgba(0.85, 0.20, 0.25, 0.5),
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.30, 0.30, 0.33))),
            border: Border {
                color: Color::from_rgba(0.85, 0.20, 0.25, 0.8),
                width: 1.0,
                radius: 4.0.into(),
            },
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.20, 0.20, 0.22))),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.18, 0.18, 0.20))),
            text_color: Color::from_rgb(0.50, 0.50, 0.50),
            border: Border {
                color: Color::from_rgba(0.50, 0.20, 0.22, 0.3),
                width: 1.0,
                radius: 4.0.into(),
            },
            ..base
        },
    }
}

pub fn danger_button_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.palette();
    let base = button::Style {
        background: Some(Background::Color(palette.danger)),
        text_color: Color::WHITE,
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(1.0, 0.35, 0.35))),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.80, 0.20, 0.20))),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.40, 0.40, 0.42))),
            text_color: Color::from_rgb(0.60, 0.60, 0.60),
            ..base
        },
    }
}

pub fn text_input_style(_theme: &Theme, status: text_input::Status) -> text_input::Style {
    let base = text_input::Style {
        background: Background::Color(Color::from_rgb(0.15, 0.15, 0.17)),
        border: Border {
            color: Color::from_rgb(0.25, 0.25, 0.28),
            width: 1.0,
            radius: 4.0.into(),
        },
        icon: Color::from_rgb(0.60, 0.60, 0.60),
        placeholder: Color::from_rgb(0.50, 0.50, 0.50),
        value: Color::from_rgb(0.90, 0.90, 0.90),
        selection: Color::from_rgba(0.85, 0.20, 0.25, 0.3),
    };

    match status {
        text_input::Status::Active => base,
        text_input::Status::Hovered => text_input::Style {
            border: Border {
                color: Color::from_rgb(0.35, 0.35, 0.38),
                ..base.border
            },
            ..base
        },
        text_input::Status::Focused { .. } => text_input::Style {
            border: Border {
                color: Color::from_rgb(0.85, 0.20, 0.25),
                ..base.border
            },
            ..base
        },
        text_input::Status::Disabled => text_input::Style {
            background: Background::Color(Color::from_rgb(0.12, 0.12, 0.14)),
            value: Color::from_rgb(0.50, 0.50, 0.50),
            ..base
        },
    }
}

pub fn pick_list_style(_theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let base = pick_list::Style {
        background: Background::Color(Color::from_rgb(0.15, 0.15, 0.17)),
        text_color: Color::from_rgb(0.90, 0.90, 0.90),
        placeholder_color: Color::from_rgb(0.50, 0.50, 0.50),
        handle_color: Color::from_rgb(0.70, 0.70, 0.70),
        border: Border {
            color: Color::from_rgb(0.25, 0.25, 0.28),
            width: 1.0,
            radius: 4.0.into(),
        },
    };

    match status {
        pick_list::Status::Active => base,
        pick_list::Status::Hovered => pick_list::Style {
            border: Border {
                color: Color::from_rgb(0.35, 0.35, 0.38),
                ..base.border
            },
            ..base
        },
        pick_list::Status::Opened { .. } => pick_list::Style {
            border: Border {
                color: Color::from_rgb(0.85, 0.20, 0.25),
                ..base.border
            },
            ..base
        },
    }
}

pub fn progress_bar_style(_theme: &Theme) -> progress_bar::Style {
    progress_bar::Style {
        background: Background::Color(Color::from_rgb(0.20, 0.20, 0.22)),
        bar: Background::Color(Color::from_rgb(0.85, 0.20, 0.25)),
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
    }
}

pub fn scrollable_style(_theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    let base_scroller = scrollable::Scroller {
        background: Background::Color(Color::from_rgb(0.35, 0.35, 0.38)),
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
    };

    let rail = scrollable::Rail {
        background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.17))),
        border: Border::default(),
        scroller: base_scroller,
    };

    let base_style = scrollable::Style {
        container: container::Style::default(),
        vertical_rail: rail,
        horizontal_rail: rail,
        gap: None,
        auto_scroll: scrollable::AutoScroll {
            background: Background::Color(Color::from_rgba(0.15, 0.15, 0.17, 0.9)),
            border: Border::default(),
            shadow: iced::Shadow::default(),
            icon: Color::from_rgb(0.70, 0.70, 0.70),
        },
    };

    match status {
        scrollable::Status::Active { .. } => base_style,
        scrollable::Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            ..
        } => {
            let hovered_scroller = scrollable::Scroller {
                background: Background::Color(Color::from_rgb(0.50, 0.50, 0.52)),
                ..base_scroller
            };
            scrollable::Style {
                vertical_rail: scrollable::Rail {
                    scroller: if is_vertical_scrollbar_hovered {
                        hovered_scroller
                    } else {
                        base_scroller
                    },
                    ..rail
                },
                horizontal_rail: scrollable::Rail {
                    scroller: if is_horizontal_scrollbar_hovered {
                        hovered_scroller
                    } else {
                        base_scroller
                    },
                    ..rail
                },
                ..base_style
            }
        }
        scrollable::Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
            ..
        } => {
            let dragged_scroller = scrollable::Scroller {
                background: Background::Color(Color::from_rgb(0.85, 0.20, 0.25)),
                ..base_scroller
            };
            scrollable::Style {
                vertical_rail: scrollable::Rail {
                    scroller: if is_vertical_scrollbar_dragged {
                        dragged_scroller
                    } else {
                        base_scroller
                    },
                    ..rail
                },
                horizontal_rail: scrollable::Rail {
                    scroller: if is_horizontal_scrollbar_dragged {
                        dragged_scroller
                    } else {
                        base_scroller
                    },
                    ..rail
                },
                ..base_style
            }
        }
    }
}

pub fn toggler_style(_theme: &Theme, status: toggler::Status) -> toggler::Style {
    match status {
        toggler::Status::Active { is_toggled } => toggler::Style {
            background: Background::Color(if is_toggled {
                Color::from_rgb(0.85, 0.20, 0.25)
            } else {
                Color::from_rgb(0.30, 0.30, 0.32)
            }),
            background_border_width: 0.0,
            background_border_color: Color::TRANSPARENT,
            foreground: Background::Color(Color::WHITE),
            foreground_border_width: 0.0,
            foreground_border_color: Color::TRANSPARENT,
            border_radius: Some(10.0.into()),
            padding_ratio: 0.1,
            text_color: None,
        },
        toggler::Status::Hovered { is_toggled } => toggler::Style {
            background: Background::Color(if is_toggled {
                Color::from_rgb(0.95, 0.30, 0.35)
            } else {
                Color::from_rgb(0.40, 0.40, 0.42)
            }),
            background_border_width: 0.0,
            background_border_color: Color::TRANSPARENT,
            foreground: Background::Color(Color::WHITE),
            foreground_border_width: 0.0,
            foreground_border_color: Color::TRANSPARENT,
            border_radius: Some(10.0.into()),
            padding_ratio: 0.1,
            text_color: None,
        },
        toggler::Status::Disabled { is_toggled } => toggler::Style {
            background: Background::Color(if is_toggled {
                Color::from_rgb(0.50, 0.20, 0.22)
            } else {
                Color::from_rgb(0.20, 0.20, 0.22)
            }),
            background_border_width: 0.0,
            background_border_color: Color::TRANSPARENT,
            foreground: Background::Color(Color::from_rgb(0.60, 0.60, 0.60)),
            foreground_border_width: 0.0,
            foreground_border_color: Color::TRANSPARENT,
            border_radius: Some(10.0.into()),
            padding_ratio: 0.1,
            text_color: None,
        },
    }
}

pub fn horizontal_rule_style(_theme: &Theme) -> rule::Style {
    rule::Style {
        color: Color::from_rgb(0.25, 0.25, 0.28),
        radius: 0.0.into(),
        fill_mode: rule::FillMode::Full,
        snap: false,
    }
}
