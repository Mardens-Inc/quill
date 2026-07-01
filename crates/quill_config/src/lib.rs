use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::{error, info};
use uuid::Uuid;
use winreg::{RegKey, enums::HKEY_CURRENT_USER};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuillSettings {
    pub dark_mode: bool,
    pub selected_printer: Option<String>,
    pub helper_service_port: u16,
    pub labels: Vec<LabelStock>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LabelStock {
    pub id: String,
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub gap: f32,
    pub liner_l: f32,
    pub liner_r: f32,
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
        let selected_printer = key.get_value::<String, _>("selectedPrinter").ok();
        let helper_service_port = key
            .get_value::<u64, _>("helperServicePort")
            .unwrap_or(51820) as u16;

        let mut labels: Vec<LabelStock> = Vec::new();
        let (stocks_key, _disp) = key.create_subkey("stocks")?;
        for subkey in stocks_key.enum_keys() {
            let subkey = subkey?;
            let (label_key, _disp) = stocks_key.create_subkey(&subkey)?;
            let name = label_key.get_value::<String, _>("name")?;
            let width = (label_key.get_value::<u64, _>("width")? as f32) / 100f32;
            let height = (label_key.get_value::<u64, _>("height")? as f32) / 100f32;
            let gap = (label_key.get_value::<u64, _>("gap")? as f32) / 100f32;
            let liner_l = (label_key.get_value::<u64, _>("liner_l")? as f32) / 100f32;
            let liner_r = (label_key.get_value::<u64, _>("liner_r")? as f32) / 100f32;
            labels.push(LabelStock {
                id: subkey.clone(),
                name,
                width,
                height,
                gap,
                liner_l,
                liner_r,
            })
        }

        Ok(Self {
            dark_mode,
            selected_printer,
            helper_service_port,
            labels,
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
        if let Err(e) = key.set_value("helperServicePort", &(self.helper_service_port as u64)) {
            error!("Failed to save helper service port: {}", e);
            return Err(e.into());
        }
        if let Some(selected_printer) = &self.selected_printer
            && let Err(e) = key.set_value("selectedPrinter", selected_printer)
        {
            error!("Failed to save selected printer: {}", e);
        }

        let (stocks, _disp) = key.create_subkey("stocks")?;

        for stock in stocks.enum_keys() {
            let stock = stock?;
            if self.labels.iter().find(|l|l.id == stock).is_none() {
                stocks.delete_subkey(&stock)?;
            }
        }

        for label in &self.labels {
            let (label_key, _disp) = stocks.create_subkey(&label.id)?;
            label_key.set_value("name", &label.name)?;
            label_key.set_value("width", &((label.width * 100f32) as u64))?;
            label_key.set_value("height", &((label.height * 100f32) as u64))?;
            label_key.set_value("gap", &((label.gap * 100f32) as u64))?;
            label_key.set_value("liner_l", &((label.liner_l * 100f32) as u64))?;
            label_key.set_value("liner_r", &((label.liner_r * 100f32) as u64))?;
        }

        Ok(())
    }

    fn path() -> PathBuf {
        Path::new("Software").join("Mardens-Inc").join("Quill")
    }
}

impl LabelStock {
    pub fn new<F>(
        name: impl Into<String>,
        width: F,
        height: F,
        gap: F,
        liner_l: F,
        liner_r: F,
    ) -> Self
    where
        F: Into<f32>,
    {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            width: width.into(),
            height: height.into(),
            gap: gap.into(),
            liner_l: liner_l.into(),
            liner_r: liner_r.into(),
        }
    }
}
