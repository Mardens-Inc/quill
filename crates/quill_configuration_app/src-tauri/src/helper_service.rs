use color_eyre::Result;
use quill_config::QuillSettings;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    version: String,
    build: String,
}

pub async fn is_running() -> Result<bool> {
    let settings = QuillSettings::load()?;
    let port = settings.helper_service_port;

    let status = reqwest::get(&format!("http://localhost:{}/health", port))
        .await?
        .error_for_status()?
        .status();
    Ok(status == reqwest::StatusCode::OK)
}

pub async fn get_versions() -> Result<Version> {
    let settings = QuillSettings::load()?;
    let port = settings.helper_service_port;

    let version: Version = reqwest::get(&format!("http://localhost:{}/version", port))
        .await?
        .json()
        .await?;

    Ok(version)
}
