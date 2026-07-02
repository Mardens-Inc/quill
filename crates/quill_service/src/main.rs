use std::ffi::OsString;
use windows_service::{Result, define_windows_service, service_dispatcher};

define_windows_service!(ffi_service_main, service_main);

fn service_main(_arguments: Vec<OsString>) {

}

fn main() -> Result<()> {
    service_dispatcher::start("quill-autoprint-service", ffi_service_main)?;
    Ok(())
}
