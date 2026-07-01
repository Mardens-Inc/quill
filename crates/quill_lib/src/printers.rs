use crate::errors::QuillError;
use crate::printer_handle::PrinterHandle;
use crate::printer_info::PrinterInfo;
use windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;
use windows::Win32::Graphics::Printing::{
    EnumPrintersA, OpenPrinterA, PRINTER_DEFAULTSA, PRINTER_ENUM_LOCAL, PRINTER_HANDLE,
    PRINTER_INFO_2A,
};
use tracing::{debug, error, info, warn};
use windows::core::PCSTR;
pub struct Printers;

impl Printers {
    /// Gets a list of available printers on the device.
    pub fn get_available_printers() -> windows::core::Result<Vec<PrinterInfo>> {
        const LEVEL: u32 = 2;
        let mut needed: u32 = 0;
        let mut returned: u32 = 0;

        debug!("Enumerating local printers (EnumPrintersA, level {LEVEL})");

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
            error!("EnumPrintersA probe failed: {e}");
            return Err(e);
        }
        if needed == 0 {
            info!("No local printers found");
            return Ok(Vec::new());
        }

        let count = (needed as usize).div_ceil(size_of::<PRINTER_INFO_2A>());
        debug!(
            "EnumPrintersA requires {needed} bytes; allocating {count} PRINTER_INFO_2A slot(s)"
        );
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
            )
            .inspect_err(|e| error!("EnumPrintersA failed to fill buffer: {e}"))?;
        }

        let head = backing.as_ptr();
        let mut out = Vec::with_capacity(returned as usize);
        for i in 0..returned as usize {
            let r = unsafe { &*head.add(i) };
            let mut info = PrinterInfo {
                server_name: crate::pstr_opt(r.pServerName),
                printer_name: crate::pstr_req(r.pPrinterName),
                share_name: crate::pstr_opt(r.pShareName),
                port_name: crate::pstr_req(r.pPortName),
                driver_name: crate::pstr_req(r.pDriverName),
                comment: crate::pstr_opt(r.pComment),
                location: crate::pstr_opt(r.pLocation),
                print_processor: crate::pstr_req(r.pPrintProcessor),
                datatype: crate::pstr_req(r.pDatatype),
                parameters: crate::pstr_opt(r.pParameters),
                sep_file: crate::pstr_opt(r.pSepFile),
                attributes: r.Attributes,
                priority: r.Priority,
                default_priority: r.DefaultPriority,
                start_time: r.StartTime,
                until_time: r.UntilTime,
                status: r.Status.into(),
                jobs: r.cJobs,
                average_ppm: r.AveragePPM,
                dpi: None,
            };
            info.dpi = info.get_dpi().ok();
            out.push(info);
        }
        info!("Enumerated {} local printer(s)", out.len());
        debug!(
            "Printers found: {:?}",
            out.iter().map(|p| p.printer_name.as_str()).collect::<Vec<_>>()
        );
        Ok(out)
    }

    pub fn get_printer_handle(name: impl AsRef<str>) -> Result<PrinterHandle, QuillError> {
        let name = name.as_ref();
        debug!("Opening printer handle for '{name}'");
        let mut handle: PRINTER_HANDLE = PRINTER_HANDLE::default();
        let name_c = crate::to_cstring(name)?;
        let result = unsafe {
            OpenPrinterA(
                PCSTR(name_c.as_ptr().cast()),
                &mut handle,
                Some(&PRINTER_DEFAULTSA::default()),
            )
        };

        if let Err(e) = result {
            error!("OpenPrinterA failed for '{name}': {}", e.message());
            return Err(QuillError::PrinterHandleError(e.message()));
        }

        match Self::get_available_printers()?
            .iter().find(|p| p.printer_name.eq(name))
        {
            Some(info) => {
                info!("Opened printer handle for '{name}'");
                Ok(PrinterHandle {
                    info: info.clone(),
                    handle: Some(handle),
                    supported_data_types: Vec::new(),
                })
            }
            None => {
                warn!(
                    "OpenPrinterA succeeded for '{name}' but it was not found in the enumerated printer list"
                );
                Err(QuillError::PrinterHandleError("Failed to find printer by name".into()))
            }
        }
    }
}
