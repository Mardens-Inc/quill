#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::configuration_app::ConfigurationApp;
use crate::theme::FontFamily;

pub mod configuration_app;
pub mod widgets;
pub mod theme;

fn main() -> iced::Result {
    color_eyre::install().expect("Failed to install `color_eyre`");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_thread_names(true)
        .init();
    iced::application(
        ConfigurationApp::default,
        ConfigurationApp::update,
        ConfigurationApp::view,
    )
    .title(ConfigurationApp::TITLE)
    .window(ConfigurationApp::window_settings())
    .subscription(ConfigurationApp::subscription)
    .centered()
    .font(FontFamily::jetbrains_mono().regular())
    .font(FontFamily::jetbrains_mono().medium())
    .font(FontFamily::jetbrains_mono().semi_bold())
    .font(FontFamily::jetbrains_mono().bold())
    .font(FontFamily::inter().regular())
    .font(FontFamily::inter().medium())
    .font(FontFamily::inter().semi_bold())
    .font(FontFamily::inter().bold())
    .run()
}
