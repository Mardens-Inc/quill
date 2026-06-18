use quill_lib::print_orientation::PageOrientation;
use quill_lib::printers::Printers;

fn main() {
    let printers =
        Printers::get_available_printers().expect("Failed to get a list of available printers");
    let printer = printers
        .iter()
        .find(|p| p.printer_name.to_lowercase().contains("tsc tx2"))
        .expect("No TSC Printer connected!");
    let handle = printer.handle().expect("Failed to get printer handle!");
	println!("Found printer: {}", printer.printer_name);

	// Load the image in
	let bytes = std::fs::read("crates/quill_lib/examples/tag.png").expect("Failed to read from tag.png!");
	let dynamic_image = image::load_from_memory(&bytes[..]).expect("Failed to load image!");
	let img = dynamic_image.into_rgb8();
	println!("Loaded image: {}x{}", img.width(), img.height());
	handle.print_png("test-print", &img, PageOrientation::Rotate180, 1.0).expect("Failed to print image!");
}
