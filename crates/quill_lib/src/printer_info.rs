use crate::errors::QuillError;
use crate::printer_handle::PrinterHandle;
use crate::printer_status::PrinterStatus;
use crate::printers::Printers;
use serde::{Deserialize, Serialize};
use tracing::debug;
use windows::Win32::Storage::Xps::{DC_ENUMRESOLUTIONS, DeviceCapabilitiesA};
use windows::core::PCSTR;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub dpi: Option<u32>,
}

impl PrinterInfo {
    pub fn handle(&self) -> Result<PrinterHandle, QuillError> {
        debug!("Acquiring handle for printer '{}'", self.printer_name);
        Printers::get_printer_handle(self.printer_name.as_str())
    }
    pub fn get_dpi(&self) -> Result<u32, QuillError> {
        let name = crate::to_cstring(&self.printer_name)?;
        let port = crate::to_cstring(&self.port_name)?;
        let name_pcstr = PCSTR(name.as_ptr().cast());
        let port_pcstr = PCSTR(port.as_ptr().cast());

        let count = unsafe {
            DeviceCapabilitiesA(name_pcstr, port_pcstr, DC_ENUMRESOLUTIONS, None, None)
        };
        if count <= 0 {
            return Err(QuillError::UnsupportedOperation(
                "DeviceCapabilitiesA(DC_ENUMRESOLUTIONS) failed or returned no resolutions".into(),
            ));
        }

        // DC_ENUMRESOLUTIONS returns pairs of LONG (i32): [x_dpi, y_dpi, x_dpi, y_dpi, ...]
        let pair_count = count as usize;
        let mut buf: Vec<i32> = vec![0i32; pair_count * 2];
        let out_ptr = windows::core::PSTR(buf.as_mut_ptr() as *mut u8);

        let result = unsafe {
            DeviceCapabilitiesA(name_pcstr, port_pcstr, DC_ENUMRESOLUTIONS, Some(out_ptr), None)
        };
        if result <= 0 {
            return Err(QuillError::UnsupportedOperation(
                "DeviceCapabilitiesA(DC_ENUMRESOLUTIONS) failed to fill buffer".into(),
            ));
        }

        // Return the X DPI of the first (typically lowest/default) resolution.
        let x_dpi = buf[0];
        if x_dpi <= 0 {
            return Err(QuillError::UnsupportedOperation(
                "DeviceCapabilitiesA returned an invalid DPI value".into(),
            ));
        }
        Ok(x_dpi as u32)
    }
}
