use crate::errors::QuillError;
use crate::printer_handle::PrinterHandle;
use crate::printer_status::PrinterStatus;
use crate::printers::Printers;

#[derive(Debug, Clone)]
pub struct PrinterInfo {
    pub server_name: Option<String>,
    pub printer_name: String,
    pub share_name: Option<String>,
    pub port_name: String,
    pub driver_name: String,
    pub comment: Option<String>,
    pub location: Option<String>,
    pub print_processor: String,
    pub datatype: String,
    pub parameters: Option<String>,
    pub sep_file: Option<String>,
    /// Bitmask of PRINTER_ATTRIBUTE_* flags (LOCAL, SHARED, NETWORK, ...).
    pub attributes: u32,
    pub priority: u32,
    pub default_priority: u32,
    /// Minutes after 12:00 AM GMT that the printer is available.
    pub start_time: u32,
    /// Minutes after 12:00 AM GMT that availability ends.
    pub until_time: u32,
    /// Bitmask of PRINTER_STATUS_* flags.
    pub status: PrinterStatus,
    pub jobs: u32,
    pub average_ppm: u32,
}


impl PrinterInfo {
    pub fn handle(&self)->Result<PrinterHandle, QuillError>{
        Printers::get_printer_handle(self.printer_name.as_str())
    }
}