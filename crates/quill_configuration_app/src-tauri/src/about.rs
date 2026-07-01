use crate::helper_service::{Version, get_versions, is_running};
use serde::Serialize;
use tauri::command;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_NUMBER: &str = env!("BUILD");
const CONFIG_SCHEMA_VERSION: u8 = 1;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutInfo {
    version: String,
    build_number: String,
    helper_version: Version,
    config_schema_version: u8,
    helper_running: bool,
}

#[command]
pub async fn about() -> Result<AboutInfo, String> {
    let running = is_running().await.unwrap_or(false);
    let version = match running {
        true => get_versions()
            .await
            .map_err(|_| "Failed to get version".to_string())?,
        false => Version::default(),
    };
    Ok(AboutInfo {
        version: APP_VERSION.to_string(),
        build_number: BUILD_NUMBER.to_string(),
        config_schema_version: CONFIG_SCHEMA_VERSION,
        helper_running: running,
        helper_version: version,
    })
}
