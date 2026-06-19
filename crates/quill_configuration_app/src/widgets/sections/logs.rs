use crate::configuration_app::{ConfigurationApp, LogEntry, Message, Severity, TIME_RANGES, LOG_LEVELS};
use crate::theme::design;
use iced::border::Radius;
use iced::font::Weight;
use iced::widget::{Space, column, container, pick_list, row, scrollable, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding};

pub fn logs_view<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let visible: Vec<&LogEntry> = state.logs.iter()
        .filter(|e| {
            let search_ok = state.search.is_empty()
                || e.message.to_lowercase().contains(&state.search.to_lowercase())
                || e.source.to_lowercase().contains(&state.search.to_lowercase());
            let sev_ok = state.sev_filter.matches(e.severity);
            search_ok && sev_ok
        })
        .collect();

    column![
        row![
            column![
                text("Logs & Diagnostics").size(22).color(design::FG).font(iced::Font {
                    weight: Weight::Semibold,
                    ..crate::theme::layout::fonts::INTER
                }),
                Space::new().height(4),
                text("Live log viewer with filters and remote submission")
                    .size(13).color(design::FG_MUTED),
            ]
            .spacing(0),
            Space::new().width(Length::Fill),
            pick_list(LOG_LEVELS, Some(state.log_level), Message::SetLogLevel)
                .text_size(12)
                .padding(Padding::from([6.0_f32, 10.0]))
                .style(|_theme, _status| iced::widget::pick_list::Style {
                    text_color: design::FG,
                    placeholder_color: design::FG_SUBTLE,
                    background: Background::Color(design::SURFACE),
                    border: Border { color: design::BORDER_STRONG, width: 1.0, radius: 6.0.into() },
                    handle_color: design::FG_MUTED,
                }),
        ]
        .align_y(Alignment::Center),
        Space::new().height(20),
        filter_bar(state),
        Space::new().height(12),
        action_bar(state, visible.len()),
        Space::new().height(12),
        log_table(state, &visible),
        Space::new().height(16),
        remote_card(state),
        Space::new().height(16),
        log_file_card(),
    ]
    .spacing(0)
    .width(Length::Fill)
    .into()
}

fn filter_bar<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let sev_chip = |label: &'static str, active: bool, msg: Message| -> Element<'_, Message> {
        iced::widget::button(
            text(label).size(11).color(if active { design::FG } else { design::FG_MUTED }),
        )
        .padding(Padding::from([4.0_f32, 10.0]))
        .on_press(msg)
        .style(move |_theme, status| {
            let hov = matches!(status, iced::widget::button::Status::Hovered);
            iced::widget::button::Style {
                background: Some(Background::Color(if active {
                    design::ACCENT_SOFT
                } else if hov {
                    design::HOVER
                } else {
                    design::SURFACE2
                })),
                border: Border {
                    color: if active { design::ACCENT } else { design::BORDER },
                    width: 1.0,
                    radius: 99.0.into(),
                },
                text_color: if active { design::FG } else { design::FG_MUTED },
                ..iced::widget::button::Style::default()
            }
        })
        .into()
    };

    row![
        text_input("Search logs…", &state.search)
            .on_input(Message::SetSearch)
            .padding(Padding::from([7.0_f32, 12.0]))
            .size(12)
            .width(Length::Fixed(220.0))
            .style(|_theme, _status| iced::widget::text_input::Style {
                background: Background::Color(design::INPUT_BG),
                border: Border { color: design::INPUT_BORDER, width: 1.0, radius: 6.0.into() },
                icon: design::FG_MUTED,
                placeholder: design::FG_SUBTLE,
                value: design::FG,
                selection: design::ACCENT_SOFT,
            }),
        Space::new().width(10),
        sev_chip("TRACE", state.sev_filter.trace, Message::ToggleSevTrace),
        Space::new().width(4),
        sev_chip("DEBUG", state.sev_filter.debug, Message::ToggleSevDebug),
        Space::new().width(4),
        sev_chip("INFO", state.sev_filter.info, Message::ToggleSevInfo),
        Space::new().width(4),
        sev_chip("WARN", state.sev_filter.warn, Message::ToggleSevWarn),
        Space::new().width(4),
        sev_chip("ERROR", state.sev_filter.error, Message::ToggleSevError),
        Space::new().width(Length::Fill),
        pick_list(TIME_RANGES, Some(state.time_range), Message::SetTimeRange)
            .text_size(11)
            .padding(Padding::from([6.0_f32, 10.0]))
            .style(|_theme, _status| iced::widget::pick_list::Style {
                text_color: design::FG,
                placeholder_color: design::FG_SUBTLE,
                background: Background::Color(design::SURFACE2),
                border: Border { color: design::BORDER, width: 1.0, radius: 6.0.into() },
                handle_color: design::FG_MUTED,
            }),
    ]
    .align_y(Alignment::Center)
    .into()
}

fn action_bar<'a>(state: &'a ConfigurationApp, count: usize) -> Element<'a, Message> {
    let live_tail = state.live_tail;

    let live_dot: Element<'_, Message> = if live_tail {
        container(Space::new())
            .width(Length::Fixed(8.0))
            .height(Length::Fixed(8.0))
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(Background::Color(design::SUCCESS_FG)),
                border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 99.0.into() },
                ..container::Style::default()
            })
            .into()
    } else {
        Space::new().width(0).into()
    };

    let live_tail_btn = iced::widget::button(
        row![
            live_dot,
            Space::new().width(if live_tail { 6 } else { 0 }),
            text(if live_tail { "Live Tail ON" } else { "Live Tail" })
                .size(12)
                .color(if live_tail { design::SUCCESS_FG } else { design::FG_MUTED }),
        ]
        .align_y(Alignment::Center),
    )
    .padding(Padding::from([6.0_f32, 12.0]))
    .on_press(Message::ToggleLiveTail)
    .style(move |_theme, status| {
        let hov = matches!(status, iced::widget::button::Status::Hovered);
        iced::widget::button::Style {
            background: Some(Background::Color(if live_tail {
                design::SUCCESS_BG
            } else if hov {
                design::HOVER
            } else {
                design::SURFACE2
            })),
            border: Border {
                color: if live_tail { design::SUCCESS_FG } else { design::BORDER },
                width: 1.0,
                radius: 99.0.into(),
            },
            text_color: if live_tail { design::SUCCESS_FG } else { design::FG_MUTED },
            ..iced::widget::button::Style::default()
        }
    });

    row![
        live_tail_btn,
        Space::new().width(12),
        text(format!("{} entries", count)).size(12).color(design::FG_MUTED),
        Space::new().width(Length::Fill),
        iced::widget::button(text("Clear").size(12).color(design::FG_MUTED))
            .padding(Padding::from([6.0_f32, 12.0]))
            .on_press(Message::ClearLogs)
            .style(|_theme, status| {
                let hov = matches!(status, iced::widget::button::Status::Hovered);
                iced::widget::button::Style {
                    background: Some(Background::Color(if hov { design::HOVER } else { design::SURFACE2 })),
                    border: Border { color: design::BORDER, width: 1.0, radius: 6.0.into() },
                    text_color: design::FG_MUTED,
                    ..iced::widget::button::Style::default()
                }
            }),
        Space::new().width(8),
        iced::widget::button(text("Export").size(12).color(design::FG_MUTED))
            .padding(Padding::from([6.0_f32, 12.0]))
            .on_press(Message::ExportLogs)
            .style(|_theme, status| {
                let hov = matches!(status, iced::widget::button::Status::Hovered);
                iced::widget::button::Style {
                    background: Some(Background::Color(if hov { design::HOVER } else { design::SURFACE2 })),
                    border: Border { color: design::BORDER, width: 1.0, radius: 6.0.into() },
                    text_color: design::FG_MUTED,
                    ..iced::widget::button::Style::default()
                }
            }),
    ]
    .align_y(Alignment::Center)
    .into()
}

fn log_table<'a>(state: &'a ConfigurationApp, entries: &[&'a LogEntry]) -> Element<'a, Message> {
    let header = container(
        row![
            text("TIMESTAMP").size(10).color(design::FG_SUBTLE).font(iced::Font {
                weight: Weight::Semibold,
                ..crate::theme::layout::fonts::INTER
            }).width(Length::Fixed(110.0)),
            text("SEV").size(10).color(design::FG_SUBTLE).font(iced::Font {
                weight: Weight::Semibold,
                ..crate::theme::layout::fonts::INTER
            }).width(Length::Fixed(56.0)),
            text("SOURCE").size(10).color(design::FG_SUBTLE).font(iced::Font {
                weight: Weight::Semibold,
                ..crate::theme::layout::fonts::INTER
            }).width(Length::Fixed(80.0)),
            text("MESSAGE").size(10).color(design::FG_SUBTLE).font(iced::Font {
                weight: Weight::Semibold,
                ..crate::theme::layout::fonts::INTER
            }).width(Length::Fill),
        ]
        .align_y(Alignment::Center)
        .spacing(0),
    )
    .padding(Padding::from([8.0_f32, 12.0]))
    .width(Length::Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(design::SURFACE2)),
        border: Border {
            color: design::BORDER,
            width: 1.0,
            radius: Radius { top_left: 8.0, top_right: 8.0, bottom_right: 0.0, bottom_left: 0.0 },
        },
        ..container::Style::default()
    });

    let mut rows_col = column![].spacing(0);

    if entries.is_empty() {
        rows_col = rows_col.push(
            container(
                text("No log entries match the current filters.")
                    .size(13).color(design::FG_MUTED),
            )
            .padding(Padding::from([24.0_f32, 16.0]))
            .center_x(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(design::SURFACE)),
                border: Border {
                    color: design::BORDER,
                    width: 1.0,
                    radius: Radius { top_left: 0.0, top_right: 0.0, bottom_right: 8.0, bottom_left: 8.0 },
                },
                ..container::Style::default()
            }),
        );
    } else {
        for (i, entry) in entries.iter().enumerate() {
            let is_last = i == entries.len() - 1;
            rows_col = rows_col.push(log_row(state, entry, is_last));
        }
    }

    let table_content = column![header, rows_col].spacing(0);

    container(
        scrollable(table_content)
            .height(Length::Fixed(380.0))
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .into()
}

fn log_row<'a>(state: &'a ConfigurationApp, entry: &'a LogEntry, is_last: bool) -> Element<'a, Message> {
    let radius = if is_last {
        Radius { top_left: 0.0, top_right: 0.0, bottom_right: 8.0, bottom_left: 8.0 }
    } else {
        Radius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 }
    };

    let (sev_fg, sev_bg) = sev_colors(entry.severity);

    let sev_badge = container(
        text(format!("{}", entry.severity)).size(9).color(sev_fg).font(iced::Font {
            weight: Weight::Bold,
            ..crate::theme::layout::fonts::INTER
        }),
    )
    .padding(Padding::from([2.0_f32, 5.0]))
    .style(move |_| container::Style {
        background: Some(Background::Color(sev_bg)),
        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 3.0.into() },
        ..container::Style::default()
    });

    let expanded = state.expanded_log == Some(entry.id);
    let entry_id = entry.id;

    let row_content = row![
        text(entry.timestamp.as_str())
            .size(11).color(design::FG_SUBTLE)
            .font(crate::theme::layout::fonts::JETBRAINS_MONO)
            .width(Length::Fixed(110.0)),
        container(sev_badge).width(Length::Fixed(56.0)),
        text(entry.source.as_str())
            .size(11).color(design::FG_MUTED)
            .font(crate::theme::layout::fonts::JETBRAINS_MONO)
            .width(Length::Fixed(80.0)),
        text(entry.message.as_str()).size(12).color(design::FG).width(Length::Fill),
    ]
    .align_y(Alignment::Center)
    .spacing(0);

    let row_btn = iced::widget::button(row_content)
        .padding(Padding::from([7.0_f32, 12.0]))
        .width(Length::Fill)
        .on_press(Message::ExpandLog(entry_id))
        .style(move |_theme, status| {
            let hov = matches!(status, iced::widget::button::Status::Hovered);
            iced::widget::button::Style {
                background: Some(Background::Color(if expanded {
                    design::SURFACE2
                } else if hov {
                    design::HOVER
                } else {
                    design::SURFACE
                })),
                border: Border { color: design::BORDER, width: 1.0, radius },
                text_color: design::FG,
                ..iced::widget::button::Style::default()
            }
        });

    if expanded {
        column![
            row_btn,
            container(
                text(format!("Full message: {}", entry.message))
                    .size(11).color(design::FG_MUTED)
                    .font(crate::theme::layout::fonts::JETBRAINS_MONO),
            )
            .padding(Padding::from([8.0_f32, 12.0]))
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(design::SURFACE_ALT)),
                border: Border { color: design::BORDER, width: 1.0, radius },
                ..container::Style::default()
            }),
        ]
        .spacing(0)
        .into()
    } else {
        row_btn.into()
    }
}

fn sev_colors(sev: Severity) -> (Color, Color) {
    match sev {
        Severity::Trace => (design::FG_SUBTLE, design::SURFACE2),
        Severity::Debug => (design::INFO_FG, design::INFO_BG),
        Severity::Info => (design::SUCCESS_FG, design::SUCCESS_BG),
        Severity::Warn => (design::WARN_FG, design::WARN_BG),
        Severity::Error => (design::DANGER_FG, design::DANGER_BG),
    }
}

fn remote_card<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let remote_enabled = state.remote_enabled;
    let remote_sending = state.remote_sending;

    let toggle_btn = iced::widget::button(
        row![
            container(Space::new())
                .width(Length::Fixed(34.0))
                .height(Length::Fixed(18.0))
                .style(move |_| container::Style {
                    background: Some(Background::Color(if remote_enabled { design::ACCENT } else { design::SURFACE2 })),
                    border: Border { color: design::BORDER_STRONG, width: 1.0, radius: 99.0.into() },
                    ..container::Style::default()
                }),
            Space::new().width(8),
            text(if remote_enabled { "Enabled" } else { "Disabled" })
                .size(12).color(if remote_enabled { design::FG } else { design::FG_MUTED }),
        ]
        .align_y(Alignment::Center),
    )
    .padding(Padding::from([0.0_f32, 0.0]))
    .on_press(Message::SetRemoteEnabled(!remote_enabled))
    .style(|_theme, _status| iced::widget::button::Style {
        background: None,
        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 0.0.into() },
        text_color: design::FG,
        ..iced::widget::button::Style::default()
    });

    let remote_fields: Element<'_, Message> = if remote_enabled {
        column![
            Space::new().height(14),
            text("Endpoint URL").size(11).color(design::FG_MUTED),
            Space::new().height(4),
            text_input("https://logs.example.com/ingest", &state.remote_url)
                .on_input(Message::SetRemoteUrl)
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
            Space::new().height(10),
            text("Authorization Header").size(11).color(design::FG_MUTED),
            Space::new().height(4),
            text_input("Bearer <token>", &state.remote_auth)
                .on_input(Message::SetRemoteAuth)
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
            Space::new().height(12),
            row![
                Space::new().width(Length::Fill),
                iced::widget::button(
                    text(if remote_sending { "Sending…" } else { "Send Logs" })
                        .size(12).color(Color::WHITE),
                )
                .padding(Padding::from([7.0_f32, 14.0]))
                .on_press_maybe(if remote_sending { None } else { Some(Message::SendRemoteLogs) })
                .style(move |_theme, status| {
                    let hov = matches!(status, iced::widget::button::Status::Hovered);
                    iced::widget::button::Style {
                        background: Some(Background::Color(if remote_sending {
                            Color { a: 0.5, ..design::ACCENT }
                        } else if hov {
                            design::ACCENT_HOVER
                        } else {
                            design::ACCENT
                        })),
                        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 6.0.into() },
                        text_color: Color::WHITE,
                        ..iced::widget::button::Style::default()
                    }
                }),
            ]
            .align_y(Alignment::Center),
            {
                let status_text: Element<'_, Message> = if !state.remote_status.is_empty() {
                    row![
                        Space::new().width(12),
                        text(state.remote_status.as_str()).size(11).color(design::FG_MUTED),
                    ]
                    .into()
                } else {
                    Space::new().width(0).into()
                };
                status_text
            },
        ]
        .spacing(0)
        .into()
    } else {
        Space::new().into()
    };

    card(column![
        row![
            text("Remote Log Submission").size(14).color(design::FG).font(iced::Font {
                weight: Weight::Semibold,
                ..crate::theme::layout::fonts::INTER
            }),
            Space::new().width(Length::Fill),
            toggle_btn,
        ]
        .align_y(Alignment::Center),
        Space::new().height(4),
        text("Forward log entries to a remote HTTP endpoint in real time.")
            .size(12).color(design::FG_MUTED),
        remote_fields,
    ]
    .spacing(0))
}

fn log_file_card<'a>() -> Element<'a, Message> {
    card(column![
        text("Log File Location").size(14).color(design::FG).font(iced::Font {
            weight: Weight::Semibold,
            ..crate::theme::layout::fonts::INTER
        }),
        Space::new().height(10),
        row![
            container(
                text("%APPDATA%\\Quill\\logs\\quill.log")
                    .size(12).color(design::FG_MUTED)
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
            iced::widget::button(text("Open Folder").size(12).color(design::FG_MUTED))
                .padding(Padding::from([7.0_f32, 12.0]))
                .on_press(Message::ExportLogs)
                .style(|_theme, status| {
                    let hov = matches!(status, iced::widget::button::Status::Hovered);
                    iced::widget::button::Style {
                        background: Some(Background::Color(if hov { design::HOVER } else { design::SURFACE2 })),
                        border: Border { color: design::BORDER_STRONG, width: 1.0, radius: 6.0.into() },
                        text_color: design::FG_MUTED,
                        ..iced::widget::button::Style::default()
                    }
                }),
        ]
        .align_y(Alignment::Center),
    ]
    .spacing(0))
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
