use quill_lib::printers::Printers;

fn main() {
    let printers = Printers::get_available_printers().expect("Could not get available printers");
    for printer in printers {
        println!("[{:?}]\t {} - {}dpi", printer.status, printer.printer_name, printer.dpi.unwrap_or(0));
    }
}
