use iced::widget::{button, column, container, pick_list, progress_bar, row, text, text_input, Space};
use iced::{Alignment, Element, Fill};

use crate::app::DownloadState;
use crate::message::{Format, Message};
use crate::theme::{
    main_container_style, pick_list_style, primary_button_style,
    progress_bar_style, secondary_button_style, text_input_style, danger_button_style,
};

fn horizontal_space() -> Space {
    Space::new().width(Fill)
}

pub fn home_view<'a>(
    url: &str,
    format: Format,
    download_state: &'a DownloadState,
) -> Element<'a, Message> {
    let url_input = text_input("Enter video URL...", url)
        .on_input(Message::UrlChanged)
        .on_submit(Message::StartDownload)
        .padding(12)
        .width(Fill)
        .style(text_input_style);

    let format_picker = pick_list(
        Format::ALL.as_slice(),
        Some(format),
        Message::FormatSelected,
    )
    .placeholder("Format")
    .padding(10)
    .width(120)
    .style(pick_list_style);

    let is_downloading = matches!(
        download_state,
        DownloadState::Downloading { .. } | DownloadState::PostProcessing { .. }
    );
    let has_url = !url.trim().is_empty();

    let download_button = if is_downloading {
        button(text("Cancel"))
            .on_press(Message::CancelDownload)
            .padding([10, 20])
            .style(danger_button_style)
    } else {
        let btn = button(text("Download")).padding([10, 20]).style(primary_button_style);
        if has_url {
            btn.on_press(Message::StartDownload)
        } else {
            btn
        }
    };

    let advanced_button = button(text("Advanced"))
        .on_press(Message::OpenSettings)
        .padding([10, 20])
        .style(secondary_button_style);

    let buttons_row = row![download_button, horizontal_space(), advanced_button]
        .spacing(10)
        .align_y(Alignment::Center);

    let progress_section: Element<'a, Message> = match download_state {
        DownloadState::Idle => column![].into(),
        DownloadState::Downloading {
            progress,
            speed,
            eta,
            filename,
        } => {
            let pbar = progress_bar(0.0..=1.0, *progress)
                .style(progress_bar_style);

            let percentage = format!("{:.1}%", progress * 100.0);
            let status_text = format!("{} - ETA: {}", speed, eta);

            let filename_text = text(truncate_filename(filename, 50))
                .size(12)
                .color(iced::Color::from_rgb(0.6, 0.6, 0.6));

            column![
                pbar,
                row![
                    text(percentage).size(14),
                    horizontal_space(),
                    text(status_text).size(14),
                ]
                .spacing(10),
                filename_text,
            ]
            .spacing(8)
            .into()
        }
        DownloadState::PostProcessing { status } => {
            let pbar = progress_bar(0.0..=1.0, 1.0)
                .style(progress_bar_style);

            column![pbar, text(status).size(14),].spacing(8).into()
        }
        DownloadState::Completed { output_path } => {
            let filename = std::path::Path::new(output_path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| output_path.clone());

            column![
                text("Download Complete!").size(16).color(iced::Color::from_rgb(0.3, 0.7, 0.35)),
                text(truncate_filename(&filename, 50)).size(12).color(iced::Color::from_rgb(0.6, 0.6, 0.6)),
            ]
            .spacing(4)
            .into()
        }
        DownloadState::Error { message } => {
            column![
                text("Error").size(16).color(iced::Color::from_rgb(0.9, 0.25, 0.25)),
                text(message).size(12).color(iced::Color::from_rgb(0.7, 0.4, 0.4)),
            ]
            .spacing(4)
            .into()
        }
    };

    let content = column![
        url_input,
        format_picker,
        Space::new().height(10),
        buttons_row,
        Space::new().height(15),
        progress_section,
    ]
    .spacing(15)
    .padding(25)
    .width(Fill);

    container(content)
        .width(Fill)
        .height(Fill)
        .style(main_container_style)
        .into()
}

fn truncate_filename(filename: &str, max_len: usize) -> String {
    if filename.len() <= max_len {
        filename.to_string()
    } else {
        format!("{}...", &filename[..max_len - 3])
    }
}
