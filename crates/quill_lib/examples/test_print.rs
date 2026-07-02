use quill_lib::print_orientation::PageOrientation;
use quill_lib::printers::Printers;
use quill_lib::stock::Stock;

fn main() {
    let printers =
        Printers::get_available_printers().expect("Failed to get a list of available printers");
    let printer = printers
        .iter()
        .find(|p| p.printer_name.to_lowercase().contains("tsc tx2"))
        .expect("No TSC Printer connected!");
    let handle = printer.handle().expect("Failed to get printer handle!");
    println!("Found printer: {}", printer.printer_name);

    let stock = Stock::inches(1.0, 0.75)
        .with_exposed_liner(0.0, 0.05)
        .with_gap(0.12);
    handle
        .test_print(stock, PageOrientation::Normal, 1.0, 128, 8.0)
        .expect("Failed to print image!");
}
