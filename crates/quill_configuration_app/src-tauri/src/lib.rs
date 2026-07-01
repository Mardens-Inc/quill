use crate::about::about;
use crate::printers::list_printers;
use tracing::info;

mod about;
mod helper_service;
mod logging;
mod printers;
mod settings;

use crate::logging::{get_logs, log, logs_directory};
use crate::settings::{create_label, load, save};

pub static DEBUG: bool = cfg!(debug_assertions);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if let Err(e) = logging::setup_logging() {
        eprintln!("failed to initialise logging: {e}");
    }
    if let Err(e) = color_eyre::install() {
        eprintln!("failed to install color-eyre: {e}");
    }

    info!(
        "Starting {} build of the application...",
        if DEBUG { "development" } else { "production" }
    );

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            log,
            get_logs,
            logs_directory,
            load,
            save,
            list_printers,
            about,
            create_label
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
