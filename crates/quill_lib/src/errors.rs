use std::ffi::NulError;

#[derive(thiserror::Error, Debug)]
pub enum QuillError {
    #[error("Failed to convert string types: {0} -> {1}\n{2:?}")]
    StringConversionError(String, String, NulError),
    #[error("Failed to get a printers handle: {0}")]
    PrinterHandleError(String),
    #[error("The printer handle has not been opened yet.")]
    PrinterNotOpenedError,
    #[error(transparent)]
    WindowsError(#[from] windows::core::Error),
}
