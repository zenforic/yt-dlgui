#[derive(Debug, Clone)]
pub enum Message {
    // URL input
    UrlChanged(String),
    UrlSubmit,

    // Format selection
    FormatSelected(Format),

    // Download actions
    StartDownload,
    CancelDownload,
    DownloadProgress(DownloadProgress),
    DownloadComplete(Result<String, String>),

    // Settings modal
    OpenSettings,
    CloseSettings,
    SaveSettings,
    ResetDefaults,
    TogglePersistence(bool),

    // Settings field updates
    SettingsChanged(SettingsField),

    // Modal backdrop click
    ModalBackdropClicked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Format {
    #[default]
    Default,
    Mp4,
    Mp3,
    Mkv,
    Aac,
}

impl Format {
    pub const ALL: [Format; 5] = [
        Format::Default,
        Format::Mp4,
        Format::Mp3,
        Format::Mkv,
        Format::Aac,
    ];

    pub fn is_audio(&self) -> bool {
        matches!(self, Format::Mp3 | Format::Aac)
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Default => write!(f, "Default"),
            Format::Mp4 => write!(f, "MP4"),
            Format::Mp3 => write!(f, "MP3"),
            Format::Mkv => write!(f, "MKV"),
            Format::Aac => write!(f, "AAC"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DownloadProgress {
    Downloading {
        progress: f32,
        speed: String,
        eta: String,
        filename: String,
    },
    PostProcessing {
        status: String,
    },
    Error(String),
}

#[derive(Debug, Clone)]
pub enum SettingsField {
    // Output
    OutputDirectory(String),
    FilenameTemplate(String),

    // Quality
    PreferredQuality(String),
    PreferredCodec(String),

    // Subtitles
    DownloadSubtitles(bool),
    SubtitleLanguages(String),
    EmbedSubtitles(bool),

    // Metadata
    EmbedThumbnail(bool),
    EmbedMetadata(bool),

    // SponsorBlock
    EnableSponsorBlock(bool),
    SponsorBlockCategories(String),

    // Network
    Proxy(String),
    RateLimit(String),
    ConcurrentFragments(String),

    // Authentication
    CookiesFile(String),

    // Advanced
    YtDlpPath(String),
    ExtraArguments(String),
}
