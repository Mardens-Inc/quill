use crate::configuration_app::{ConfigurationApp, Message, Orientation, PRINT_SPEEDS};
use crate::theme::design;
use crate::theme::Icon;
use crate::theme::icon;
use iced::font::Weight;
use iced::widget::{Space, column, container, pick_list, row, slider, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding};

pub fn print_settings_view<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    column![
        text("Print Settings").size(22).color(design::FG).font(iced::Font {
            weight: Weight::Semibold,
            ..crate::theme::layout::fonts::INTER
        }),
        Space::new().height(4),
        text("Configure default print parameters").size(13).color(design::FG_MUTED),
        Space::new().height(24),
        density_card(state),
        Space::new().height(16),
        row![
            speed_card(state),
            Space::new().width(16),
            orientation_card(state),
        ],
        Space::new().height(16),
        scale_card(state),
        Space::new().height(16),
        advanced_card(state),
    ]
    .spacing(0)
    .width(Length::Fill)
    .into()
}

fn density_card<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    card(column![
        row![
            column![
                text("Darkness / Density").size(14).color(design::FG).font(iced::Font {
                    weight: Weight::Semibold,
                    ..crate::theme::layout::fonts::INTER
                }),
                Space::new().height(2),
                text("Controls thermal print head heat (0 = lightest, 15 = darkest)")
                    .size(12).color(design::FG_MUTED),
            ],
            Space::new().width(Length::Fill),
            badge(state.density.to_string()),
        ]
        .align_y(Alignment::Center),
        Space::new().height(16),
        slider(0u8..=15u8, state.density, Message::SetDensity)
            .step(1u8)
            .style(|_theme, _status| iced::widget::slider::Style {
                rail: iced::widget::slider::Rail {
                    backgrounds: (
                        Background::Color(design::ACCENT),
                        Background::Color(design::SURFACE2),
                    ),
                    width: 4.0,
                    border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 2.0.into() },
                },
                handle: iced::widget::slider::Handle {
                    shape: iced::widget::slider::HandleShape::Circle { radius: 8.0 },
                    background: Background::Color(design::ACCENT),
                    border_width: 2.0,
                    border_color: Color::WHITE,
                },
            }),
        Space::new().height(8),
        row![
            text("0").size(10).color(design::FG_SUBTLE),
            Space::new().width(Length::Fill),
            text("15").size(10).color(design::FG_SUBTLE),
        ],
    ]
    .spacing(0))
}

fn speed_card<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    card(
        column![
            text("Print Speed").size(14).color(design::FG).font(iced::Font {
                weight: Weight::Semibold,
                ..crate::theme::layout::fonts::INTER
            }),
            Space::new().height(2),
            text("Inches per second").size(12).color(design::FG_MUTED),
            Space::new().height(16),
            pick_list(PRINT_SPEEDS, Some(state.speed), Message::SetSpeed)
                .text_size(13)
                .padding(Padding::from([8.0_f32, 12.0]))
                .width(Length::Fill)
                .style(|_theme, _status| iced::widget::pick_list::Style {
                    text_color: design::FG,
                    placeholder_color: design::FG_SUBTLE,
                    background: Background::Color(design::INPUT_BG),
                    border: Border { color: design::INPUT_BORDER, width: 1.0, radius: 6.0.into() },
                    handle_color: design::FG_MUTED,
                }),
        ]
        .spacing(0)
        .width(Length::Fill),
    )
}

fn orientation_card<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let portrait_active = state.orientation == Orientation::Portrait;
    let landscape_active = state.orientation == Orientation::Landscape;

    let portrait_btn = seg_btn("Portrait", portrait_active, Message::SetOrientation(Orientation::Portrait));
    let landscape_btn = seg_btn("Landscape", landscape_active, Message::SetOrientation(Orientation::Landscape));

    card(
        column![
            text("Default Orientation").size(14).color(design::FG).font(iced::Font {
                weight: Weight::Semibold,
                ..crate::theme::layout::fonts::INTER
            }),
            Space::new().height(2),
            text("Label rotation").size(12).color(design::FG_MUTED),
            Space::new().height(16),
            container(row![portrait_btn, landscape_btn].spacing(0))
                .style(|_| container::Style {
                    background: Some(Background::Color(design::SURFACE2)),
                    border: Border { color: design::BORDER, width: 1.0, radius: 6.0.into() },
                    ..container::Style::default()
                }),
        ]
        .spacing(0)
        .width(Length::Fill),
    )
}

fn seg_btn<'a>(label: &'a str, active: bool, msg: Message) -> Element<'a, Message> {
    iced::widget::button(
        text(label).size(12).color(if active { Color::WHITE } else { design::FG_MUTED }),
    )
    .padding(Padding::from([7.0_f32, 16.0]))
    .on_press(msg)
    .style(move |_theme, status| {
        let hov = matches!(status, iced::widget::button::Status::Hovered);
        iced::widget::button::Style {
            background: Some(Background::Color(if active {
                design::ACCENT
            } else if hov {
                design::HOVER
            } else {
                Color::TRANSPARENT
            })),
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 4.0.into() },
            text_color: if active { Color::WHITE } else { design::FG_MUTED },
            ..iced::widget::button::Style::default()
        }
    })
    .into()
}

fn scale_card<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    card(column![
        row![
            column![
                text("Default Scale").size(14).color(design::FG).font(iced::Font {
                    weight: Weight::Semibold,
                    ..crate::theme::layout::fonts::INTER
                }),
                Space::new().height(2),
                text("Label output size as a percentage").size(12).color(design::FG_MUTED),
            ],
            Space::new().width(Length::Fill),
            badge(format!("{}%", state.scale)),
        ]
        .align_y(Alignment::Center),
        Space::new().height(16),
        slider(50u8..=150u8, state.scale, Message::SetScale)
            .step(1u8)
            .style(|_theme, _status| iced::widget::slider::Style {
                rail: iced::widget::slider::Rail {
                    backgrounds: (
                        Background::Color(design::ACCENT),
                        Background::Color(design::SURFACE2),
                    ),
                    width: 4.0,
                    border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 2.0.into() },
                },
                handle: iced::widget::slider::Handle {
                    shape: iced::widget::slider::HandleShape::Circle { radius: 8.0 },
                    background: Background::Color(design::ACCENT),
                    border_width: 2.0,
                    border_color: Color::WHITE,
                },
            }),
        Space::new().height(8),
        row![
            text("50%").size(10).color(design::FG_SUBTLE),
            Space::new().width(Length::Fill),
            text("150%").size(10).color(design::FG_SUBTLE),
        ],
    ]
    .spacing(0))
}

fn advanced_card<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let chevron_icon = if state.advanced_open {
        icon(Icon::lucide().chevron_up(), 14, Some((design::FG_MUTED, design::FG_MUTED)))
    } else {
        icon(Icon::lucide().chevron_down(), 14, Some((design::FG_MUTED, design::FG_MUTED)))
    };

    let toggle_btn = iced::widget::button(
        row![
            text("Advanced").size(14).color(design::FG).font(iced::Font {
                weight: Weight::Semibold,
                ..crate::theme::layout::fonts::INTER
            }),
            Space::new().width(Length::Fill),
            chevron_icon,
        ]
        .align_y(Alignment::Center),
    )
    .padding(Padding::from([0.0_f32, 0.0]))
    .width(Length::Fill)
    .on_press(Message::ToggleAdvanced)
    .style(|_theme, _status| iced::widget::button::Style {
        background: None,
        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 0.0.into() },
        text_color: design::FG,
        ..iced::widget::button::Style::default()
    });

    let advanced_content: Element<'_, Message> = if state.advanced_open {
        column![
            divider(),
            Space::new().height(14),
            row![
                column![
                    text("Mono Threshold").size(14).color(design::FG).font(iced::Font {
                        weight: Weight::Semibold,
                        ..crate::theme::layout::fonts::INTER
                    }),
                    Space::new().height(2),
                    text("Grayscale cutoff for monochrome conversion (0–255)")
                        .size(12).color(design::FG_MUTED),
                ],
                Space::new().width(Length::Fill),
                badge(state.mono_threshold.to_string()),
            ]
            .align_y(Alignment::Center),
            Space::new().height(12),
            slider(0u8..=255u8, state.mono_threshold, Message::SetMonoThreshold)
                .step(1u8)
                .style(|_theme, _status| iced::widget::slider::Style {
                    rail: iced::widget::slider::Rail {
                        backgrounds: (
                            Background::Color(design::ACCENT),
                            Background::Color(design::SURFACE2),
                        ),
                        width: 4.0,
                        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 2.0.into() },
                    },
                    handle: iced::widget::slider::Handle {
                        shape: iced::widget::slider::HandleShape::Circle { radius: 8.0 },
                        background: Background::Color(design::ACCENT),
                        border_width: 2.0,
                        border_color: Color::WHITE,
                    },
                }),
            Space::new().height(8),
            row![
                text("0").size(10).color(design::FG_SUBTLE),
                Space::new().width(Length::Fill),
                text("255").size(10).color(design::FG_SUBTLE),
            ],
        ]
        .spacing(0)
        .into()
    } else {
        Space::new().into()
    };

    card(column![toggle_btn, advanced_content].spacing(0))
}

fn badge<'a>(label: impl ToString) -> Element<'a, Message> {
    container(
        text(label.to_string()).size(13).color(design::FG).font(iced::Font {
            weight: Weight::Semibold,
            ..crate::theme::layout::fonts::JETBRAINS_MONO
        }),
    )
    .padding(Padding::from([4.0_f32, 10.0]))
    .style(|_| container::Style {
        background: Some(Background::Color(design::SURFACE2)),
        border: Border { color: design::BORDER_STRONG, width: 1.0, radius: 6.0.into() },
        ..container::Style::default()
    })
    .into()
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

fn divider<'a>() -> Element<'a, Message> {
    container(Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(|_| container::Style {
            background: Some(Background::Color(design::BORDER)),
            ..container::Style::default()
        })
        .into()
}
