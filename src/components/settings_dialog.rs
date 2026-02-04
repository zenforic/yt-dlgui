use iced::widget::{
    button, column, container, row, scrollable, text, text_input, toggler, rule, Space,
};
use iced::{Alignment, Element, Fill};

use crate::message::{Message, SettingsField};
use crate::settings::AdvancedSettings;
use crate::theme::{
    horizontal_rule_style, primary_button_style, scrollable_style, secondary_button_style,
    section_style, text_input_style, toggler_style,
};

fn horizontal_space() -> Space {
    Space::new().width(Fill)
}

pub fn settings_dialog<'a>(
    settings: &AdvancedSettings,
    persist_enabled: bool,
) -> Element<'a, Message> {
    let header = text("Advanced Settings").size(20);

    let content = scrollable(
        column![
            // Output Section
            section(
                "Output",
                column![
                    labeled_input(
                        "Output Directory",
                        "Leave empty for current directory",
                        &settings.output_directory,
                        |s| Message::SettingsChanged(SettingsField::OutputDirectory(s)),
                    ),
                    labeled_input(
                        "Filename Template",
                        "%(title)s.%(ext)s",
                        &settings.filename_template,
                        |s| Message::SettingsChanged(SettingsField::FilenameTemplate(s)),
                    ),
                ]
                .spacing(12),
            ),
            rule::horizontal(1).style(horizontal_rule_style),
            // Quality Section
            section(
                "Quality",
                column![
                    labeled_input(
                        "Preferred Quality",
                        "e.g., bestvideo+bestaudio/best",
                        &settings.preferred_quality,
                        |s| Message::SettingsChanged(SettingsField::PreferredQuality(s)),
                    ),
                    labeled_input(
                        "Preferred Codec",
                        "e.g., h264, vp9",
                        &settings.preferred_codec,
                        |s| Message::SettingsChanged(SettingsField::PreferredCodec(s)),
                    ),
                ]
                .spacing(12),
            ),
            rule::horizontal(1).style(horizontal_rule_style),
            // Subtitles Section
            section(
                "Subtitles",
                column![
                    labeled_toggle(
                        "Download Subtitles",
                        settings.download_subtitles,
                        |b| Message::SettingsChanged(SettingsField::DownloadSubtitles(b)),
                    ),
                    labeled_input(
                        "Subtitle Languages",
                        "e.g., en,es,de",
                        &settings.subtitle_languages,
                        |s| Message::SettingsChanged(SettingsField::SubtitleLanguages(s)),
                    ),
                    labeled_toggle(
                        "Embed Subtitles",
                        settings.embed_subtitles,
                        |b| Message::SettingsChanged(SettingsField::EmbedSubtitles(b)),
                    ),
                ]
                .spacing(12),
            ),
            rule::horizontal(1).style(horizontal_rule_style),
            // Metadata Section
            section(
                "Metadata",
                column![
                    labeled_toggle(
                        "Embed Thumbnail",
                        settings.embed_thumbnail,
                        |b| Message::SettingsChanged(SettingsField::EmbedThumbnail(b)),
                    ),
                    labeled_toggle(
                        "Embed Metadata",
                        settings.embed_metadata,
                        |b| Message::SettingsChanged(SettingsField::EmbedMetadata(b)),
                    ),
                ]
                .spacing(12),
            ),
            rule::horizontal(1).style(horizontal_rule_style),
            // SponsorBlock Section
            section(
                "SponsorBlock",
                column![
                    labeled_toggle(
                        "Enable SponsorBlock",
                        settings.enable_sponsorblock,
                        |b| Message::SettingsChanged(SettingsField::EnableSponsorBlock(b)),
                    ),
                    labeled_input(
                        "Categories",
                        "sponsor,selfpromo,interaction",
                        &settings.sponsorblock_categories,
                        |s| Message::SettingsChanged(SettingsField::SponsorBlockCategories(s)),
                    ),
                ]
                .spacing(12),
            ),
            rule::horizontal(1).style(horizontal_rule_style),
            // Network Section
            section(
                "Network",
                column![
                    labeled_input(
                        "Proxy",
                        "e.g., socks5://127.0.0.1:1080",
                        &settings.proxy,
                        |s| Message::SettingsChanged(SettingsField::Proxy(s)),
                    ),
                    labeled_input(
                        "Rate Limit",
                        "e.g., 1M, 500K",
                        &settings.rate_limit,
                        |s| Message::SettingsChanged(SettingsField::RateLimit(s)),
                    ),
                    labeled_input(
                        "Concurrent Fragments",
                        "1",
                        &settings.concurrent_fragments,
                        |s| Message::SettingsChanged(SettingsField::ConcurrentFragments(s)),
                    ),
                ]
                .spacing(12),
            ),
            rule::horizontal(1).style(horizontal_rule_style),
            // Authentication Section
            section(
                "Authentication",
                column![labeled_input(
                    "Cookies File",
                    "Path to cookies.txt",
                    &settings.cookies_file,
                    |s| Message::SettingsChanged(SettingsField::CookiesFile(s)),
                ),]
                .spacing(12),
            ),
            rule::horizontal(1).style(horizontal_rule_style),
            // Advanced Section
            section(
                "Advanced",
                column![
                    labeled_input(
                        "yt-dlp Path",
                        "yt-dlp",
                        &settings.ytdlp_path,
                        |s| Message::SettingsChanged(SettingsField::YtDlpPath(s)),
                    ),
                    labeled_input(
                        "Extra Arguments",
                        "Additional command-line arguments",
                        &settings.extra_arguments,
                        |s| Message::SettingsChanged(SettingsField::ExtraArguments(s)),
                    ),
                ]
                .spacing(12),
            ),
        ]
        .spacing(20)
        .padding(20),
    )
    .height(Fill)
    .style(scrollable_style);

    let persist_toggle = row![
        text("Persist Settings").size(14),
        Space::new().width(10),
        toggler(persist_enabled)
            .on_toggle(Message::TogglePersistence)
            .size(20)
            .style(toggler_style),
    ]
    .align_y(Alignment::Center);

    let defaults_button = button(text("Defaults").size(14))
        .on_press(Message::ResetDefaults)
        .padding([8, 16])
        .style(secondary_button_style);

    let cancel_button = button(text("Cancel").size(14))
        .on_press(Message::CloseSettings)
        .padding([8, 16])
        .style(secondary_button_style);

    let save_button = button(text("Save").size(14))
        .on_press(Message::SaveSettings)
        .padding([8, 16])
        .style(primary_button_style);

    let footer = row![
        persist_toggle,
        horizontal_space(),
        defaults_button,
        Space::new().width(10),
        cancel_button,
        Space::new().width(10),
        save_button,
    ]
    .padding([15, 20])
    .align_y(Alignment::Center);

    column![
        container(header).padding([15, 20]),
        content,
        rule::horizontal(1).style(horizontal_rule_style),
        footer,
    ]
    .into()
}

fn section<'a>(title: &'a str, content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    column![
        text(title).size(16).color(iced::Color::from_rgb(0.7, 0.7, 0.7)),
        Space::new().height(8),
        container(content).padding(15).style(section_style),
    ]
    .spacing(4)
    .into()
}

fn labeled_input<'a>(
    label: &'a str,
    placeholder: &'a str,
    value: &str,
    on_change: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    column![
        text(label).size(13).color(iced::Color::from_rgb(0.6, 0.6, 0.6)),
        text_input(placeholder, value)
            .on_input(on_change)
            .padding(10)
            .size(14)
            .style(text_input_style),
    ]
    .spacing(4)
    .into()
}

fn labeled_toggle<'a>(
    label: &'a str,
    value: bool,
    on_toggle: impl Fn(bool) -> Message + 'a,
) -> Element<'a, Message> {
    row![
        text(label).size(14),
        horizontal_space(),
        toggler(value)
            .on_toggle(on_toggle)
            .size(20)
            .style(toggler_style),
    ]
    .align_y(Alignment::Center)
    .into()
}
