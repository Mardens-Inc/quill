use tracing::{debug, error, info, trace, warn};
use crate::about::about;
use crate::printers::list_printers;

mod logging;
mod settings;
mod printers;
mod about;
mod helper_service;

use crate::settings::{load, save, create_label};

pub static DEBUG: bool = cfg!(debug_assertions);

/// Bridge for webview-side logging.
///
/// The frontend logger (see `src/util/logger.ts`) invokes this command so that
/// `console.*` / `log.*` calls in React are re-emitted as `tracing` events
/// under the `frontend` target and land in the same rolling log files as the
/// Rust-side logs.
#[tauri::command]
fn log(level: String, message: String, location: Option<String>) {
    match level.as_str() {
        "trace" => trace!(target: "frontend", location = ?location, "{message}"),
        "debug" => debug!(target: "frontend", location = ?location, "{message}"),
        "info" => info!(target: "frontend", location = ?location, "{message}"),
        "warn" => warn!(target: "frontend", location = ?location, "{message}"),
        "error" => error!(target: "frontend", location = ?location, "{message}"),
        _ => info!(target: "frontend", location = ?location, "{message}"),
    }
}

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
        .invoke_handler(tauri::generate_handler![log, load, save, list_printers, about, create_label])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
