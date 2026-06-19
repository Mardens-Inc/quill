use crate::theme::design;
use crate::widgets::sections;
use crate::widgets::titlebar::{self, TitlebarMessage};
use iced::widget::{Space, column, container, mouse_area, row, scrollable, text};
use iced::window::Direction;
use iced::window::settings::PlatformSpecific;
use iced::window::settings::platform::CornerPreference;
use iced::{
    Background, Border, Color, Element, Event, Length, Padding, Point, Size, Subscription, Task,
    event, mouse, time, window,
};
use std::fmt;
use std::time::Duration;

// ─── Domain enums ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Section {
    #[default]
    Printer,
    Stocks,
    PrintSettings,
    Server,
    Logs,
    About,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DpiMode {
    #[default]
    Auto,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DpiOverride {
    #[default]
    Dpi203,
    Dpi300,
    Dpi600,
}

impl fmt::Display for DpiOverride {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DpiOverride::Dpi203 => write!(f, "203 dpi"),
            DpiOverride::Dpi300 => write!(f, "300 dpi"),
            DpiOverride::Dpi600 => write!(f, "600 dpi"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Printer {
    pub name: String,
    pub meta: String,
    pub online: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stock {
    pub id: u32,
    pub name: String,
    pub width_mm: f32,
    pub height_mm: f32,
    pub gap_mm: f32,
    pub liner_left_mm: f32,
    pub liner_right_mm: f32,
}

#[derive(Debug, Clone)]
pub struct StockModal {
    pub editing_id: Option<u32>,
    pub name: String,
    pub width: String,
    pub height: String,
    pub gap: String,
    pub liner_left: String,
    pub liner_right: String,
}

impl Default for StockModal {
    fn default() -> Self {
        Self::new()
    }
}

impl StockModal {
    pub fn new() -> Self {
        StockModal {
            editing_id: None,
            name: String::new(),
            width: String::new(),
            height: String::new(),
            gap: String::new(),
            liner_left: String::new(),
            liner_right: String::new(),
        }
    }

    pub fn from_stock(stock: &Stock) -> Self {
        StockModal {
            editing_id: Some(stock.id),
            name: stock.name.clone(),
            width: stock.width_mm.to_string(),
            height: stock.height_mm.to_string(),
            gap: stock.gap_mm.to_string(),
            liner_left: stock.liner_left_mm.to_string(),
            liner_right: stock.liner_right_mm.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrintSpeed {
    S1,
    S2,
    S3,
    #[default]
    S4,
    S5,
    S6,
}

impl fmt::Display for PrintSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrintSpeed::S1 => write!(f, "1 ips"),
            PrintSpeed::S2 => write!(f, "2 ips"),
            PrintSpeed::S3 => write!(f, "3 ips"),
            PrintSpeed::S4 => write!(f, "4 ips"),
            PrintSpeed::S5 => write!(f, "5 ips"),
            PrintSpeed::S6 => write!(f, "6 ips"),
        }
    }
}

pub const PRINT_SPEEDS: &[PrintSpeed] = &[
    PrintSpeed::S1,
    PrintSpeed::S2,
    PrintSpeed::S3,
    PrintSpeed::S4,
    PrintSpeed::S5,
    PrintSpeed::S6,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    Portrait,
    Landscape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "Trace"),
            LogLevel::Debug => write!(f, "Debug"),
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Warn => write!(f, "Warn"),
            LogLevel::Error => write!(f, "Error"),
        }
    }
}

pub const LOG_LEVELS: &[LogLevel] = &[
    LogLevel::Trace,
    LogLevel::Debug,
    LogLevel::Info,
    LogLevel::Warn,
    LogLevel::Error,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimeRange {
    #[default]
    Last15m,
    Last1h,
    Last6h,
    Last24h,
    All,
}

impl fmt::Display for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeRange::Last15m => write!(f, "Last 15 min"),
            TimeRange::Last1h => write!(f, "Last 1 hour"),
            TimeRange::Last6h => write!(f, "Last 6 hours"),
            TimeRange::Last24h => write!(f, "Last 24 hours"),
            TimeRange::All => write!(f, "All time"),
        }
    }
}

pub const TIME_RANGES: &[TimeRange] = &[
    TimeRange::Last15m,
    TimeRange::Last1h,
    TimeRange::Last6h,
    TimeRange::Last24h,
    TimeRange::All,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Severity {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Trace => write!(f, "TRACE"),
            Severity::Debug => write!(f, "DEBUG"),
            Severity::Info => write!(f, "INFO"),
            Severity::Warn => write!(f, "WARN"),
            Severity::Error => write!(f, "ERROR"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SevFilter {
    pub trace: bool,
    pub debug: bool,
    pub info: bool,
    pub warn: bool,
    pub error: bool,
}

impl SevFilter {
    pub fn all_off() -> Self {
        SevFilter::default()
    }
    pub fn matches(&self, sev: Severity) -> bool {
        let any_on = self.trace || self.debug || self.info || self.warn || self.error;
        if !any_on {
            return true;
        }
        match sev {
            Severity::Trace => self.trace,
            Severity::Debug => self.debug,
            Severity::Info => self.info,
            Severity::Warn => self.warn,
            Severity::Error => self.error,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub id: u32,
    pub timestamp: String,
    pub severity: Severity,
    pub source: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HelperStatus {
    Running,
    #[default]
    Stopped,
    Restarting,
}

// ─── Application state ───────────────────────────────────────────────────────

pub struct ConfigurationApp {
    // Window management (keep existing)
    pub window_id: Option<window::Id>,
    pub window_size: Size,
    pub cursor: Point,
    pub resize_dir: Option<Direction>,

    // Navigation
    pub section: Section,

    // Printer
    pub scanning: bool,
    pub printers: Vec<Printer>,
    pub selected_printer: Option<String>,
    pub dpi_mode: DpiMode,
    pub dpi_auto: u32,
    pub dpi_override: DpiOverride,
    pub testing: bool,
    pub last_tested: Option<String>,
    pub test_msg: String,
    pub test_ok: bool,

    // Stocks
    pub stocks: Vec<Stock>,
    pub modal: Option<StockModal>,
    pub next_stock_id: u32,

    // Print settings
    pub density: u8,
    pub speed: PrintSpeed,
    pub orientation: Orientation,
    pub scale: u8,
    pub advanced_open: bool,
    pub mono_threshold: u8,

    // Server
    pub port: String,
    pub token: String,
    pub token_visible: bool,
    pub token_copied: bool,
    pub origins: Vec<String>,
    pub new_origin: String,
    pub restarting: bool,
    pub restart_msg: String,
    pub helper_status: HelperStatus,

    // Logs
    pub log_level: LogLevel,
    pub search: String,
    pub time_range: TimeRange,
    pub sev_filter: SevFilter,
    pub live_tail: bool,
    pub logs: Vec<LogEntry>,
    pub expanded_log: Option<u32>,
    pub next_log_id: u32,
    pub remote_enabled: bool,
    pub remote_url: String,
    pub remote_auth: String,
    pub remote_sending: bool,
    pub remote_status: String,

    // Toast
    pub toast: Option<String>,
}

impl Default for ConfigurationApp {
    fn default() -> Self {
        ConfigurationApp {
            window_id: None,
            window_size: Size::new(1200.0, 800.0),
            cursor: Point::ORIGIN,
            resize_dir: None,
            section: Section::Printer,
            scanning: false,
            printers: Vec::new(),
            selected_printer: None,
            dpi_mode: DpiMode::Auto,
            dpi_auto: 203,
            dpi_override: DpiOverride::Dpi203,
            testing: false,
            last_tested: None,
            test_msg: String::new(),
            test_ok: false,
            stocks: vec![
                Stock {
                    id: 1,
                    name: "4×6 Shipping".to_string(),
                    width_mm: 101.6,
                    height_mm: 152.4,
                    gap_mm: 3.0,
                    liner_left_mm: 1.5,
                    liner_right_mm: 1.5,
                },
                Stock {
                    id: 2,
                    name: "2×1 Barcode".to_string(),
                    width_mm: 50.8,
                    height_mm: 25.4,
                    gap_mm: 2.0,
                    liner_left_mm: 1.0,
                    liner_right_mm: 1.0,
                },
                Stock {
                    id: 3,
                    name: "Jewelry Tag".to_string(),
                    width_mm: 25.4,
                    height_mm: 50.8,
                    gap_mm: 2.5,
                    liner_left_mm: 0.5,
                    liner_right_mm: 0.5,
                },
                Stock {
                    id: 4,
                    name: "Price Tag 30×20".to_string(),
                    width_mm: 30.0,
                    height_mm: 20.0,
                    gap_mm: 2.0,
                    liner_left_mm: 1.0,
                    liner_right_mm: 1.0,
                },
            ],
            modal: None,
            next_stock_id: 5,
            density: 8,
            speed: PrintSpeed::S4,
            orientation: Orientation::Portrait,
            scale: 100,
            advanced_open: false,
            mono_threshold: 128,
            port: "9100".to_string(),
            token: "qk_live_8f2c7a91d4e6b03597af".to_string(),
            token_visible: false,
            token_copied: false,
            origins: vec![
                "https://labels.quillco.internal".to_string(),
                "http://localhost:3000".to_string(),
            ],
            new_origin: String::new(),
            restarting: false,
            restart_msg: String::new(),
            helper_status: HelperStatus::Running,
            log_level: LogLevel::Info,
            search: String::new(),
            time_range: TimeRange::Last15m,
            sev_filter: SevFilter::all_off(),
            live_tail: false,
            logs: seed_logs(),
            expanded_log: None,
            next_log_id: 15,
            remote_enabled: false,
            remote_url: String::new(),
            remote_auth: String::new(),
            remote_sending: false,
            remote_status: String::new(),
            toast: None,
        }
    }
}

fn seed_logs() -> Vec<LogEntry> {
    vec![
        LogEntry { id: 1, timestamp: "10:41:02.001".to_string(), severity: Severity::Info, source: "server".to_string(), message: "HTTP server listening on 127.0.0.1:9100".to_string() },
        LogEntry { id: 2, timestamp: "10:41:02.045".to_string(), severity: Severity::Debug, source: "auth".to_string(), message: "Token validation middleware registered".to_string() },
        LogEntry { id: 3, timestamp: "10:41:02.103".to_string(), severity: Severity::Info, source: "printer".to_string(), message: "USB device scan started".to_string() },
        LogEntry { id: 4, timestamp: "10:41:02.211".to_string(), severity: Severity::Info, source: "printer".to_string(), message: "Found: Zebra ZD421 (USB\\VID_0A5F&PID_00D8)".to_string() },
        LogEntry { id: 5, timestamp: "10:41:02.312".to_string(), severity: Severity::Debug, source: "printer".to_string(), message: "DPI auto-detected: 203".to_string() },
        LogEntry { id: 6, timestamp: "10:41:03.001".to_string(), severity: Severity::Info, source: "config".to_string(), message: "Config schema v7 loaded from disk".to_string() },
        LogEntry { id: 7, timestamp: "10:41:05.222".to_string(), severity: Severity::Info, source: "server".to_string(), message: "POST /print/label — 200 OK (42 ms)".to_string() },
        LogEntry { id: 8, timestamp: "10:41:07.819".to_string(), severity: Severity::Warn, source: "printer".to_string(), message: "Media out sensor triggered; pausing".to_string() },
        LogEntry { id: 9, timestamp: "10:41:08.001".to_string(), severity: Severity::Info, source: "printer".to_string(), message: "Media out cleared; resuming queue".to_string() },
        LogEntry { id: 10, timestamp: "10:41:10.555".to_string(), severity: Severity::Error, source: "auth".to_string(), message: "Invalid token in request from 192.168.1.44".to_string() },
        LogEntry { id: 11, timestamp: "10:41:11.001".to_string(), severity: Severity::Debug, source: "server".to_string(), message: "Request throttled: 429 Too Many Requests".to_string() },
        LogEntry { id: 12, timestamp: "10:41:12.334".to_string(), severity: Severity::Trace, source: "image".to_string(), message: "Dithering pass complete — 1872 bytes".to_string() },
        LogEntry { id: 13, timestamp: "10:41:13.001".to_string(), severity: Severity::Info, source: "server".to_string(), message: "GET /status — 200 OK (2 ms)".to_string() },
        LogEntry { id: 14, timestamp: "10:41:14.777".to_string(), severity: Severity::Warn, source: "config".to_string(), message: "Origin 'http://localhost:5173' not in allow-list".to_string() },
    ]
}

// ─── Messages ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Message {
    // Window management
    WindowOpened(window::Id),
    CursorMoved(Point),
    Resized(Size),
    LeftClick,
    Titlebar(TitlebarMessage),

    // Navigation
    SetSection(Section),

    // Printer
    ScanPrinters,
    ScanComplete(Vec<Printer>),
    SelectPrinter(String),
    SetDpiMode(DpiMode),
    SetDpiOverride(DpiOverride),
    TestPrint,
    TestComplete(bool, String),

    // Stocks
    OpenAddStock,
    OpenEditStock(u32),
    CloseModal,
    SaveStock,
    DeleteStock(u32),
    ModalName(String),
    ModalWidth(String),
    ModalHeight(String),
    ModalGap(String),
    ModalLinerLeft(String),
    ModalLinerRight(String),

    // Print settings
    SetDensity(u8),
    SetSpeed(PrintSpeed),
    SetOrientation(Orientation),
    SetScale(u8),
    ToggleAdvanced,
    SetMonoThreshold(u8),

    // Server
    SetPort(String),
    SetToken(String),
    ToggleTokenVisible,
    CopyToken,
    TokenCopied,
    GenerateToken,
    SetNewOrigin(String),
    AddOrigin,
    RemoveOrigin(usize),
    RestartHelper,
    RestartComplete(bool),

    // Logs
    SetLogLevel(LogLevel),
    SetSearch(String),
    SetTimeRange(TimeRange),
    ToggleSevTrace,
    ToggleSevDebug,
    ToggleSevInfo,
    ToggleSevWarn,
    ToggleSevError,
    ToggleLiveTail,
    PushLog(time::Instant),
    ClearLogs,
    ExportLogs,
    ExpandLog(u32),
    SetRemoteEnabled(bool),
    SetRemoteUrl(String),
    SetRemoteAuth(String),
    SendRemoteLogs,
    RemoteSendComplete(bool),

    // Toast
    DismissToast,
}

// ─── App implementation ───────────────────────────────────────────────────────

impl ConfigurationApp {
    pub const TITLE: &str = "Quill Configurator";
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    const BORDER: f32 = 8.0;

    pub fn update(state: &mut ConfigurationApp, message: Message) -> Task<Message> {
        match message {
            // ── Window management ──
            Message::WindowOpened(id) => {
                state.window_id = Some(id);
                Task::none()
            }
            Message::CursorMoved(pos) => {
                state.cursor = pos;
                state.resize_dir = Self::resize_direction(pos, state.window_size, Self::BORDER);
                Task::none()
            }
            Message::Resized(size) => {
                state.window_size = size;
                Task::none()
            }
            Message::LeftClick => {
                if let Some(id) = state.window_id {
                    return if let Some(dir) =
                        Self::resize_direction(state.cursor, state.window_size, Self::BORDER)
                    {
                        window::drag_resize(id, dir)
                    } else {
                        Task::none()
                    };
                }
                Task::none()
            }
            Message::Titlebar(msg) => titlebar::update(msg, state),

            // ── Navigation ──
            Message::SetSection(s) => {
                state.section = s;
                Task::none()
            }

            // ── Printer ──
            Message::ScanPrinters => {
                state.scanning = true;
                state.printers.clear();
                state.selected_printer = None;
                Task::perform(
                    async {
                        tokio::time::sleep(Duration::from_millis(1100)).await;
                        vec![
                            Printer {
                                name: "Zebra ZD421".to_string(),
                                meta: "USB\\VID_0A5F&PID_00D8".to_string(),
                                online: true,
                            },
                            Printer {
                                name: "Zebra ZD230".to_string(),
                                meta: "USB\\VID_0A5F&PID_00B3".to_string(),
                                online: true,
                            },
                            Printer {
                                name: "Zebra LP2824".to_string(),
                                meta: "USB\\VID_0A5F&PID_0027".to_string(),
                                online: false,
                            },
                        ]
                    },
                    Message::ScanComplete,
                )
            }
            Message::ScanComplete(printers) => {
                state.scanning = false;
                state.printers = printers;
                if !state.printers.is_empty() {
                    state.selected_printer = Some(state.printers[0].name.clone());
                }
                Task::none()
            }
            Message::SelectPrinter(name) => {
                state.selected_printer = Some(name);
                Task::none()
            }
            Message::SetDpiMode(mode) => {
                state.dpi_mode = mode;
                Task::none()
            }
            Message::SetDpiOverride(dpi) => {
                state.dpi_override = dpi;
                Task::none()
            }
            Message::TestPrint => {
                state.testing = true;
                let selected = state.selected_printer.clone().unwrap_or_default();
                Task::perform(
                    async move {
                        tokio::time::sleep(Duration::from_millis(1400)).await;
                        (true, format!("Test label sent to {}", selected))
                    },
                    |(ok, msg)| Message::TestComplete(ok, msg),
                )
            }
            Message::TestComplete(ok, msg) => {
                state.testing = false;
                state.test_ok = ok;
                state.test_msg = msg;
                state.last_tested = Some("Just now".to_string());
                Task::none()
            }

            // ── Stocks ──
            Message::OpenAddStock => {
                state.modal = Some(StockModal::new());
                Task::none()
            }
            Message::OpenEditStock(id) => {
                if let Some(stock) = state.stocks.iter().find(|s| s.id == id) {
                    state.modal = Some(StockModal::from_stock(stock));
                }
                Task::none()
            }
            Message::CloseModal => {
                state.modal = None;
                Task::none()
            }
            Message::SaveStock => {
                if let Some(modal) = &state.modal {
                    let width = modal.width.parse::<f32>().unwrap_or(0.0);
                    let height = modal.height.parse::<f32>().unwrap_or(0.0);
                    let gap = modal.gap.parse::<f32>().unwrap_or(0.0);
                    let liner_left = modal.liner_left.parse::<f32>().unwrap_or(0.0);
                    let liner_right = modal.liner_right.parse::<f32>().unwrap_or(0.0);
                    let name = modal.name.clone();
                    if let Some(edit_id) = modal.editing_id {
                        if let Some(s) = state.stocks.iter_mut().find(|s| s.id == edit_id) {
                            s.name = name;
                            s.width_mm = width;
                            s.height_mm = height;
                            s.gap_mm = gap;
                            s.liner_left_mm = liner_left;
                            s.liner_right_mm = liner_right;
                        }
                    } else {
                        let id = state.next_stock_id;
                        state.next_stock_id += 1;
                        state.stocks.push(Stock {
                            id,
                            name,
                            width_mm: width,
                            height_mm: height,
                            gap_mm: gap,
                            liner_left_mm: liner_left,
                            liner_right_mm: liner_right,
                        });
                    }
                }
                state.modal = None;
                Task::none()
            }
            Message::DeleteStock(id) => {
                state.stocks.retain(|s| s.id != id);
                Task::none()
            }
            Message::ModalName(v) => { if let Some(m) = &mut state.modal { m.name = v; } Task::none() }
            Message::ModalWidth(v) => { if let Some(m) = &mut state.modal { m.width = v; } Task::none() }
            Message::ModalHeight(v) => { if let Some(m) = &mut state.modal { m.height = v; } Task::none() }
            Message::ModalGap(v) => { if let Some(m) = &mut state.modal { m.gap = v; } Task::none() }
            Message::ModalLinerLeft(v) => { if let Some(m) = &mut state.modal { m.liner_left = v; } Task::none() }
            Message::ModalLinerRight(v) => { if let Some(m) = &mut state.modal { m.liner_right = v; } Task::none() }

            // ── Print settings ──
            Message::SetDensity(v) => { state.density = v; Task::none() }
            Message::SetSpeed(v) => { state.speed = v; Task::none() }
            Message::SetOrientation(v) => { state.orientation = v; Task::none() }
            Message::SetScale(v) => { state.scale = v; Task::none() }
            Message::ToggleAdvanced => { state.advanced_open = !state.advanced_open; Task::none() }
            Message::SetMonoThreshold(v) => { state.mono_threshold = v; Task::none() }

            // ── Server ──
            Message::SetPort(v) => { state.port = v; Task::none() }
            Message::SetToken(v) => { state.token = v; Task::none() }
            Message::ToggleTokenVisible => { state.token_visible = !state.token_visible; Task::none() }
            Message::CopyToken => {
                state.token_copied = true;
                Task::perform(
                    async { tokio::time::sleep(Duration::from_millis(1600)).await; },
                    |_| Message::TokenCopied,
                )
            }
            Message::TokenCopied => { state.token_copied = false; Task::none() }
            Message::GenerateToken => {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut h = DefaultHasher::new();
                std::time::SystemTime::now().hash(&mut h);
                let n = h.finish();
                state.token = format!("qk_live_{:016x}", n);
                Task::none()
            }
            Message::SetNewOrigin(v) => { state.new_origin = v; Task::none() }
            Message::AddOrigin => {
                let o = state.new_origin.trim().to_string();
                if !o.is_empty() && !state.origins.contains(&o) {
                    state.origins.push(o);
                    state.new_origin.clear();
                }
                Task::none()
            }
            Message::RemoveOrigin(i) => {
                if i < state.origins.len() { state.origins.remove(i); }
                Task::none()
            }
            Message::RestartHelper => {
                state.restarting = true;
                state.helper_status = HelperStatus::Restarting;
                state.restart_msg = "Restarting…".to_string();
                Task::perform(
                    async { tokio::time::sleep(Duration::from_millis(1600)).await; true },
                    Message::RestartComplete,
                )
            }
            Message::RestartComplete(ok) => {
                state.restarting = false;
                if ok {
                    state.helper_status = HelperStatus::Running;
                    state.restart_msg = "Helper restarted successfully.".to_string();
                } else {
                    state.helper_status = HelperStatus::Stopped;
                    state.restart_msg = "Restart failed.".to_string();
                }
                Task::none()
            }

            // ── Logs ──
            Message::SetLogLevel(v) => { state.log_level = v; Task::none() }
            Message::SetSearch(v) => { state.search = v; Task::none() }
            Message::SetTimeRange(v) => { state.time_range = v; Task::none() }
            Message::ToggleSevTrace => { state.sev_filter.trace = !state.sev_filter.trace; Task::none() }
            Message::ToggleSevDebug => { state.sev_filter.debug = !state.sev_filter.debug; Task::none() }
            Message::ToggleSevInfo => { state.sev_filter.info = !state.sev_filter.info; Task::none() }
            Message::ToggleSevWarn => { state.sev_filter.warn = !state.sev_filter.warn; Task::none() }
            Message::ToggleSevError => { state.sev_filter.error = !state.sev_filter.error; Task::none() }
            Message::ToggleLiveTail => { state.live_tail = !state.live_tail; Task::none() }
            Message::PushLog(_now) => {
                let sev = match state.next_log_id % 5 {
                    0 => Severity::Error,
                    1 => Severity::Warn,
                    2 => Severity::Debug,
                    3 => Severity::Trace,
                    _ => Severity::Info,
                };
                let messages = [
                    "POST /print/label — 200 OK (38 ms)",
                    "GET /status — 200 OK (1 ms)",
                    "USB heartbeat acknowledged",
                    "Queue depth: 0",
                    "Media ready",
                ];
                let msg = messages[(state.next_log_id as usize) % messages.len()];
                let sources = ["server", "printer", "auth", "config", "image"];
                let src = sources[(state.next_log_id as usize) % sources.len()];
                state.logs.push(LogEntry {
                    id: state.next_log_id,
                    timestamp: "live".to_string(),
                    severity: sev,
                    source: src.to_string(),
                    message: msg.to_string(),
                });
                state.next_log_id += 1;
                if state.logs.len() > 200 {
                    state.logs.drain(0..state.logs.len() - 200);
                }
                Task::none()
            }
            Message::ClearLogs => { state.logs.clear(); Task::none() }
            Message::ExportLogs => {
                state.toast = Some("Logs exported to quill_logs.txt".to_string());
                Task::none()
            }
            Message::ExpandLog(id) => {
                state.expanded_log = if state.expanded_log == Some(id) { None } else { Some(id) };
                Task::none()
            }
            Message::SetRemoteEnabled(v) => { state.remote_enabled = v; Task::none() }
            Message::SetRemoteUrl(v) => { state.remote_url = v; Task::none() }
            Message::SetRemoteAuth(v) => { state.remote_auth = v; Task::none() }
            Message::SendRemoteLogs => {
                state.remote_sending = true;
                state.remote_status = "Sending…".to_string();
                Task::perform(
                    async { tokio::time::sleep(Duration::from_millis(1200)).await; true },
                    Message::RemoteSendComplete,
                )
            }
            Message::RemoteSendComplete(ok) => {
                state.remote_sending = false;
                state.remote_status = if ok { "Sent successfully.".to_string() } else { "Send failed.".to_string() };
                Task::none()
            }

            // ── Toast ──
            Message::DismissToast => { state.toast = None; Task::none() }
        }
    }

    pub fn view(state: &'_ ConfigurationApp) -> Element<'_, Message> {
        let interaction = match state.resize_dir {
            Some(Direction::North | Direction::South) => mouse::Interaction::ResizingVertically,
            Some(Direction::East | Direction::West) => mouse::Interaction::ResizingHorizontally,
            Some(Direction::NorthEast | Direction::SouthWest) => mouse::Interaction::ResizingDiagonallyUp,
            Some(Direction::NorthWest | Direction::SouthEast) => mouse::Interaction::ResizingDiagonallyDown,
            None => mouse::Interaction::Idle,
        };

        let status_bar = status_bar_view(state);
        let sidebar = sidebar_view(state);
        let main = main_content_view(state);

        let body = row![sidebar, main].width(Length::Fill).height(Length::Fill);
        let root = column![status_bar, body].width(Length::Fill).height(Length::Fill);

        let root_container = container(root)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Background::Color(design::APP_BG)),
                ..container::Style::default()
            });

        let base: Element<'_, Message> = if state.modal.is_some() {
            use iced::widget::stack;
            stack![root_container, modal_overlay(state)].into()
        } else {
            root_container.into()
        };

        mouse_area(base)
            .interaction(interaction)
            .on_press(Message::LeftClick)
            .into()
    }

    pub fn subscription(state: &ConfigurationApp) -> Subscription<Message> {
        let mut subs = vec![
            window::open_events().map(Message::WindowOpened),
            event::listen_raw(|event, _status, _id| match event {
                Event::Mouse(mouse::Event::CursorMoved { position }) => Some(Message::CursorMoved(position)),
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => Some(Message::LeftClick),
                Event::Window(window::Event::Resized(size)) => Some(Message::Resized(size)),
                _ => None,
            }),
        ];

        if state.live_tail {
            subs.push(time::every(Duration::from_millis(1500)).map(Message::PushLog));
        }

        Subscription::batch(subs)
    }

    fn resize_direction(cursor: Point, size: Size, border: f32) -> Option<Direction> {
        let left = cursor.x <= border;
        let right = cursor.x >= size.width - border;
        let top = cursor.y <= border;
        let bottom = cursor.y >= size.height - border;
        match (top, bottom, left, right) {
            (true, _, true, _) => Some(Direction::NorthWest),
            (true, _, _, true) => Some(Direction::NorthEast),
            (_, true, true, _) => Some(Direction::SouthWest),
            (_, true, _, true) => Some(Direction::SouthEast),
            (true, _, _, _) => Some(Direction::North),
            (_, true, _, _) => Some(Direction::South),
            (_, _, true, _) => Some(Direction::West),
            (_, _, _, true) => Some(Direction::East),
            _ => None,
        }
    }

    pub fn window_settings() -> window::Settings {
        window::Settings {
            decorations: false,
            resizable: true,
            platform_specific: PlatformSpecific {
                corner_preference: CornerPreference::Default,
                undecorated_shadow: true,
                drag_and_drop: true,
                skip_taskbar: false,
            },
            ..window::Settings::default()
        }
    }
}

// ─── Status bar ──────────────────────────────────────────────────────────────

fn status_bar_view<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    use crate::theme::{Icon, icon};
    use crate::widgets::components::button::{ButtonRadius, button};
    use iced::font::Weight;

    let brand = container(
        row![
            icon(Icon::lucide().printer(), 16, Some((design::ACCENT, design::ACCENT))),
            Space::new().width(8),
            text("QUILL")
                .size(13)
                .color(design::FG)
                .font(iced::Font {
                    weight: Weight::Bold,
                    ..crate::theme::layout::fonts::INTER
                }),
        ]
        .align_y(iced::Alignment::Center),
    )
    .width(Length::Fixed(228.0))
    .height(Length::Fill)
    .padding(Padding::from([0.0_f32, 16.0]));

    let helper_pill = status_pill(
        match state.helper_status {
            HelperStatus::Running => "Helper · Running",
            HelperStatus::Stopped => "Helper · Stopped",
            HelperStatus::Restarting => "Helper · Restarting",
        },
        match state.helper_status {
            HelperStatus::Running => PillKind::Success,
            HelperStatus::Stopped => PillKind::Danger,
            HelperStatus::Restarting => PillKind::Warn,
        },
    );

    let printer_pill = if let Some(name) = &state.selected_printer {
        let online = state.printers.iter().find(|p| &p.name == name).map(|p| p.online).unwrap_or(false);
        status_pill(
            if online { "Printer · Online" } else { "Printer · Offline" },
            if online { PillKind::Success } else { PillKind::Danger },
        )
    } else {
        status_pill("No Printer", PillKind::Muted)
    };

    let active_name_pill: Element<'_, Message> = if let Some(name) = &state.selected_printer {
        container(text(name.as_str()).size(11).color(design::FG_MUTED))
            .padding(Padding::from([3.0_f32, 8.0]))
            .style(|_| container::Style {
                background: Some(Background::Color(design::SURFACE)),
                border: Border { color: design::BORDER, width: 1.0, radius: 4.0.into() },
                ..container::Style::default()
            })
            .into()
    } else {
        Space::new().into()
    };

    let center = row![
        helper_pill,
        Space::new().width(6),
        printer_pill,
        Space::new().width(6),
        active_name_pill,
    ]
    .align_y(iced::Alignment::Center);

    let titlebar_controls = row![
        button(icon(Icon::material_symbols().minimize_rounded(), 16, None))
            .on_press(Message::Titlebar(TitlebarMessage::Minimize))
            .ghost()
            .radius(ButtonRadius::None)
            .icon_only(),
        button(icon(Icon::material_symbols().square_outline_rounded(), 16, None))
            .on_press(Message::Titlebar(TitlebarMessage::ToggleMaximize))
            .ghost()
            .radius(ButtonRadius::None)
            .icon_only(),
        button(icon(Icon::material_symbols().close_rounded(), 16, None))
            .on_press(Message::Titlebar(TitlebarMessage::Close))
            .danger_soft()
            .radius(ButtonRadius::None)
            .icon_only(),
    ]
    .align_y(iced::Alignment::Center);

    let status_row = mouse_area(
        row![
            brand,
            Space::new().width(Length::Fill),
            center,
            Space::new().width(Length::Fill),
            titlebar_controls,
        ]
        .align_y(iced::Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fixed(58.0)),
    )
    .on_press(Message::Titlebar(TitlebarMessage::DragStart));

    container(status_row)
        .width(Length::Fill)
        .height(Length::Fixed(58.0))
        .style(|_| container::Style {
            background: Some(Background::Color(design::TOPBAR)),
            border: Border { color: design::BORDER, width: 0.0, radius: 0.0.into() },
            ..container::Style::default()
        })
        .into()
}

// ─── Sidebar ─────────────────────────────────────────────────────────────────

fn sidebar_view<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    use crate::theme::{Icon, icon};

    let nav_items: &[(&str, Section, u8)] = &[
        ("Printer Setup", Section::Printer, 0),
        ("Label Stocks", Section::Stocks, 1),
        ("Print Settings", Section::PrintSettings, 2),
        ("Server & Security", Section::Server, 3),
        ("Logs & Diagnostics", Section::Logs, 4),
        ("About", Section::About, 5),
    ];

    let mut nav_col = iced::widget::Column::new().spacing(2).padding(Padding::from([12.0_f32, 8.0]));

    for (label, section, icon_idx) in nav_items {
        let active = state.section == *section;
        let ic = if active {
            Some((design::ACCENT, design::ACCENT))
        } else {
            Some((design::FG_MUTED, design::FG_MUTED))
        };

        let icon_el: Element<'_, Message> = match icon_idx {
            0 => icon(Icon::lucide().printer(), 16, ic),
            1 => icon(Icon::lucide().tag(), 16, ic),
            2 => icon(Icon::lucide().sliders_horizontal(), 16, ic),
            3 => icon(Icon::lucide().shield(), 16, ic),
            4 => icon(Icon::lucide().list(), 16, ic),
            5 => icon(Icon::lucide().info(), 16, ic),
            _ => icon(Icon::lucide().circle(), 16, ic),
        };

        let nav_btn = iced::widget::button(
            row![
                icon_el,
                Space::new().width(10),
                text(*label).size(13).color(if active { design::FG } else { design::FG_MUTED }),
            ]
            .align_y(iced::Alignment::Center),
        )
        .padding(Padding::from([8.0_f32, 12.0]))
        .width(Length::Fill)
        .on_press(Message::SetSection(*section))
        .style(move |_theme, status| {
            let hovered = matches!(status, iced::widget::button::Status::Hovered);
            let bg = if active {
                Some(Background::Color(design::ACCENT_SOFT))
            } else if hovered {
                Some(Background::Color(design::HOVER))
            } else {
                None
            };
            iced::widget::button::Style {
                background: bg,
                border: Border {
                    color: if active { design::ACCENT } else { Color::TRANSPARENT },
                    width: if active { 1.0 } else { 0.0 },
                    radius: 6.0.into(),
                },
                text_color: if active { design::FG } else { design::FG_MUTED },
                ..iced::widget::button::Style::default()
            }
        });

        nav_col = nav_col.push(nav_btn);
    }

    let footer = container(
        text("Config schema v7 · 127.0.0.1").size(10).color(design::FG_SUBTLE),
    )
    .padding(Padding::from([12.0_f32, 16.0]));

    let sidebar_col = column![
        nav_col,
        Space::new().height(Length::Fill),
        footer,
    ]
    .width(Length::Fixed(228.0))
    .height(Length::Fill);

    container(sidebar_col)
        .width(Length::Fixed(228.0))
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(design::SIDEBAR)),
            border: Border { color: design::BORDER_STRONG, width: 0.0, radius: 0.0.into() },
            ..container::Style::default()
        })
        .into()
}

// ─── Main content ─────────────────────────────────────────────────────────────

fn main_content_view<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let section_content: Element<'_, Message> = match state.section {
        Section::Printer => sections::printer::printer_view(state),
        Section::Stocks => sections::stocks::stocks_view(state),
        Section::PrintSettings => sections::print_settings::print_settings_view(state),
        Section::Server => sections::server::server_view(state),
        Section::Logs => sections::logs::logs_view(state),
        Section::About => sections::about::about_view(state),
    };

    let inner = container(section_content)
        .max_width(840)
        .padding(Padding::from([36.0_f32, 40.0]));

    let centered = container(
        row![
            Space::new().width(Length::Fill),
            inner,
            Space::new().width(Length::Fill),
        ]
    )
    .width(Length::Fill);

    scrollable(centered)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

// ─── Modal overlay ────────────────────────────────────────────────────────────

fn modal_overlay<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let Some(modal) = &state.modal else {
        return Space::new().into();
    };

    let title = if modal.editing_id.is_some() { "Edit Stock" } else { "Add Stock" };

    let mk_field = |label: &'static str, val: &str, msg: fn(String) -> Message| -> Element<'_, Message> {
        column![
            text(label).size(11).color(design::FG_MUTED),
            Space::new().height(4),
            iced::widget::text_input("", val)
                .on_input(msg)
                .padding(Padding::from([8.0_f32, 12.0]))
                .size(13)
                .style(|_theme, _status| iced::widget::text_input::Style {
                    background: Background::Color(design::INPUT_BG),
                    border: Border { color: design::INPUT_BORDER, width: 1.0, radius: 6.0.into() },
                    icon: design::FG_MUTED,
                    placeholder: design::FG_SUBTLE,
                    value: design::FG,
                    selection: design::ACCENT_SOFT,
                }),
        ]
        .spacing(0)
        .into()
    };

    let form = column![
        text(title).size(16).color(design::FG).font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..crate::theme::layout::fonts::INTER
        }),
        Space::new().height(16),
        mk_field("Name", &modal.name, Message::ModalName),
        Space::new().height(10),
        row![
            mk_field("Width (mm)", &modal.width, Message::ModalWidth),
            Space::new().width(10),
            mk_field("Height (mm)", &modal.height, Message::ModalHeight),
        ],
        Space::new().height(10),
        mk_field("Gap (mm)", &modal.gap, Message::ModalGap),
        Space::new().height(10),
        row![
            mk_field("Liner Left (mm)", &modal.liner_left, Message::ModalLinerLeft),
            Space::new().width(10),
            mk_field("Liner Right (mm)", &modal.liner_right, Message::ModalLinerRight),
        ],
        Space::new().height(20),
        row![
            iced::widget::button(text("Cancel").size(13).color(design::FG_MUTED))
                .padding(Padding::from([8.0_f32, 16.0]))
                .on_press(Message::CloseModal)
                .style(|_theme, status| {
                    let hov = matches!(status, iced::widget::button::Status::Hovered);
                    iced::widget::button::Style {
                        background: Some(Background::Color(if hov { design::SURFACE2 } else { design::SURFACE })),
                        border: Border { color: design::BORDER_STRONG, width: 1.0, radius: 6.0.into() },
                        text_color: design::FG_MUTED,
                        ..iced::widget::button::Style::default()
                    }
                }),
            Space::new().width(Length::Fill),
            iced::widget::button(text("Save").size(13).color(Color::WHITE))
                .padding(Padding::from([8.0_f32, 16.0]))
                .on_press(Message::SaveStock)
                .style(|_theme, status| {
                    let hov = matches!(status, iced::widget::button::Status::Hovered);
                    iced::widget::button::Style {
                        background: Some(Background::Color(if hov { design::ACCENT_HOVER } else { design::ACCENT })),
                        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 6.0.into() },
                        text_color: Color::WHITE,
                        ..iced::widget::button::Style::default()
                    }
                }),
        ]
        .align_y(iced::Alignment::Center),
    ]
    .spacing(0);

    let modal_card = container(form)
        .width(Length::Fixed(480.0))
        .padding(Padding::from([28.0_f32, 28.0]))
        .style(|_| container::Style {
            background: Some(Background::Color(design::SURFACE)),
            border: Border { color: design::BORDER_STRONG, width: 1.0, radius: 12.0.into() },
            ..container::Style::default()
        });

    let backdrop = mouse_area(
        container(
            container(modal_card).center_x(Length::Fill).center_y(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color { a: 0.6, ..Color::BLACK })),
            ..container::Style::default()
        }),
    )
    .on_press(Message::CloseModal);

    backdrop.into()
}

// ─── Pill helpers ─────────────────────────────────────────────────────────────

enum PillKind {
    Success,
    Danger,
    Warn,
    Muted,
}

fn status_pill<'a>(label: &'a str, kind: PillKind) -> Element<'a, Message> {
    let (fg, bg) = match kind {
        PillKind::Success => (design::SUCCESS_FG, design::SUCCESS_BG),
        PillKind::Danger => (design::DANGER_FG, design::DANGER_BG),
        PillKind::Warn => (design::WARN_FG, design::WARN_BG),
        PillKind::Muted => (design::FG_SUBTLE, design::SURFACE),
    };
    container(text(label).size(11).color(fg))
        .padding(Padding::from([3.0_f32, 8.0]))
        .style(move |_| container::Style {
            background: Some(Background::Color(bg)),
            border: Border { color: fg, width: 0.0, radius: 99.0.into() },
            ..container::Style::default()
        })
        .into()
}
