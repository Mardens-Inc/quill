use iced::{
    Element, Point, Size, Subscription, Task,
    widget::column,
    window::{self, Direction, settings::PlatformSpecific, settings::platform::CornerPreference},
};

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
pub enum HelperStatus {
    Running,
    #[default]
    Stopped,
    Restarting,
}

pub struct ConfigurationApp {
    pub window_id: Option<window::Id>,
    pub window_size: Size,
    pub cursor: Point,
    pub resize_dir: Option<Direction>,
}

impl Default for ConfigurationApp {
    fn default() -> Self {
        ConfigurationApp {
            window_id: None,
            window_size: Size::new(1200.0, 800.0),
            cursor: Point::ORIGIN,
            resize_dir: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {}

impl ConfigurationApp {
    pub const TITLE: &str = "Quill Configurator";
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    const BORDER: f32 = 8.0;

    pub fn update(state: &mut ConfigurationApp, message: Message) -> Task<Message> {
        match message {}
    }

    pub fn view(state: &'_ ConfigurationApp) -> Element<'_, Message> {
        column![].into()
    }

    pub fn subscription(state: &ConfigurationApp) -> Subscription<Message> {
        Subscription::batch(vec![])
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
