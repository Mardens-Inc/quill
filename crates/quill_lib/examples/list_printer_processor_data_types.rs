use quill_lib::printers::Printers;

fn main() {
    let printers = Printers::get_available_printers().expect("Could not get available printers");
    for printer in printers {
        let printer_name: &str = printer.printer_name.as_ref();
        if let Ok(mut open_handle) = Printers::get_printer_handle(printer_name) {
            if let Ok(types) = open_handle.get_print_processor_data_types() {
                println!("{} -> {:?}", printer_name, types);
            } else {
                eprintln!("Could not get printer data");
            }
        } else {
            eprintln!("Could not open a printer handle");
        }
    }
}
