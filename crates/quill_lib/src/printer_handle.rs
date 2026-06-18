use crate::errors::QuillError;
use crate::print_orientation::PageOrientation;
use crate::printer_info::PrinterInfo;
use crate::stock::Stock;
use crate::{image_processing, to_cstring};
use windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;
use windows::Win32::Graphics::Printing::{
    DATATYPES_INFO_1A, DOC_INFO_1A, EndDocPrinter, EndPagePrinter, EnumPrintProcessorDatatypesA,
    PRINTER_HANDLE, StartDocPrinterA, StartPagePrinter, WritePrinter,
};
use tracing::{debug, error, info, warn};
use windows::core::{PCSTR, PSTR};

pub struct PrinterHandle {
    pub(crate) handle: Option<PRINTER_HANDLE>,
    pub(crate) info: PrinterInfo,
    pub(crate) supported_data_types: Vec<String>,
}

impl PrinterHandle {
    pub fn print_bytes(
        &self,
        job_name: impl AsRef<str>,
        bytes: &[u8],
        data_type: Option<String>,
    ) -> Result<(), QuillError> {
        let Some(handle) = self.handle else {
            warn!(
                "print_bytes called on '{}' but the printer handle is not open",
                self.info.printer_name
            );
            return Err(QuillError::PrinterNotOpenedError);
        };

        let job_name = job_name.as_ref();
        let datatype_str = data_type.unwrap_or_else(|| "RAW".into());
        debug!(
            "Spooling {} byte(s) to printer '{}' (job='{job_name}', datatype='{datatype_str}')",
            bytes.len(),
            self.info.printer_name
        );

        let doc_name = to_cstring(job_name)?;
        let datatype = to_cstring(datatype_str)?;

        let job_id = unsafe {
            StartDocPrinterA(
                handle,
                /*
                   To future me, this is the version of the doc info struct,
                   the only version currently is DOC_INFO_1,
                   so 1 is the only valid option?!?!?!?
                   See https://learn.microsoft.com/en-us/windows/win32/printdocs/startdocprinter for more information
                   Author:  Drew Chase
                   Date:    06/18/2026
                */
                1,
                &DOC_INFO_1A {
                    pDocName: PSTR(doc_name.as_ptr() as *mut u8),
                    pOutputFile: PSTR::null(),
                    pDatatype: PSTR(datatype.as_ptr() as *mut u8),
                },
            )
        };
        if job_id == 0 {
            let err = windows::core::Error::from_thread();
            error!(
                "StartDocPrinterA failed for job '{job_name}' on '{}': {err}",
                self.info.printer_name
            );
            return Err(QuillError::WindowsError(err));
        }
        debug!(
            "Started print job '{job_name}' on '{}' (job_id={job_id})",
            self.info.printer_name
        );

        let write = (|| -> Result<(), QuillError> {
            unsafe {
                StartPagePrinter(handle)
                    .ok()
                    .map_err(QuillError::WindowsError)?
            };

            /*
                WritePrinter may accept fewer bytes than requested,
                so loop until the whole buffer is spooled.
            */
            let mut offset = 0usize;
            while offset < bytes.len() {
                let chunk = &bytes[offset..];
                let mut written: u32 = 0;
                unsafe {
                    WritePrinter(
                        handle,
                        chunk.as_ptr() as *const core::ffi::c_void,
                        chunk.len() as u32,
                        &mut written,
                    )
                    .ok()
                    .map_err(QuillError::WindowsError)?;
                }
                if written == 0 {
                    let err = windows::core::Error::from_thread();
                    error!("WritePrinter wrote 0 bytes at offset {offset} for job_id={job_id}: {err}");
                    return Err(QuillError::WindowsError(err));
                }
                offset += written as usize;
            }
            debug!("Spooled {offset} byte(s) for job_id={job_id}");

            unsafe {
                EndPagePrinter(handle)
                    .ok()
                    .map_err(QuillError::WindowsError)
            }
        })();

        if let Err(e) = &write {
            error!("Failed while writing page data for job_id={job_id}: {e}");
        }

        let end = unsafe { EndDocPrinter(handle).ok().map_err(QuillError::WindowsError) };
        if let Err(e) = &end {
            error!("EndDocPrinter failed for job_id={job_id}: {e}");
        }

        let result = write.and(end);
        if result.is_ok() {
            info!(
                "Finished print job '{job_name}' on '{}' ({} byte(s), job_id={job_id})",
                self.info.printer_name,
                bytes.len()
            );
        }
        result
    }

    pub fn print_png(
        &self,
        job_name: impl AsRef<str>,
        png: &image::RgbImage,
        stock: Stock,
        orientation: PageOrientation,
        scale: f32,
    ) -> Result<(), QuillError> {
        const BRIGHTNESS_THREASHOLD: u32 = 191; // ~25% brightness -> 255-(255*0.25) = 191.25
        const DOTS_PER_MM: f32 = 8.0;

        debug!(
            "print_png on '{}': source {}x{} px, stock {:.2}x{:.2} mm, orientation {:?}, scale {scale}",
            self.info.printer_name,
            png.width(),
            png.height(),
            stock.width_mm(),
            stock.height_mm(),
            orientation
        );

        let to_dots = |mm: f32| (mm * DOTS_PER_MM).round() as u32;

        let label_w_dots = to_dots(stock.width_mm()).max(1);
        let label_h_dots = to_dots(stock.height_mm()).max(1);
        let left_dots = to_dots(stock.exposed_liner_left_mm());
        let right_dots = to_dots(stock.exposed_liner_right_mm());
        let avail_w = label_w_dots.saturating_sub(left_dots + right_dots).max(1);
        let avail_h = label_h_dots;
        debug!(
            "Label geometry: {label_w_dots}x{label_h_dots} dots, liner left={left_dots} right={right_dots}, printable area {avail_w}x{avail_h} dots"
        );

        let rotated = image_processing::prepare_image(png, orientation, 1.0);
        if rotated.width() == 0 || rotated.height() == 0 {
            warn!(
                "Prepared image is empty ({}x{}); skipping print job '{}'",
                rotated.width(),
                rotated.height(),
                job_name.as_ref()
            );
            return Ok(());
        }
        let zoom = if scale > 0.0 { scale } else { 1.0 };
        let fit = (avail_w as f32 / rotated.width() as f32)
            .min(avail_h as f32 / rotated.height() as f32)
            * zoom;
        let target_w = ((rotated.width() as f32 * fit).round() as u32).max(1);
        let target_h = ((rotated.height() as f32 * fit).round() as u32).max(1);
        debug!(
            "Scaling prepared image {}x{} -> {target_w}x{target_h} dots (fit factor {fit:.4}, zoom {zoom})",
            rotated.width(),
            rotated.height()
        );
        let source = image::imageops::resize(
            &rotated,
            target_w,
            target_h,
            image::imageops::FilterType::Lanczos3,
        );
        let width = source.width();
        let height = source.height();

        let width_bytes = width.div_ceil(8);
        let mut raster = Vec::with_capacity((width_bytes * height) as usize);
        for y in 0..height {
            for byte_col in 0..width_bytes {
                let mut byte = 0xFFu8;
                for bit in 0..8u32 {
                    let x = byte_col * 8 + bit;
                    if x >= width {
                        break;
                    }
                    let [r, g, b] = source.get_pixel(x, y).0;
                    let luma = (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000;
                    if luma < BRIGHTNESS_THREASHOLD {
                        byte &= !(0x80u8 >> bit); // MSB is the left-most pixel; 0 = ink
                    }
                }
                raster.push(byte);
            }
        }

        let width_mm = stock.width_mm();
        let height_mm = stock.height_mm();
        let gap_mm = stock.gap_mm();
        let x_offset = left_dots + (avail_w - width) / 2;
        let y_offset = (label_h_dots - height) / 2;
        debug!(
            "Built TSPL monochrome raster: {width}x{height} px ({} bytes, {width_bytes} bytes/row) at offset ({x_offset},{y_offset})",
            raster.len()
        );

        let mut job = Vec::new();
        job.extend_from_slice(format!("SIZE {width_mm:.2} mm,{height_mm:.2} mm\r\n").as_bytes());
        job.extend_from_slice(format!("GAP {gap_mm:.2} mm,0 mm\r\n").as_bytes());
        job.extend_from_slice(b"CLS\r\n");
        job.extend_from_slice(
            format!("BITMAP {x_offset},{y_offset},{width_bytes},{height},0,").as_bytes(),
        );
        job.extend_from_slice(&raster);
        job.extend_from_slice(b"\r\n");
        job.extend_from_slice(b"PRINT 1,1\r\n");

        self.print_bytes(job_name, &job, None)
    }

    pub fn get_print_processor_data_types(&mut self) -> Result<Vec<String>, QuillError> {
        // DATATYPES_INFO_1 is the only defined level for this API.
        const LEVEL: u32 = 1;

        debug!(
            "Enumerating supported datatypes for print processor '{}'",
            self.info.print_processor
        );

        let processor = to_cstring(self.info.print_processor.as_str())?;
        let processor_name = PCSTR(processor.as_ptr().cast());

        let mut needed: u32 = 0;
        let mut returned: u32 = 0;

        let probe = unsafe {
            EnumPrintProcessorDatatypesA(
                PCSTR::null(),
                processor_name,
                LEVEL,
                None,
                &mut needed,
                &mut returned,
            )
        };
        if let Err(e) = probe.ok()
            && e.code() != ERROR_INSUFFICIENT_BUFFER.to_hresult()
        {
            error!(
                "EnumPrintProcessorDatatypesA probe failed for processor '{}': {e}",
                self.info.print_processor
            );
            return Err(QuillError::WindowsError(e));
        }
        if needed == 0 {
            debug!(
                "Print processor '{}' reports no supported datatypes",
                self.info.print_processor
            );
            self.supported_data_types = Vec::new();
            return Ok(Vec::new());
        }

        let count = (needed as usize).div_ceil(size_of::<DATATYPES_INFO_1A>());
        let mut backing: Vec<DATATYPES_INFO_1A> = Vec::with_capacity(count);
        unsafe {
            let bytes = std::slice::from_raw_parts_mut(
                backing.as_mut_ptr() as *mut u8,
                count * size_of::<DATATYPES_INFO_1A>(),
            );
            EnumPrintProcessorDatatypesA(
                PCSTR::null(),
                processor_name,
                LEVEL,
                Some(bytes),
                &mut needed,
                &mut returned,
            )
            .ok()
            .inspect_err(|e| {
                error!("EnumPrintProcessorDatatypesA failed to fill buffer: {e}")
            })
            .map_err(QuillError::WindowsError)?;
        }

        let head = backing.as_ptr();
        let mut out = Vec::with_capacity(returned as usize);
        for i in 0..returned as usize {
            let r = unsafe { &*head.add(i) };
            out.push(crate::pstr_req(r.pName));
        }

        info!(
            "Print processor '{}' supports {} datatype(s): {out:?}",
            self.info.print_processor,
            out.len()
        );
        self.supported_data_types = out.clone();
        Ok(out)
    }

    pub fn close(&mut self) -> Result<(), QuillError> {
        if let Some(handle) = self.handle {
            debug!("Closing printer handle for '{}'", self.info.printer_name);
            let result = unsafe {
                windows::Win32::Graphics::Printing::ClosePrinter(handle)
                    .map_err(QuillError::WindowsError)
            };
            self.handle = None;
            match &result {
                Ok(()) => info!("Closed printer handle for '{}'", self.info.printer_name),
                Err(e) => error!(
                    "ClosePrinter failed for '{}': {e}",
                    self.info.printer_name
                ),
            }
            result
        } else {
            warn!(
                "close called on '{}' but the printer handle was already closed",
                self.info.printer_name
            );
            Err(QuillError::PrinterNotOpenedError)
        }
    }
}
