use crate::message::{DownloadProgress, Format};
use crate::settings::AdvancedSettings;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;

use super::progress::parse_progress_line;

pub struct DownloadTask {
    url: String,
    format: Format,
    settings: AdvancedSettings,
}

impl DownloadTask {
    pub fn new(url: String, format: Format, settings: AdvancedSettings) -> Self {
        Self {
            url,
            format,
            settings,
        }
    }

    pub fn build_command(&self) -> Command {
        let ytdlp_path = if self.settings.ytdlp_path.is_empty() {
            "yt-dlp"
        } else {
            &self.settings.ytdlp_path
        };

        let mut cmd = Command::new(ytdlp_path);

        // Progress output format
        cmd.arg("--progress-template")
            .arg("download:%(progress)j");
        cmd.arg("--newline");

        // Format selection
        match self.format {
            Format::Default => {}
            Format::Mp4 => {
                cmd.arg("--merge-output-format").arg("mp4");
            }
            Format::Mkv => {
                cmd.arg("--merge-output-format").arg("mkv");
            }
            Format::Mp3 => {
                cmd.arg("-x").arg("--audio-format").arg("mp3");
            }
            Format::Aac => {
                cmd.arg("-x").arg("--audio-format").arg("aac");
            }
        }

        // Output settings
        if !self.settings.output_directory.is_empty() {
            cmd.arg("-o")
                .arg(format!("{}/{}", self.settings.output_directory, self.settings.filename_template));
        } else if self.settings.filename_template != "%(title)s.%(ext)s" {
            cmd.arg("-o").arg(&self.settings.filename_template);
        }

        // Quality settings
        if !self.settings.preferred_quality.is_empty() {
            cmd.arg("-f").arg(&self.settings.preferred_quality);
        }

        // Subtitles
        if self.settings.download_subtitles {
            cmd.arg("--write-subs");
            if !self.settings.subtitle_languages.is_empty() {
                cmd.arg("--sub-langs").arg(&self.settings.subtitle_languages);
            }
            if self.settings.embed_subtitles {
                cmd.arg("--embed-subs");
            }
        }

        // Metadata
        if self.settings.embed_thumbnail {
            cmd.arg("--embed-thumbnail");
        }
        if self.settings.embed_metadata {
            cmd.arg("--embed-metadata");
        }

        // SponsorBlock
        if self.settings.enable_sponsorblock {
            cmd.arg("--sponsorblock-remove");
            if !self.settings.sponsorblock_categories.is_empty() {
                cmd.arg(&self.settings.sponsorblock_categories);
            } else {
                cmd.arg("sponsor");
            }
        }

        // Network
        if !self.settings.proxy.is_empty() {
            cmd.arg("--proxy").arg(&self.settings.proxy);
        }
        if !self.settings.rate_limit.is_empty() {
            cmd.arg("--limit-rate").arg(&self.settings.rate_limit);
        }
        if !self.settings.concurrent_fragments.is_empty()
            && self.settings.concurrent_fragments != "1"
        {
            cmd.arg("--concurrent-fragments")
                .arg(&self.settings.concurrent_fragments);
        }

        // Authentication
        if !self.settings.cookies_file.is_empty() {
            cmd.arg("--cookies").arg(&self.settings.cookies_file);
        }

        // JS Runtimes (for YouTube JS challenges)
        if !self.settings.js_runtimes.is_empty() {
            cmd.arg("--js-runtimes").arg(&self.settings.js_runtimes);
        }

        // Extra arguments
        if !self.settings.extra_arguments.is_empty() {
            for arg in self.settings.extra_arguments.split_whitespace() {
                cmd.arg(arg);
            }
        }

        // URL always last
        cmd.arg(&self.url);

        cmd
    }

    pub async fn run(
        self,
        progress_tx: mpsc::UnboundedSender<DownloadProgress>,
    ) -> Result<String, String> {
        let mut cmd = self.build_command();

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        // Prevent window popup on Windows
        #[cfg(windows)]
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        let mut child = cmd.spawn().map_err(|e| format!("Failed to start yt-dlp: {}", e))?;

        let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
        let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;

        let mut stdout_reader = BufReader::new(stdout).lines();
        let mut stderr_reader = BufReader::new(stderr).lines();

        let progress_tx_clone = progress_tx.clone();

        // Read stdout
        let stdout_handle = tokio::spawn(async move {
            let mut last_filename = String::new();
            while let Ok(Some(line)) = stdout_reader.next_line().await {
                if let Some(progress) = parse_progress_line(&line) {
                    if let DownloadProgress::Downloading { ref filename, .. } = progress {
                        last_filename = filename.clone();
                    }
                    let _ = progress_tx.send(progress);
                }
            }
            last_filename
        });

        // Read stderr for errors and post-processing
        let stderr_handle = tokio::spawn(async move {
            let mut errors = Vec::new();
            while let Ok(Some(line)) = stderr_reader.next_line().await {
                if let Some(progress) = parse_progress_line(&line) {
                    let _ = progress_tx_clone.send(progress);
                }
                if line.to_lowercase().contains("error") {
                    errors.push(line);
                }
            }
            errors
        });

        let status = child
            .wait()
            .await
            .map_err(|e| format!("Failed to wait for yt-dlp: {}", e))?;

        let filename = stdout_handle
            .await
            .map_err(|e| format!("Stdout task failed: {}", e))?;
        let errors = stderr_handle
            .await
            .map_err(|e| format!("Stderr task failed: {}", e))?;

        if status.success() {
            Ok(filename)
        } else {
            let error_msg = if errors.is_empty() {
                format!("yt-dlp exited with code: {:?}", status.code())
            } else {
                errors.join("\n")
            };
            Err(error_msg)
        }
    }
}
