use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u32)]
pub enum PrinterStatus {
    /// The printer is idle
    Idle = 0,
    /// The printer is paused.
    Paused = 1,
    /// The printer is in an error state.
    Error = 1 << 1,
    /// The printer is being deleted.
    PendingDeletion = 1 << 2,
    /// Paper is jammed in the printer
    PaperJam = 1 << 3,
    /// The printer is out of paper.
    PaperOut = 1 << 4,
    /// The printer is in a manual feed state.
    ManualFeed = 1 << 5,
    /// The printer has a paper problem.
    PaperProblem = 1 << 6,
    /// The printer is offline.
    Offline = 1 << 7,
    /// The printer is in an active input/output state
    IoActive = 1 << 8,
    /// The printer is busy.
    Busy = 1 << 9,
    /// The printer is printing.
    Printing = 1 << 10,
    /// The printer's output bin is full.
    OutputBinFull = 1 << 11,
    /// The printer is not available for printing.
    NotAvailable = 1 << 12,
    /// The printer is waiting.
    Waiting = 1 << 13,
    /// The printer is processing a print job.
    Processing = 1 << 14,
    /// The printer is initializing.
    Initializing = 1 << 15,
    /// The printer is warming up.
    WarmingUp = 1 << 16,
    /// The printer is low on toner.
    TonerLow = 1 << 17,
    /// The printer is out of toner.
    NoToner = 1 << 18,
    /// The printer cannot print the current page.
    PagePunt = 1 << 19,
    /// The printer requires user intervention.
    UserIntervention = 1 << 20,
    /// The printer has run out of memory.
    OutOfMemory = 1 << 21,
    /// The printer door is open.
    DoorOpen = 1 << 22,
    /// The printer server status is unknown.
    ServerUnknown = 1 << 23,
    /// The printer is in power save mode.
    PowerSave = 1 << 24,
    /// The printer server is offline.
    ServerOffline = 1 << 25,
    /// The printer driver needs to be updated.
    DriverUpdateNeeded = 1 << 26,
}

impl From<u32> for PrinterStatus {
    fn from(value: u32) -> Self {
        match value {
            0 => PrinterStatus::Idle,
            1 => PrinterStatus::Paused,
            2 => PrinterStatus::Error,
            4 => PrinterStatus::PendingDeletion,
            8 => PrinterStatus::PaperJam,
            16 => PrinterStatus::PaperOut,
            32 => PrinterStatus::ManualFeed,
            64 => PrinterStatus::PaperProblem,
            128 => PrinterStatus::Offline,
            256 => PrinterStatus::IoActive,
            512 => PrinterStatus::Busy,
            1024 => PrinterStatus::Printing,
            2048 => PrinterStatus::OutputBinFull,
            4096 => PrinterStatus::NotAvailable,
            8192 => PrinterStatus::Waiting,
            16384 => PrinterStatus::Processing,
            32768 => PrinterStatus::Initializing,
            65536 => PrinterStatus::WarmingUp,
            131072 => PrinterStatus::TonerLow,
            262144 => PrinterStatus::NoToner,
            524288 => PrinterStatus::PagePunt,
            1048576 => PrinterStatus::UserIntervention,
            2097152 => PrinterStatus::OutOfMemory,
            4194304 => PrinterStatus::DoorOpen,
            8388608 => PrinterStatus::ServerUnknown,
            16777216 => PrinterStatus::PowerSave,
            33554432 => PrinterStatus::ServerOffline,
            67108864 => PrinterStatus::DriverUpdateNeeded,
            _ => {
                tracing::error!(
                    "Unknown printer status value {value:#010x}; expected a single PRINTER_STATUS_* flag"
                );
                panic!("Unknown printer status value: {}", value)
            }
        }
    }
}
