use quill_config::{LabelStock, QuillSettings};
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

#[command]
pub fn create_label(
    name: String,
    width: f32,
    height: f32,
    gap: f32,
    label_l: f32,
    label_r: f32,
) -> Result<(), String> {
    let label = LabelStock::new(name, width, height, gap, label_l, label_r);
    let mut settings =
        QuillSettings::load().map_err(|e| format!("Failed to load settings: {}", e))?;
    settings.labels.push(label);
    settings
        .save()
        .map_err(|e| format!("Failed to save settings: {}", e))?;
    Ok(())
}
