use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::{error, info};
use winreg::{RegKey, enums::HKEY_CURRENT_USER};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuillSettings {
    dark_mode: bool,
}

impl QuillSettings {
    pub fn load() -> Result<Self> {
        info!("Loading QuillSettings");
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (key, _disp) = hkcu.create_subkey(Self::path())?;
        let dark_mode: bool = key
            .get_value::<String, _>("darkMode")
            .unwrap_or("false".to_string())
            == "true";

        Ok(Self {
            dark_mode,
            ..Self::default()
        })
    }
    pub fn save(&self) -> Result<()> {
        info!("Saving QuillSettings");
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (key, _disp) = hkcu.create_subkey(Self::path())?;
        if let Err(e) = key.set_value("darkMode", &self.dark_mode.to_string()) {
            error!("Failed to save dark mode: {}", e);
            return Err(e.into());
        }
        Ok(())
    }

    fn path() -> PathBuf {
        Path::new("Software").join("Mardens-Inc").join("Quill")
    }
}
