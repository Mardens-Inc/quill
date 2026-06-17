use crate::errors::QuillError;
use windows::Win32::Graphics::Printing::PRINTER_HANDLE;

pub struct PrinterHandle {
    pub(crate) handle: Option<PRINTER_HANDLE>,
}

impl PrinterHandle {
    pub fn close(&mut self) -> Result<(), QuillError> {
        if let Some(handle) = self.handle {
            let result = unsafe {
                windows::Win32::Graphics::Printing::ClosePrinter(handle)
                    .map_err(QuillError::WindowsError)
            };
            self.handle = None;
            result
        } else {
            Err(QuillError::PrinterHandleError(String::from(
                "The printer handle has not been opened yet.",
            )))
        }
    }
}
