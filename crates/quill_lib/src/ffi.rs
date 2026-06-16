use std::ffi::CStr;
use windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;
use windows::Win32::Graphics::Printing::{EnumPrintersA, PRINTER_ENUM_LOCAL, PRINTER_INFO_2A};
use windows::core::{PCSTR, PSTR};

pub struct Printers;

#[derive(Debug)]
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
    /// TODO: Replace this with an enum instead of a flag
    pub status: u32,
    pub jobs: u32,
    pub average_ppm: u32,
}

impl Printers {
    pub fn get_available_printers() -> windows::core::Result<Vec<PrinterInfo>> {
        const LEVEL: u32 = 2;
        let mut needed: u32 = 0;
        let mut returned: u32 = 0;

        let probe = unsafe {
            EnumPrintersA(
                PRINTER_ENUM_LOCAL,
                PCSTR::null(),
                LEVEL,
                None,
                &mut needed,
                &mut returned,
            )
        };
        if let Err(e) = probe
            && e.code() != ERROR_INSUFFICIENT_BUFFER.to_hresult()
        {
            return Err(e);
        }
        if needed == 0 {
            return Ok(Vec::new());
        }

        let count = (needed as usize).div_ceil(size_of::<PRINTER_INFO_2A>());
        let mut backing: Vec<PRINTER_INFO_2A> = Vec::with_capacity(count);

        unsafe {
            let bytes = std::slice::from_raw_parts_mut(
                backing.as_mut_ptr() as *mut u8,
                count * size_of::<PRINTER_INFO_2A>(),
            );
            EnumPrintersA(
                PRINTER_ENUM_LOCAL,
                PCSTR::null(),
                LEVEL,
                Some(bytes),
                &mut needed,
                &mut returned,
            )?;
        }

        let head = backing.as_ptr();
        let mut out = Vec::with_capacity(returned as usize);
        for i in 0..returned as usize {
            let r = unsafe { &*head.add(i) };
            out.push(PrinterInfo {
                server_name: pstr_opt(r.pServerName),
                printer_name: pstr_req(r.pPrinterName),
                share_name: pstr_opt(r.pShareName),
                port_name: pstr_req(r.pPortName),
                driver_name: pstr_req(r.pDriverName),
                comment: pstr_opt(r.pComment),
                location: pstr_opt(r.pLocation),
                print_processor: pstr_req(r.pPrintProcessor),
                datatype: pstr_req(r.pDatatype),
                parameters: pstr_opt(r.pParameters),
                sep_file: pstr_opt(r.pSepFile),
                attributes: r.Attributes,
                priority: r.Priority,
                default_priority: r.DefaultPriority,
                start_time: r.StartTime,
                until_time: r.UntilTime,
                status: r.Status,
                jobs: r.cJobs,
                average_ppm: r.AveragePPM,
            });
        }
        Ok(out)
    }

    pub fn get_printer_handle(_name: impl AsRef<str>) {}
}

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
