use crate::configuration_app::{ConfigurationApp, HelperStatus, Message};
use crate::theme::design;
use iced::font::Weight;
use iced::widget::{Space, column, container, row, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding};

pub fn server_view<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    column![
        text("Server & Security").size(22).color(design::FG).font(iced::Font {
            weight: Weight::Semibold,
            ..crate::theme::layout::fonts::INTER
        }),
        Space::new().height(4),
        text("Configure listening address, authentication, and CORS origins")
            .size(13).color(design::FG_MUTED),
        Space::new().height(24),
        port_card(state),
        Space::new().height(16),
        token_card(state),
        Space::new().height(16),
        origins_card(state),
        Space::new().height(16),
        restart_card(state),
    ]
    .spacing(0)
    .width(Length::Fill)
    .into()
}

fn port_card<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let port_valid = state.port.parse::<u16>().is_ok();

    let port_input_row = row![
        container(text("127.0.0.1:").size(13).color(design::FG_MUTED))
            .padding(Padding::from([8.0_f32, 12.0]))
            .style(|_| container::Style {
                background: Some(Background::Color(design::SURFACE2)),
                border: Border {
                    color: design::BORDER,
                    width: 1.0,
                    radius: iced::border::Radius {
                        top_left: 6.0,
                        top_right: 0.0,
                        bottom_right: 0.0,
                        bottom_left: 6.0,
                    },
                },
                ..container::Style::default()
            }),
        text_input("9100", &state.port)
            .on_input(Message::SetPort)
            .padding(Padding::from([8.0_f32, 12.0]))
            .size(13)
            .width(Length::Fixed(120.0))
            .style(move |_theme, _status| iced::widget::text_input::Style {
                background: Background::Color(design::INPUT_BG),
                border: Border {
                    color: if port_valid { design::INPUT_BORDER } else { design::DANGER_FG },
                    width: 1.0,
                    radius: iced::border::Radius {
                        top_left: 0.0,
                        top_right: 6.0,
                        bottom_right: 6.0,
                        bottom_left: 0.0,
                    },
                },
                icon: design::FG_MUTED,
                placeholder: design::FG_SUBTLE,
                value: design::FG,
                selection: design::ACCENT_SOFT,
            }),
    ]
    .align_y(Alignment::Center);

    let port_err: Element<'_, Message> = if !port_valid {
        column![
            Space::new().height(6),
            text("Invalid port number. Must be 1–65535.").size(11).color(design::DANGER_FG),
        ]
        .spacing(0)
        .into()
    } else {
        Space::new().into()
    };

    card(column![
        text("Listening Port").size(14).color(design::FG).font(iced::Font {
            weight: Weight::Semibold,
            ..crate::theme::layout::fonts::INTER
        }),
        Space::new().height(4),
        text("The port the Quill helper service listens on.").size(12).color(design::FG_MUTED),
        Space::new().height(12),
        port_input_row,
        port_err,
    ]
    .spacing(0))
}

fn token_card<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let masked: String = if state.token_visible {
        state.token.clone()
    } else {
        let prefix = "qk_live_";
        let rest_len = state.token.len().saturating_sub(prefix.len());
        format!("{}{}", prefix, "•".repeat(rest_len))
    };

    let copy_label = if state.token_copied { "Copied!" } else { "Copy" };
    let token_copied = state.token_copied;
    let token_visible = state.token_visible;

    card(column![
        text("Auth Token").size(14).color(design::FG).font(iced::Font {
            weight: Weight::Semibold,
            ..crate::theme::layout::fonts::INTER
        }),
        Space::new().height(4),
        text("Include this token in the Authorization header for all API requests.")
            .size(12).color(design::FG_MUTED),
        Space::new().height(12),
        row![
            container(
                text(masked.clone())
                    .size(13)
                    .color(design::FG)
                    .font(crate::theme::layout::fonts::JETBRAINS_MONO),
            )
            .padding(Padding::from([8.0_f32, 12.0]))
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(design::INPUT_BG)),
                border: Border {
                    color: design::INPUT_BORDER,
                    width: 1.0,
                    radius: iced::border::Radius {
                        top_left: 6.0,
                        top_right: 0.0,
                        bottom_right: 0.0,
                        bottom_left: 6.0,
                    },
                },
                ..container::Style::default()
            }),
            text_btn(if token_visible { "Hide" } else { "Show" }, Message::ToggleTokenVisible, false),
            text_btn(copy_label, Message::CopyToken, token_copied),
            text_btn("Generate", Message::GenerateToken, false),
        ]
        .align_y(Alignment::Center),
    ]
    .spacing(0))
}

fn text_btn<'a>(label: &'a str, msg: Message, accent: bool) -> Element<'a, Message> {
    iced::widget::button(
        text(label).size(12).color(if accent { design::ACCENT } else { design::FG_MUTED }),
    )
    .padding(Padding::from([8.0_f32, 12.0]))
    .on_press(msg)
    .style(move |_theme, status| {
        let hov = matches!(status, iced::widget::button::Status::Hovered);
        iced::widget::button::Style {
            background: Some(Background::Color(if hov { design::HOVER } else { design::SURFACE2 })),
            border: Border { color: design::BORDER, width: 1.0, radius: 0.0.into() },
            text_color: if accent { design::ACCENT } else { design::FG_MUTED },
            ..iced::widget::button::Style::default()
        }
    })
    .into()
}

fn origins_card<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let mut origins_list = column![].spacing(6);

    for (i, origin) in state.origins.iter().enumerate() {
        let idx = i;
        let origin_row = row![
            container(
                text(origin.as_str())
                    .size(12)
                    .color(design::FG)
                    .font(crate::theme::layout::fonts::JETBRAINS_MONO),
            )
            .padding(Padding::from([7.0_f32, 12.0]))
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(design::SURFACE2)),
                border: Border { color: design::BORDER, width: 1.0, radius: 6.0.into() },
                ..container::Style::default()
            }),
            Space::new().width(8),
            iced::widget::button(text("Remove").size(11).color(design::DANGER_FG))
                .padding(Padding::from([6.0_f32, 10.0]))
                .on_press(Message::RemoveOrigin(idx))
                .style(|_theme, status| {
                    let hov = matches!(status, iced::widget::button::Status::Hovered);
                    iced::widget::button::Style {
                        background: Some(Background::Color(if hov { design::DANGER_BG } else { Color::TRANSPARENT })),
                        border: Border { color: design::DANGER_FG, width: 1.0, radius: 4.0.into() },
                        text_color: design::DANGER_FG,
                        ..iced::widget::button::Style::default()
                    }
                }),
        ]
        .align_y(Alignment::Center);
        origins_list = origins_list.push(origin_row);
    }

    let add_row = row![
        text_input("https://example.com", &state.new_origin)
            .on_input(Message::SetNewOrigin)
            .padding(Padding::from([8.0_f32, 12.0]))
            .size(12)
            .width(Length::Fill)
            .style(|_theme, _status| iced::widget::text_input::Style {
                background: Background::Color(design::INPUT_BG),
                border: Border { color: design::INPUT_BORDER, width: 1.0, radius: 6.0.into() },
                icon: design::FG_MUTED,
                placeholder: design::FG_SUBTLE,
                value: design::FG,
                selection: design::ACCENT_SOFT,
            }),
        Space::new().width(8),
        iced::widget::button(text("Add").size(12).color(Color::WHITE))
            .padding(Padding::from([8.0_f32, 14.0]))
            .on_press(Message::AddOrigin)
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
    .align_y(Alignment::Center);

    card(column![
        text("Allowed Origins (CORS)").size(14).color(design::FG).font(iced::Font {
            weight: Weight::Semibold,
            ..crate::theme::layout::fonts::INTER
        }),
        Space::new().height(4),
        text("Requests from origins not in this list will be rejected.")
            .size(12).color(design::FG_MUTED),
        Space::new().height(14),
        origins_list,
        Space::new().height(10),
        add_row,
    ]
    .spacing(0))
}

fn restart_card<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let restarting = state.restarting;

    let restart_row = row![
        column![
            text("Restart Helper Service").size(14).color(design::FG).font(iced::Font {
                weight: Weight::Semibold,
                ..crate::theme::layout::fonts::INTER
            }),
            Space::new().height(4),
            text("Restarts the background Quill helper process. Active print jobs may be interrupted.")
                .size(12).color(design::FG_MUTED),
        ],
        Space::new().width(Length::Fill),
        iced::widget::button(
            text(if restarting { "Restarting…" } else { "Restart" })
                .size(13).color(Color::BLACK),
        )
        .padding(Padding::from([8.0_f32, 16.0]))
        .on_press_maybe(if restarting { None } else { Some(Message::RestartHelper) })
        .style(move |_theme, status| {
            let hov = matches!(status, iced::widget::button::Status::Hovered);
            iced::widget::button::Style {
                background: Some(Background::Color(if restarting {
                    Color { a: 0.5, ..design::WARN_FG }
                } else if hov {
                    Color::from_rgb8(0xea, 0xab, 0x15)
                } else {
                    design::WARN_FG
                })),
                border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 6.0.into() },
                text_color: Color::BLACK,
                ..iced::widget::button::Style::default()
            }
        }),
    ]
    .align_y(Alignment::Center);

    let restart_status: Element<'_, Message> = if !state.restart_msg.is_empty() {
        column![
            Space::new().height(10),
            text(state.restart_msg.as_str()).size(12).color(match state.helper_status {
                HelperStatus::Running => design::SUCCESS_FG,
                HelperStatus::Stopped => design::DANGER_FG,
                HelperStatus::Restarting => design::WARN_FG,
            }),
        ]
        .spacing(0)
        .into()
    } else {
        Space::new().into()
    };
    card(column![restart_row, restart_status].spacing(0))
}

fn card<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .width(Length::Fill)
        .padding(Padding::from([20.0_f32, 20.0]))
        .style(|_| container::Style {
            background: Some(Background::Color(design::SURFACE)),
            border: Border { color: design::BORDER_STRONG, width: 1.0, radius: 10.0.into() },
            ..container::Style::default()
        })
        .into()
}
