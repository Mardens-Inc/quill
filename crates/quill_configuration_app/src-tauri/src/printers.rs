use quill_config::QuillSettings;
use quill_lib::print_orientation::PageOrientation;
use quill_lib::printer_info::PrinterInfo;
use quill_lib::printers::Printers;
use tauri::command;

#[command]
pub fn list_printers() -> Result<Vec<PrinterInfo>, String> {
    Printers::get_available_printers().map_err(|_| "Could not get available printers".to_string())
}

#[command]
pub async fn create_test_print(stock_id: String) -> Result<(), String> {
    let settings = QuillSettings::load().map_err(|_| "Failed to load settings")?;
    let printer = match settings.selected_printer {
        Some(printer) => printer,
        None => return Err(String::from("No printer selected")),
    };
    let stock = match settings.labels.iter().find(|x| x.id == stock_id) {
        Some(stock) => stock,
        None => return Err(String::from("No stock selected")),
    };
    let orientation = match settings.default_orientation {
        0 => PageOrientation::Normal,
        1 => PageOrientation::Rotate90,
        2 => PageOrientation::Rotate180,
        3 => PageOrientation::Rotate270,
        val => PageOrientation::Degrees(val as f32),
    };
    let handle =
        Printers::get_printer_handle(printer).map_err(|_| "Failed to get printer handle")?;
    handle
        .test_print(
            stock.into(),
            orientation,
            settings.scale as f32 / 100f32,
            settings.monochrome_threshold as u32,
            settings.density as f32,
        )
        .map_err(|_| "Failed to test printer")?;

    Ok(())
}
