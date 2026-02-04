use crate::settings::AdvancedSettings;
use std::fs;
use std::path::PathBuf;

fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("yt-dlgui").join("config.json"))
}

pub fn load_settings() -> Option<AdvancedSettings> {
    let path = config_path()?;
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

pub fn save_settings(settings: &AdvancedSettings) -> Result<(), String> {
    let path = config_path().ok_or("Could not determine config directory")?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    let content = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    fs::write(path, content).map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

pub fn delete_settings() -> Result<(), String> {
    if let Some(path) = config_path() {
        if path.exists() {
            fs::remove_file(path).map_err(|e| format!("Failed to delete config file: {}", e))?;
        }
    }
    Ok(())
}
