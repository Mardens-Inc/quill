use crate::errors::QuillError;
use crate::print_orientation::PageOrientation;
use crate::printer_info::PrinterInfo;
use crate::{image_processing, to_cstring};
use windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;
use windows::Win32::Graphics::Printing::{
    DATATYPES_INFO_1A, DOC_INFO_1A, EndDocPrinter, EndPagePrinter, EnumPrintProcessorDatatypesA,
    PRINTER_HANDLE, StartDocPrinterA, StartPagePrinter, WritePrinter,
};
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
            return Err(QuillError::PrinterNotOpenedError);
        };

        let doc_name = to_cstring(job_name.as_ref())?;
        let datatype = to_cstring(data_type.unwrap_or("RAW".into()))?;

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
            return Err(QuillError::WindowsError(windows::core::Error::from_thread()));
        }

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
                    return Err(QuillError::WindowsError(windows::core::Error::from_thread()));
                }
                offset += written as usize;
            }

            unsafe {
                EndPagePrinter(handle)
                    .ok()
                    .map_err(QuillError::WindowsError)
            }
        })();

        let end = unsafe { EndDocPrinter(handle).ok().map_err(QuillError::WindowsError) };

        write.and(end)
    }

    pub fn print_png(
        &self,
        job_name: impl AsRef<str>,
        png: &image::RgbImage,
        orentation: PageOrientation,
        scale: f32,
    ) -> Result<(), QuillError> {
        const BRIGHTNESS_THREASHOLD: u32 = 191; // ~25% brightness -> 255-(255*0.25) = 191.25
        const DOTS_PER_MM: u32 = 8;

        let source = image_processing::prepare_image(png, orentation, scale);
        let width = source.width();
        let height = source.height();
        if width == 0 || height == 0 {
            return Ok(());
        }

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

        let width_mm = width.div_ceil(DOTS_PER_MM);
        let height_mm = height.div_ceil(DOTS_PER_MM);

        let mut job = Vec::new();
        job.extend_from_slice(format!("SIZE {width_mm} mm,{height_mm} mm\r\n").as_bytes());
        job.extend_from_slice(b"GAP 0 mm,0 mm\r\n");
        job.extend_from_slice(b"CLS\r\n");
        job.extend_from_slice(format!("BITMAP 0,0,{width_bytes},{height},0,").as_bytes());
        job.extend_from_slice(&raster);
        job.extend_from_slice(b"\r\n");
        job.extend_from_slice(b"PRINT 1,1\r\n");

        self.print_bytes(job_name, &job, None)
    }

    pub fn get_print_processor_data_types(&mut self) -> Result<Vec<String>, QuillError> {
        // DATATYPES_INFO_1 is the only defined level for this API.
        const LEVEL: u32 = 1;

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
            return Err(QuillError::WindowsError(e));
        }
        if needed == 0 {
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
            .map_err(QuillError::WindowsError)?;
        }

        let head = backing.as_ptr();
        let mut out = Vec::with_capacity(returned as usize);
        for i in 0..returned as usize {
            let r = unsafe { &*head.add(i) };
            out.push(crate::pstr_req(r.pName));
        }

        self.supported_data_types = out.clone();
        Ok(out)
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
