use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    // Output
    pub output_directory: String,
    pub filename_template: String,

    // Quality
    pub preferred_quality: String,
    pub preferred_codec: String,

    // Subtitles
    pub download_subtitles: bool,
    pub subtitle_languages: String,
    pub embed_subtitles: bool,

    // Metadata
    pub embed_thumbnail: bool,
    pub embed_metadata: bool,

    // SponsorBlock
    pub enable_sponsorblock: bool,
    pub sponsorblock_categories: String,

    // Network
    pub proxy: String,
    pub rate_limit: String,
    pub concurrent_fragments: String,

    // Authentication
    pub cookies_file: String,

    // Advanced
    pub ytdlp_path: String,
    pub js_runtimes: String,
    pub extra_arguments: String,
}

impl Default for AdvancedSettings {
    fn default() -> Self {
        Self {
            // Output
            output_directory: String::new(),
            filename_template: "%(title)s.%(ext)s".to_string(),

            // Quality
            preferred_quality: String::new(),
            preferred_codec: String::new(),

            // Subtitles
            download_subtitles: false,
            subtitle_languages: "en".to_string(),
            embed_subtitles: false,

            // Metadata
            embed_thumbnail: false,
            embed_metadata: false,

            // SponsorBlock
            enable_sponsorblock: false,
            sponsorblock_categories: "sponsor".to_string(),

            // Network
            proxy: String::new(),
            rate_limit: String::new(),
            concurrent_fragments: "1".to_string(),

            // Authentication
            cookies_file: String::new(),

            // Advanced
            ytdlp_path: "yt-dlp".to_string(),
            js_runtimes: String::new(),
            extra_arguments: String::new(),
        }
    }
}
