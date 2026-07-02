use quill_lib::stock::Stock;
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
    /// The loopback port that the http server is running on.
    /// This is the server that actually will handle the print jobs
    /// and be reachable by the web browser.
    pub helper_service_port: u16,
    pub labels: Vec<LabelStock>,
    /// Higher values burn darker.
    /// Too high smears on synthetic stock; too low fades on thermal paper.
    /// Most stock prints clean at 8–10.
    /// A value between 0 and 15
    pub density: u8,
    /// Slower speeds give crisper barcodes.
    /// Drop to 2–4 ips if scanners struggle to read printed codes.
    /// The printers' iterations per second, a value between 2 and 8
    pub print_ips: u8,
    /// The rotation applied before printing.
    /// Most product tags are in portrait.
    /// 0 = portrait, 1 = landscape
    pub default_orientation: u8,
    /// Scales label content. Keep at 100% unless artwork is consistently over- or undersized.
    /// Value between 50 and 150
    pub scale: u8,
    /// Pixels darker than this become black;
    /// lighter become white when converting color artwork for thermal printing.
    /// Raise it to keep faint detail, lower it to drop background noise.
    /// Value between 0 and 255
    pub monochrome_threshold: u8,
    /// To restrict access to this software.
    /// This will ensure that only addresses with the specified origin will be allowed.
    /// This will allow wildcards, ex: https://*.mardens.com
    pub allowed_origins: Vec<String>,
    pub install_dir: PathBuf,
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

        let density = key.get_value::<u64, _>("density").unwrap_or(8) as u8;
        let print_ips = key.get_value::<u64, _>("printIPS").unwrap_or(4) as u8;
        let default_orientation = key.get_value::<u64, _>("defaultOrientation").unwrap_or(0) as u8;
        let scale = key.get_value::<u64, _>("scale").unwrap_or(100) as u8;
        let monochrome_threshold = key
            .get_value::<u64, _>("monochromeThreshold")
            .unwrap_or(128) as u8;
        let allowed_origins: Vec<String> = key.get_value("allowedOrigins").unwrap_or_default();
        let install_dir: PathBuf = PathBuf::from(key.get_value::<String, _>("installDir").unwrap_or_default());

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
            density,
            print_ips,
            default_orientation,
            scale,
            monochrome_threshold,
            allowed_origins,
            install_dir
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
        if let Err(e) = key.set_value("allowedOrigins", &self.allowed_origins) {
            error!("Failed to save allowed origins array: {}", e);
            return Err(e.into());
        }
        if let Err(e) = key.set_value("density", &(self.density as u64)) {
            error!("Failed to save density: {}", e);
            return Err(e.into());
        }
        if let Err(e) = key.set_value("printIPS", &(self.print_ips as u64)) {
            error!("Failed to save the print ips: {}", e);
            return Err(e.into());
        }
        if let Err(e) = key.set_value("monochromeThreshold", &(self.monochrome_threshold as u64)) {
            error!("Failed to save the monochrome threshold: {}", e);
            return Err(e.into());
        }
        if let Err(e) = key.set_value("defaultOrientation", &(self.default_orientation as u64)) {
            error!("Failed to save the default orientation: {}", e);
            return Err(e.into());
        }
        if let Err(e) = key.set_value("scale", &(self.scale as u64)) {
            error!("Failed to save scale: {}", e);
            return Err(e.into());
        }
        if let Err(e) = key.set_value("installDir", &self.install_dir.to_string_lossy().to_string()) {
            error!("Failed to save install directory: {}", e);
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
            if self.labels.iter().find(|l| l.id == stock).is_none() {
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

impl From<LabelStock> for Stock {
    fn from(val: LabelStock) -> Self {
        Stock::from(&val)
    }
}
impl From<&LabelStock> for Stock {
    fn from(val: &LabelStock) -> Self {
        Stock::inches(val.width, val.height).with_gap(val.gap).with_exposed_liner(val.liner_l, val.liner_r)
    }
}