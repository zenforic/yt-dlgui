use iced::widget::container;
use iced::{Element, Fill, Task, Theme};
use tokio::sync::mpsc;

use crate::components::{home_view, settings_dialog};
use crate::config;
use crate::download::DownloadTask;
use crate::message::{DownloadProgress, Format, Message, SettingsField};
use crate::settings::AdvancedSettings;
use crate::theme::{custom_theme, main_container_style};
use crate::widgets::modal;

pub struct App {
    url: String,
    format: Format,
    download_state: DownloadState,
    settings: AdvancedSettings,
    pending_settings: Option<AdvancedSettings>,
    show_settings: bool,
    persist_settings: bool,
    cancel_sender: Option<mpsc::Sender<()>>,
}

#[derive(Debug, Clone, Default)]
pub enum DownloadState {
    #[default]
    Idle,
    Downloading {
        progress: f32,
        speed: String,
        eta: String,
        filename: String,
    },
    PostProcessing {
        status: String,
    },
    Completed {
        output_path: String,
    },
    Error {
        message: String,
    },
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let (settings, persist) = config::load_settings()
            .map(|s| (s, true))
            .unwrap_or_else(|| (AdvancedSettings::default(), false));

        (
            Self {
                url: String::new(),
                format: Format::Default,
                download_state: DownloadState::Idle,
                settings,
                pending_settings: None,
                show_settings: false,
                persist_settings: persist,
                cancel_sender: None,
            },
            Task::none(),
        )
    }


    pub fn theme(_state: &Self) -> Theme {
        custom_theme()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UrlChanged(url) => {
                self.url = url;
                Task::none()
            }
            Message::UrlSubmit => {
                if !self.url.trim().is_empty() {
                    self.update(Message::StartDownload)
                } else {
                    Task::none()
                }
            }
            Message::FormatSelected(format) => {
                self.format = format;
                Task::none()
            }
            Message::StartDownload => {
                if self.url.trim().is_empty() {
                    return Task::none();
                }

                let url = self.url.clone();
                let format = self.format;
                let settings = self.settings.clone();

                let (cancel_tx, mut cancel_rx) = mpsc::channel::<()>(1);
                self.cancel_sender = Some(cancel_tx);

                self.download_state = DownloadState::Downloading {
                    progress: 0.0,
                    speed: "Starting...".to_string(),
                    eta: "N/A".to_string(),
                    filename: String::new(),
                };

                Task::stream(async_stream::stream! {
                    let (progress_tx, mut progress_rx) = mpsc::unbounded_channel();
                    let task = DownloadTask::new(url, format, settings);

                    let download_handle = tokio::spawn(async move { task.run(progress_tx).await });

                    loop {
                        tokio::select! {
                            _ = cancel_rx.recv() => {
                                download_handle.abort();
                                yield Message::DownloadComplete(Err("Cancelled".to_string()));
                                break;
                            }
                            progress = progress_rx.recv() => {
                                match progress {
                                    Some(p) => yield Message::DownloadProgress(p),
                                    None => break,
                                }
                            }
                        }
                    }

                    match download_handle.await {
                        Ok(Ok(path)) => yield Message::DownloadComplete(Ok(path)),
                        Ok(Err(e)) => yield Message::DownloadComplete(Err(e)),
                        Err(e) if e.is_cancelled() => {},
                        Err(e) => yield Message::DownloadComplete(Err(format!("Task error: {}", e))),
                    }
                })
            }
            Message::CancelDownload => {
                if let Some(sender) = self.cancel_sender.take() {
                    let _ = sender.try_send(());
                }
                Task::none()
            }
            Message::DownloadProgress(progress) => {
                match progress {
                    DownloadProgress::Downloading {
                        progress: pct,
                        speed,
                        eta,
                        filename,
                    } => {
                        self.download_state = DownloadState::Downloading {
                            progress: pct,
                            speed,
                            eta,
                            filename,
                        };
                    }
                    DownloadProgress::PostProcessing { status } => {
                        self.download_state = DownloadState::PostProcessing { status };
                    }
                    DownloadProgress::Error(msg) => {
                        self.download_state = DownloadState::Error { message: msg };
                    }
                }
                Task::none()
            }
            Message::DownloadComplete(result) => {
                self.cancel_sender = None;
                match result {
                    Ok(path) => {
                        self.download_state = DownloadState::Completed { output_path: path };
                    }
                    Err(msg) => {
                        if msg != "Cancelled" {
                            self.download_state = DownloadState::Error { message: msg };
                        } else {
                            self.download_state = DownloadState::Idle;
                        }
                    }
                }
                Task::none()
            }
            Message::OpenSettings => {
                self.pending_settings = Some(self.settings.clone());
                self.show_settings = true;
                Task::none()
            }
            Message::CloseSettings | Message::ModalBackdropClicked => {
                self.pending_settings = None;
                self.show_settings = false;
                Task::none()
            }
            Message::SaveSettings => {
                if let Some(pending) = self.pending_settings.take() {
                    self.settings = pending;
                    if self.persist_settings {
                        let _ = config::save_settings(&self.settings);
                    }
                }
                self.show_settings = false;
                Task::none()
            }
            Message::ResetDefaults => {
                self.pending_settings = Some(AdvancedSettings::default());
                Task::none()
            }
            Message::TogglePersistence(enabled) => {
                self.persist_settings = enabled;
                if !enabled {
                    let _ = config::delete_settings();
                }
                Task::none()
            }
            Message::SettingsChanged(field) => {
                if let Some(ref mut settings) = self.pending_settings {
                    match field {
                        SettingsField::OutputDirectory(v) => settings.output_directory = v,
                        SettingsField::FilenameTemplate(v) => settings.filename_template = v,
                        SettingsField::PreferredQuality(v) => settings.preferred_quality = v,
                        SettingsField::PreferredCodec(v) => settings.preferred_codec = v,
                        SettingsField::DownloadSubtitles(v) => settings.download_subtitles = v,
                        SettingsField::SubtitleLanguages(v) => settings.subtitle_languages = v,
                        SettingsField::EmbedSubtitles(v) => settings.embed_subtitles = v,
                        SettingsField::EmbedThumbnail(v) => settings.embed_thumbnail = v,
                        SettingsField::EmbedMetadata(v) => settings.embed_metadata = v,
                        SettingsField::EnableSponsorBlock(v) => settings.enable_sponsorblock = v,
                        SettingsField::SponsorBlockCategories(v) => {
                            settings.sponsorblock_categories = v
                        }
                        SettingsField::Proxy(v) => settings.proxy = v,
                        SettingsField::RateLimit(v) => settings.rate_limit = v,
                        SettingsField::ConcurrentFragments(v) => settings.concurrent_fragments = v,
                        SettingsField::CookiesFile(v) => settings.cookies_file = v,
                        SettingsField::YtDlpPath(v) => settings.ytdlp_path = v,
                        SettingsField::ExtraArguments(v) => settings.extra_arguments = v,
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let home = home_view(&self.url, self.format, &self.download_state);

        if self.show_settings {
            let settings = self.pending_settings.as_ref().unwrap_or(&self.settings);
            let dialog = settings_dialog(settings, self.persist_settings);
            modal(home, dialog, Message::ModalBackdropClicked)
        } else {
            container(home)
                .width(Fill)
                .height(Fill)
                .style(main_container_style)
                .into()
        }
    }
}
