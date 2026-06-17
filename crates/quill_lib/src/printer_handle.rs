use crate::errors::QuillError;
use image::EncodableLayout;
use windows::Win32::Graphics::Printing::{AddJobA, PRINTER_HANDLE};

pub struct PrinterHandle {
    pub(crate) handle: Option<PRINTER_HANDLE>,
}

//pub struct AddJobInfo{
//
//}

impl PrinterHandle {
    pub fn print_bytes(&self, bytes: &[u8]) -> Result<(), QuillError> {
        if let Some(handle) = self.handle {
            let mut bytes_required_for_print_job = 0u32;

            /*
               Probe: pData = None.  This fails with `ERROR_INSUFFICIENT_BUFFER`,
               but this still fills the bytes required variable.
               The `windows` crate maps the BOOL=0 to an error (Err),
               so ignoring the error is required.
            */
            let _ = unsafe {
                AddJobA(
                    handle,
                    /*
                       To future me, this is the version of the addjob info struct,
                       the only version currently is ADDJOB_INFO_1,
                       so 1 is the only valid option?!?!?!?
                       See https://learn.microsoft.com/en-us/windows/win32/printdocs/addjob for more information
                       Author:  Drew Chase
                       Date:    06/17/2026
                    */
                    1,
                    None,
                    &mut bytes_required_for_print_job,
                )
            };

            let mut buffer = vec![0u8; bytes_required_for_print_job as usize];
            unsafe {
                AddJobA(
                    handle,
                    1,
                    Some(&mut buffer),
                    &mut bytes_required_for_print_job,
                )
            }
            .ok()
            .map_err(QuillError::WindowsError)?;

            Ok(())
        } else {
            Err(QuillError::PrinterNotOpenedError)
        }
    }

    pub fn print_image(&self, image: image::RgbImage) -> Result<(), QuillError> {
        self.print_bytes(image.to_vec().as_bytes())
    }

    pub fn close(&mut self) -> Result<(), QuillError> {
        if let Some(handle) = self.handle {
            let result = unsafe {
                windows::Win32::Graphics::Printing::ClosePrinter(handle)
                    .map_err(QuillError::WindowsError)
            };
            self.handle = None;
            result
        } else {
            Err(QuillError::PrinterNotOpenedError)
        }
    }
}
