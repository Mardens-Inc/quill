use crate::configuration_app::{ConfigurationApp, Message};
use crate::theme::layout::fonts::JETBRAINS_MONO;
use crate::theme::{Icon, icon};
use crate::widgets::components::button::{ButtonRadius, button};
use iced::font::Weight;
use iced::widget::{mouse_area, row, text};
use iced::{Element, Font, Length, Task, window};

#[derive(Debug, Clone)]
pub enum TitlebarMessage {
    Minimize,
    ToggleMaximize,
    Close,
    DragStart,
}

pub fn titlebar() -> Element<'static, TitlebarMessage> {
    row![
        mouse_area(
            text(format!(
                "{} - {}",
                ConfigurationApp::TITLE,
                ConfigurationApp::VERSION
            ))
            .font(Font {
                weight: Weight::Normal,
                ..JETBRAINS_MONO
            })
        )
        .on_press(TitlebarMessage::DragStart),
        button(icon(Icon::material_symbols().minimize_rounded(), 18, None))
            .on_press(TitlebarMessage::Minimize)
            .ghost()
            .radius(ButtonRadius::None)
            .icon_only(),
        button(icon(
            Icon::material_symbols().square_outline_rounded(),
            18,
            None
        ))
        .on_press(TitlebarMessage::ToggleMaximize)
        .ghost()
        .radius(ButtonRadius::None)
        .icon_only(),
        button(icon(Icon::material_symbols().close_rounded(), 18, None))
            .on_press(TitlebarMessage::Close)
            .danger_soft()
            .radius(ButtonRadius::None)
            .icon_only(),
    ]
    .width(Length::Fill)
    .into()
}

pub fn update(msg: TitlebarMessage, state: &mut ConfigurationApp) -> Task<Message> {
    match msg {
        TitlebarMessage::Close => {
            if let Some(id) = state.window_id {
                window::close(id)
            } else {
                Task::none()
            }
        }
        TitlebarMessage::ToggleMaximize => {
            if let Some(id) = state.window_id {
                window::toggle_maximize(id)
            } else {
                Task::none()
            }
        }
        TitlebarMessage::Minimize => {
            if let Some(id) = state.window_id {
                window::minimize(id, true)
            } else {
                Task::none()
            }
        }
        TitlebarMessage::DragStart => {
            if let Some(id) = state.window_id {
                window::drag(id)
            } else {
                Task::none()
            }
        }
    }
}
