use crate::configuration_app::{ConfigurationApp, HelperStatus, Message};
use crate::theme::design;
use crate::theme::Icon;
use crate::theme::icon;
use iced::font::Weight;
use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding};

pub fn about_view<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    column![
        text("About").size(22).color(design::FG).font(iced::Font {
            weight: Weight::Semibold,
            ..crate::theme::layout::fonts::INTER
        }),
        Space::new().height(4),
        text("Version information and support contacts")
            .size(13).color(design::FG_MUTED),
        Space::new().height(24),
        logo_card(state),
        Space::new().height(16),
        version_card(state),
        Space::new().height(16),
        support_card(),
    ]
    .spacing(0)
    .width(Length::Fill)
    .into()
}

fn logo_card<'a>(_state: &'a ConfigurationApp) -> Element<'a, Message> {
    card(
        row![
            container(
                icon(Icon::lucide().printer(), 40, Some((design::ACCENT, design::ACCENT))),
            )
            .padding(Padding::from([12.0_f32, 12.0]))
            .style(|_| container::Style {
                background: Some(Background::Color(design::ACCENT_SOFT)),
                border: Border { color: design::ACCENT, width: 1.0, radius: 12.0.into() },
                ..container::Style::default()
            }),
            Space::new().width(18),
            column![
                text("Quill Configurator").size(20).color(design::FG).font(iced::Font {
                    weight: Weight::Bold,
                    ..crate::theme::layout::fonts::INTER
                }),
                Space::new().height(4),
                text("Admin configuration interface for the Quill thermal print helper service.")
                    .size(12).color(design::FG_MUTED),
                Space::new().height(6),
                container(
                    text("Dark theme · Custom titlebar · iced-rs 0.14")
                        .size(10).color(design::FG_SUBTLE)
                        .font(crate::theme::layout::fonts::JETBRAINS_MONO),
                )
                .padding(Padding::from([3.0_f32, 8.0]))
                .style(|_| container::Style {
                    background: Some(Background::Color(design::SURFACE2)),
                    border: Border { color: design::BORDER, width: 1.0, radius: 4.0.into() },
                    ..container::Style::default()
                }),
            ]
            .spacing(0),
        ]
        .align_y(Alignment::Center),
    )
}

fn version_card<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let helper_status_text = match state.helper_status {
        HelperStatus::Running => ("Running", design::SUCCESS_FG, design::SUCCESS_BG),
        HelperStatus::Stopped => ("Stopped", design::DANGER_FG, design::DANGER_BG),
        HelperStatus::Restarting => ("Restarting", design::WARN_FG, design::WARN_BG),
    };

    let rows: &[(&str, &str)] = &[
        ("App Version", "2.4.1"),
        ("Config Schema", "v7"),
        ("Listening Port", "9100"),
        ("Build", "1180"),
    ];

    let mut table = column![].spacing(0);

    for (label, value) in rows.iter() {
        table = table.push(version_row(label, value));
    }

    let helper_fg = helper_status_text.1;
    let helper_bg = helper_status_text.2;

    let helper_row = container(
        row![
            text("Helper Service").size(12).color(design::FG_MUTED).width(Length::Fixed(180.0)),
            text("2.4.0 · build 1180").size(12).color(design::FG)
                .font(crate::theme::layout::fonts::JETBRAINS_MONO)
                .width(Length::Fill),
            container(
                text(helper_status_text.0).size(10).color(helper_fg),
            )
            .padding(Padding::from([2.0_f32, 7.0]))
            .style(move |_| container::Style {
                background: Some(Background::Color(helper_bg)),
                border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 99.0.into() },
                ..container::Style::default()
            }),
        ]
        .align_y(Alignment::Center),
    )
    .padding(Padding::from([10.0_f32, 14.0]))
    .width(Length::Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(design::SURFACE)),
        border: Border {
            color: design::BORDER,
            width: 1.0,
            radius: iced::border::Radius {
                top_left: 0.0,
                top_right: 0.0,
                bottom_right: 8.0,
                bottom_left: 8.0,
            },
        },
        ..container::Style::default()
    });

    card(column![
        text("Version Info").size(14).color(design::FG).font(iced::Font {
            weight: Weight::Semibold,
            ..crate::theme::layout::fonts::INTER
        }),
        Space::new().height(14),
        table,
        helper_row,
    ]
    .spacing(0))
}

fn version_row<'a>(label: &'a str, value: &'a str) -> Element<'a, Message> {
    container(
        row![
            text(label).size(12).color(design::FG_MUTED).width(Length::Fixed(180.0)),
            text(value).size(12).color(design::FG)
                .font(crate::theme::layout::fonts::JETBRAINS_MONO),
        ]
        .align_y(Alignment::Center),
    )
    .padding(Padding::from([10.0_f32, 14.0]))
    .width(Length::Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(design::SURFACE)),
        border: Border { color: design::BORDER, width: 1.0, radius: 0.0.into() },
        ..container::Style::default()
    })
    .into()
}

fn support_card<'a>() -> Element<'a, Message> {
    card(column![
        text("Support & Documentation").size(14).color(design::FG).font(iced::Font {
            weight: Weight::Semibold,
            ..crate::theme::layout::fonts::INTER
        }),
        Space::new().height(4),
        text("For internal use only. Contact the IT team for assistance.")
            .size(12).color(design::FG_MUTED),
        Space::new().height(14),
        support_link("IT Support", "it-support@quillco.internal"),
        Space::new().height(8),
        support_link("Internal Docs", "https://docs.quillco.internal/quill"),
        Space::new().height(8),
        support_link("GitHub", "https://github.com/quillco/quill"),
    ]
    .spacing(0))
}

fn support_link<'a>(label: &'a str, href: &'a str) -> Element<'a, Message> {
    row![
        text(label).size(12).color(design::FG_MUTED).width(Length::Fixed(120.0)),
        text(href).size(12).color(design::INFO_FG)
            .font(crate::theme::layout::fonts::JETBRAINS_MONO),
    ]
    .align_y(Alignment::Center)
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
