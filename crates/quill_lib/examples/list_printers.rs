use quill_lib::ffi::Printers;

fn main(){
	let printers = Printers::get_available_printers().expect("Could not get available printers");
	println!("{:#?}", printers);
}