use crate::message::DownloadProgress;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct YtDlpProgress {
    status: Option<String>,
    #[serde(rename = "_percent_str")]
    percent_str: Option<String>,
    #[serde(rename = "_speed_str")]
    speed_str: Option<String>,
    #[serde(rename = "_eta_str")]
    eta_str: Option<String>,
    filename: Option<String>,
    #[serde(rename = "downloaded_bytes")]
    downloaded_bytes: Option<u64>,
    #[serde(rename = "total_bytes")]
    total_bytes: Option<u64>,
    #[serde(rename = "total_bytes_estimate")]
    total_bytes_estimate: Option<u64>,
}

pub fn parse_progress_line(line: &str) -> Option<DownloadProgress> {
    // Try to parse JSON progress output
    if let Ok(progress) = serde_json::from_str::<YtDlpProgress>(line) {
        let status = progress.status.as_deref().unwrap_or("");

        match status {
            "downloading" => {
                let percent = parse_percent(&progress);
                let speed = progress
                    .speed_str
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string())
                    .trim()
                    .to_string();
                let eta = progress
                    .eta_str
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string())
                    .trim()
                    .to_string();
                let filename = progress
                    .filename
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string());

                return Some(DownloadProgress::Downloading {
                    progress: percent,
                    speed,
                    eta,
                    filename,
                });
            }
            "finished" => {
                return Some(DownloadProgress::PostProcessing {
                    status: "Processing...".to_string(),
                });
            }
            "error" => {
                return Some(DownloadProgress::Error("Download failed".to_string()));
            }
            _ => {}
        }
    }

    // Handle non-JSON output for post-processing messages
    let line_lower = line.to_lowercase();
    if line_lower.contains("[merger]")
        || line_lower.contains("[ffmpeg]")
        || line_lower.contains("merging")
    {
        return Some(DownloadProgress::PostProcessing {
            status: "Merging formats...".to_string(),
        });
    }

    if line_lower.contains("[embedthumbnail]") {
        return Some(DownloadProgress::PostProcessing {
            status: "Embedding thumbnail...".to_string(),
        });
    }

    if line_lower.contains("[metadata]") {
        return Some(DownloadProgress::PostProcessing {
            status: "Writing metadata...".to_string(),
        });
    }

    if line_lower.contains("[sponsorblock]") {
        return Some(DownloadProgress::PostProcessing {
            status: "Removing sponsor segments...".to_string(),
        });
    }

    if line_lower.contains("error") || line_lower.contains("error:") {
        return Some(DownloadProgress::Error(line.to_string()));
    }

    None
}

fn parse_percent(progress: &YtDlpProgress) -> f32 {
    // Try to parse from percent_str first
    if let Some(ref pct) = progress.percent_str {
        let cleaned = pct.trim().trim_end_matches('%');
        if let Ok(val) = cleaned.parse::<f32>() {
            return val / 100.0;
        }
    }

    // Fall back to calculating from bytes
    let downloaded = progress.downloaded_bytes.unwrap_or(0) as f64;
    let total = progress
        .total_bytes
        .or(progress.total_bytes_estimate)
        .unwrap_or(0) as f64;

    if total > 0.0 {
        (downloaded / total) as f32
    } else {
        0.0
    }
}
