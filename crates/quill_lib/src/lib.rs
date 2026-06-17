use std::ffi::{CStr, CString};
use windows::core::{PCSTR, PSTR};
use crate::errors::QuillError;

pub mod printers;
pub mod printer_status;
mod printer_info;
pub mod errors;
pub mod printer_handle;

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

fn str_pcstr(value: impl Into<String>) -> Result<PCSTR, QuillError> {
    let string = value.into();
    let c_string = CString::new(string)
        .map_err(|e| QuillError::StringConversionError("String".into(), "CString".into(), e))?;
    Ok(PCSTR(c_string.as_ptr().cast()))
}
fn str_pstr(value: impl Into<String>) -> Result<PSTR, QuillError> {
    let string = value.into();
    let c_string = CString::new(string)
        .map_err(|e| QuillError::StringConversionError("String".into(), "CString".into(), e))?;
    Ok(PSTR(c_string.as_ptr().cast::<u8>() as *mut u8))
}