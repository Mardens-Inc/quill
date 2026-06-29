use crate::settings::data::QuillSettings;
use tauri::command;

#[command]
pub fn save(value: QuillSettings) -> Result<(), String> {
    value
        .save()
        .map_err(|e| format!("Failed to save settings: {}", e))?;
    Ok(())
}

#[command]
pub fn load() -> Result<QuillSettings, String> {
    let settings = QuillSettings::load().map_err(|e| format!("Failed to load settings: {}", e))?;
    Ok(settings)
}
