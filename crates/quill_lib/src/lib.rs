use crate::errors::QuillError;
use std::ffi::{CStr, CString};
use tracing::error;
use windows::core::PSTR;

pub mod errors;
pub mod image_processing;
pub mod print_orientation;
pub mod printer_handle;
pub mod printer_info;
pub mod printer_status;
pub mod printers;
pub mod stock;

/// Null PSTR -> None; otherwise decode the ANSI (code-page) C string.
fn pstr_opt(p: PSTR) -> Option<String> {
    if p.is_null() {
        None
    } else {
        unsafe {
            Some(
                CStr::from_ptr(p.0 as *const i8)
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }
}

/// For fields the API guarantees are present; null collapses to "".
fn pstr_req(p: PSTR) -> String {
    pstr_opt(p).unwrap_or_default()
}

fn to_cstring(value: impl Into<String>) -> Result<CString, QuillError> {
    CString::new(value.into()).map_err(|e| {
        error!(
            "Failed to convert string to CString (interior NUL at byte {}): {e}",
            e.nul_position()
        );
        QuillError::StringConversionError("String".into(), "CString".into(), e)
    })
}
