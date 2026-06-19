use crate::configuration_app::{ConfigurationApp, DpiMode, DpiOverride, Message};
use crate::theme::design;
use iced::widget::{column, container, pick_list, row, text, Space};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding};

pub fn printer_view<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    column![
        section_header("Printer Setup", "Manage connected USB label printers"),
        iced::widget::Space::with_height(24),
        detected_printers_card(state),
        iced::widget::Space::with_height(16),
        active_printer_card(state),
    ]
    .spacing(0)
    .width(Length::Fill)
    .into()
}

fn section_header<'a>(title: &'a str, subtitle: &'a str) -> Element<'a, Message> {
    column![
        text(title)
            .size(22)
            .color(design::FG)
            .font(iced::Font {
                weight: iced::font::Weight::SemiBold,
                ..crate::theme::layout::fonts::INTER
            }),
        iced::widget::Space::with_height(4),
        text(subtitle).size(13).color(design::FG_MUTED),
    ]
    .spacing(0)
    .into()
}

fn detected_printers_card<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let scan_btn = iced::widget::button(
        row![
            text(if state.scanning { "Scanning…" } else { "Scan for Printers" })
                .size(13)
                .color(Color::WHITE),
        ]
        .align_y(Alignment::Center),
    )
    .padding(Padding::from([8, 16]))
    .style(move |_theme, status| {
        let hov = matches!(status, iced::widget::button::Status::Hovered);
        let dim = state.scanning;
        iced::widget::button::Style {
            background: Some(Background::Color(if dim {
                Color { a: 0.5, ..design::ACCENT }
            } else if hov {
                design::ACCENT_HOVER
            } else {
                design::ACCENT
            })),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 6.0.into(),
            },
            text_color: Color::WHITE,
            ..iced::widget::button::Style::default()
        }
    })
    .on_press_maybe(if state.scanning { None } else { Some(Message::ScanPrinters) });

    let header = row![
        text("Detected Printers")
            .size(14)
            .color(design::FG)
            .font(iced::Font {
                weight: iced::font::Weight::SemiBold,
                ..crate::theme::layout::fonts::INTER
            }),
        Space::with_width(Length::Fill),
        scan_btn,
    ]
    .align_y(Alignment::Center);

    let body: Element<'_, Message> = if state.printers.is_empty() && !state.scanning {
        container(
            column![
                text("No printers found").size(14).color(design::FG_MUTED),
                iced::widget::Space::with_height(4),
                text("Click \"Scan for Printers\" to detect USB label printers.")
                    .size(12)
                    .color(design::FG_SUBTLE),
            ]
            .align_x(Alignment::Center),
        )
        .padding(Padding::from([32, 16]))
        .center_x(Length::Fill)
        .into()
    } else if state.scanning {
        container(
            text("Scanning for USB printers…").size(13).color(design::FG_MUTED),
        )
        .padding(Padding::from([24, 16]))
        .center_x(Length::Fill)
        .into()
    } else {
        let mut list = column![].spacing(4);
        for printer in &state.printers {
            let selected = state.selected_printer.as_deref() == Some(printer.name.as_str());
            let status_pill = container(
                text(if printer.online { "Connected" } else { "Offline" })
                    .size(10)
                    .color(if printer.online { design::SUCCESS_FG } else { design::DANGER_FG }),
            )
            .padding(Padding::from([2, 7]))
            .style(move |_| container::Style {
                background: Some(Background::Color(if printer.online {
                    design::SUCCESS_BG
                } else {
                    design::DANGER_BG
                })),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 99.0.into(),
                },
                ..container::Style::default()
            });

            let row_bg = if selected { design::ACCENT_SOFT } else { design::SURFACE2 };

            let printer_row = iced::widget::button(
                row![
                    // Selection indicator
                    container(iced::widget::Space::with_width(3))
                        .height(Length::Fixed(32.0))
                        .style(move |_| container::Style {
                            background: Some(Background::Color(if selected {
                                design::ACCENT
                            } else {
                                Color::TRANSPARENT
                            })),
                            border: Border {
                                color: Color::TRANSPARENT,
                                width: 0.0,
                                radius: 2.0.into(),
                            },
                            ..container::Style::default()
                        }),
                    iced::widget::Space::with_width(10),
                    column![
                        text(printer.name.as_str())
                            .size(13)
                            .color(design::FG),
                        text(printer.meta.as_str())
                            .size(11)
                            .color(design::FG_SUBTLE)
                            .font(crate::theme::layout::fonts::JETBRAINS_MONO),
                    ]
                    .spacing(2),
                    Space::with_width(Length::Fill),
                    status_pill,
                ]
                .align_y(Alignment::Center),
            )
            .padding(Padding::from([8, 12]))
            .width(Length::Fill)
            .on_press(Message::SelectPrinter(printer.name.clone()))
            .style(move |_theme, status| {
                let hov = matches!(status, iced::widget::button::Status::Hovered);
                iced::widget::button::Style {
                    background: Some(Background::Color(if hov && !selected {
                        design::HOVER
                    } else {
                        row_bg
                    })),
                    border: Border {
                        color: if selected { design::ACCENT } else { Color::TRANSPARENT },
                        width: if selected { 1.0 } else { 0.0 },
                        radius: 6.0.into(),
                    },
                    text_color: design::FG,
                    ..iced::widget::button::Style::default()
                }
            });

            list = list.push(printer_row);
        }
        list.into()
    };

    card(column![
        header,
        iced::widget::Space::with_height(16),
        body,
    ]
    .spacing(0))
}

fn active_printer_card<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let Some(name) = &state.selected_printer else {
        return iced::widget::Space::with_height(0).into();
    };

    let printer = state.printers.iter().find(|p| &p.name == name);
    let online = printer.map(|p| p.online).unwrap_or(false);
    let meta = printer.map(|p| p.meta.as_str()).unwrap_or("Unknown");

    let status_pill = container(
        text(if online { "Online" } else { "Offline" })
            .size(10)
            .color(if online { design::SUCCESS_FG } else { design::DANGER_FG }),
    )
    .padding(Padding::from([2, 7]))
    .style(move |_| container::Style {
        background: Some(Background::Color(if online { design::SUCCESS_BG } else { design::DANGER_BG })),
        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 99.0.into() },
        ..container::Style::default()
    });

    let printer_header = row![
        column![
            text("ACTIVE PRINTER").size(10).color(design::FG_SUBTLE).font(iced::Font {
                weight: iced::font::Weight::SemiBold,
                ..crate::theme::layout::fonts::INTER
            }),
            iced::widget::Space::with_height(4),
            text(name.as_str()).size(15).color(design::FG).font(iced::Font {
                weight: iced::font::Weight::SemiBold,
                ..crate::theme::layout::fonts::INTER
            }),
            text(meta).size(11).color(design::FG_SUBTLE).font(crate::theme::layout::fonts::JETBRAINS_MONO),
        ]
        .spacing(0),
        Space::with_width(Length::Fill),
        status_pill,
    ]
    .align_y(Alignment::Center);

    let offline_warn: Option<Element<'_, Message>> = if !online {
        Some(container(
            row![
                text("⚠ Printer is offline. Check USB connection before printing.")
                    .size(12)
                    .color(design::WARN_FG),
            ],
        )
        .padding(Padding::from([10, 14]))
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(design::WARN_BG)),
            border: Border { color: design::WARN_FG, width: 1.0, radius: 6.0.into() },
            ..container::Style::default()
        })
        .into())
    } else {
        None
    };

    // DPI row
    let dpi_auto_btn = iced::widget::button(
        text(format!("Auto · {} dpi", state.dpi_auto)).size(12).color(
            if state.dpi_mode == DpiMode::Auto { Color::WHITE } else { design::FG_MUTED },
        ),
    )
    .padding(Padding::from([6, 14]))
    .on_press(Message::SetDpiMode(DpiMode::Auto))
    .style(move |_theme, status| {
        let hov = matches!(status, iced::widget::button::Status::Hovered);
        let active = state.dpi_mode == DpiMode::Auto;
        iced::widget::button::Style {
            background: Some(Background::Color(if active {
                design::ACCENT
            } else if hov {
                design::HOVER
            } else {
                design::SURFACE2
            })),
            border: Border { color: design::BORDER, width: 1.0, radius: 6.0.into() },
            text_color: if active { Color::WHITE } else { design::FG_MUTED },
            ..iced::widget::button::Style::default()
        }
    });

    let dpi_manual_btn = iced::widget::button(
        text("Manual override").size(12).color(
            if state.dpi_mode == DpiMode::Manual { Color::WHITE } else { design::FG_MUTED },
        ),
    )
    .padding(Padding::from([6, 14]))
    .on_press(Message::SetDpiMode(DpiMode::Manual))
    .style(move |_theme, status| {
        let hov = matches!(status, iced::widget::button::Status::Hovered);
        let active = state.dpi_mode == DpiMode::Manual;
        iced::widget::button::Style {
            background: Some(Background::Color(if active {
                design::ACCENT
            } else if hov {
                design::HOVER
            } else {
                design::SURFACE2
            })),
            border: Border { color: design::BORDER, width: 1.0, radius: 6.0.into() },
            text_color: if active { Color::WHITE } else { design::FG_MUTED },
            ..iced::widget::button::Style::default()
        }
    });

    let dpi_options: &[DpiOverride] = &[DpiOverride::Dpi203, DpiOverride::Dpi300, DpiOverride::Dpi600];

    let dpi_row: Element<'_, Message> = if state.dpi_mode == DpiMode::Manual {
        row![
            dpi_auto_btn,
            iced::widget::Space::with_width(8),
            dpi_manual_btn,
            iced::widget::Space::with_width(12),
            pick_list(dpi_options, Some(state.dpi_override), Message::SetDpiOverride)
                .text_size(12)
                .padding(Padding::from([6, 10]))
                .style(|_theme, _status| iced::widget::pick_list::Style {
                    text_color: design::FG,
                    placeholder_color: design::FG_SUBTLE,
                    background: Background::Color(design::SURFACE2),
                    border: Border { color: design::BORDER_STRONG, width: 1.0, radius: 6.0.into() },
                    handle_color: design::FG_MUTED,
                }),
        ]
        .align_y(Alignment::Center)
        .into()
    } else {
        row![dpi_auto_btn, iced::widget::Space::with_width(8), dpi_manual_btn]
            .align_y(Alignment::Center)
            .into()
    };

    let dpi_section = column![
        text("DPI Mode").size(11).color(design::FG_MUTED),
        iced::widget::Space::with_height(8),
        dpi_row,
    ]
    .spacing(0);

    // Last test print
    let last_test_el: Element<'_, Message> = column![
        text("Last Test Print").size(11).color(design::FG_MUTED),
        iced::widget::Space::with_height(4),
        text(state.last_tested.as_deref().unwrap_or("Never"))
            .size(13)
            .color(design::FG),
        if !state.test_msg.is_empty() {
            text(state.test_msg.as_str())
                .size(11)
                .color(if state.test_ok { design::SUCCESS_FG } else { design::DANGER_FG })
                .into()
        } else {
            iced::widget::Space::with_height(0).into()
        },
    ]
    .spacing(2)
    .into();

    let test_btn = iced::widget::button(
        text(if state.testing { "Printing…" } else { "Test Print" })
            .size(13)
            .color(Color::WHITE),
    )
    .padding(Padding::from([9, 18]))
    .on_press_maybe(if state.testing || !online { None } else { Some(Message::TestPrint) })
    .style(move |_theme, status| {
        let hov = matches!(status, iced::widget::button::Status::Hovered);
        let dim = state.testing || !online;
        iced::widget::button::Style {
            background: Some(Background::Color(if dim {
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
    });

    let bottom_row = row![
        last_test_el,
        Space::with_width(Length::Fill),
        test_btn,
    ]
    .align_y(Alignment::Center);

    let mut content_col = column![
        printer_header,
        iced::widget::Space::with_height(16),
    ]
    .spacing(0);

    if let Some(warn) = offline_warn {
        content_col = content_col.push(warn);
        content_col = content_col.push(iced::widget::Space::with_height(12));
    }

    content_col = content_col.push(divider());
    content_col = content_col.push(iced::widget::Space::with_height(14));
    content_col = content_col.push(dpi_section);
    content_col = content_col.push(iced::widget::Space::with_height(14));
    content_col = content_col.push(divider());
    content_col = content_col.push(iced::widget::Space::with_height(14));
    content_col = content_col.push(bottom_row);

    card(content_col)
}

fn card<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .width(Length::Fill)
        .padding(Padding::from([20, 20]))
        .style(|_| container::Style {
            background: Some(Background::Color(design::SURFACE)),
            border: Border {
                color: design::BORDER_STRONG,
                width: 1.0,
                radius: 10.0.into(),
            },
            ..container::Style::default()
        })
        .into()
}

fn divider<'a>() -> Element<'a, Message> {
    container(iced::widget::Space::with_height(1))
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(|_| container::Style {
            background: Some(Background::Color(design::BORDER)),
            ..container::Style::default()
        })
        .into()
}
