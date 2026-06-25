use iced::{
    widget::column, window::{self, settings::platform::CornerPreference, settings::PlatformSpecific}, Element,
    Subscription,
    Task,
};
use crate::components::window_chrome::WindowChrome;

#[derive(Default)]
pub struct ConfigurationApp;


#[derive(Debug, Clone)]
pub enum Message {}

impl ConfigurationApp {
    pub const TITLE: &str = "Quill Configurator";
    pub const VERSION: &str = env!("CARGO_PKG_BUILD");

    pub fn update(state: &mut ConfigurationApp, message: Message) -> Task<Message> {
        match message {}
    }

    pub fn view(state: &'_ ConfigurationApp) -> Element<'_, Message> {
        column![].into()
    }

    pub fn subscription(state: &ConfigurationApp) -> Subscription<Message> {
        Subscription::batch(vec![])
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
