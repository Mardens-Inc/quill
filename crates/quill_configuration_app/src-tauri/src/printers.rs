use quill_lib::printer_info::PrinterInfo;
use quill_lib::printers::Printers;
use tauri::command;

#[command]
pub fn list_printers() -> Result<Vec<PrinterInfo>, String> {
    Printers::get_available_printers()
        .map_err(|_| "Could not get available printers".to_string())
}
