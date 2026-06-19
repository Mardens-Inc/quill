use crate::configuration_app::{ConfigurationApp, Message, Stock};
use crate::theme::design;
use crate::theme::Icon;
use crate::theme::icon;
use iced::border::Radius;
use iced::font::Weight;
use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding};

pub fn stocks_view<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    column![
        row![
            column![
                text("Label Stocks").size(22).color(design::FG).font(iced::Font {
                    weight: Weight::Semibold,
                    ..crate::theme::layout::fonts::INTER
                }),
                Space::new().height(4),
                text("Configure label dimensions and liner offsets")
                    .size(13)
                    .color(design::FG_MUTED),
            ]
            .spacing(0),
            Space::new().width(Length::Fill),
            iced::widget::button(text("+ Add Stock").size(13).color(Color::WHITE))
                .padding(Padding::from([8.0_f32, 16.0]))
                .on_press(Message::OpenAddStock)
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
        .align_y(Alignment::Center),
        Space::new().height(24),
        stocks_table(state),
        Space::new().height(12),
        text("All dimensions in millimetres. Gap is the space between label edges.")
            .size(11)
            .color(design::FG_SUBTLE),
    ]
    .spacing(0)
    .width(Length::Fill)
    .into()
}

fn stocks_table<'a>(state: &'a ConfigurationApp) -> Element<'a, Message> {
    let header = container(
        row![
            Space::new().width(48),
            col_header("NAME", Length::Fill),
            col_header("WIDTH", Length::Fixed(70.0)),
            col_header("HEIGHT", Length::Fixed(70.0)),
            col_header("GAP", Length::Fixed(60.0)),
            col_header("LINER L", Length::Fixed(70.0)),
            col_header("LINER R", Length::Fixed(70.0)),
            Space::new().width(64),
        ]
        .align_y(Alignment::Center)
        .spacing(0),
    )
    .padding(Padding::from([10.0_f32, 16.0]))
    .width(Length::Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(design::SURFACE2)),
        border: Border {
            color: design::BORDER,
            width: 1.0,
            radius: Radius {
                top_left: 8.0,
                top_right: 8.0,
                bottom_right: 0.0,
                bottom_left: 0.0,
            },
        },
        ..container::Style::default()
    });

    let mut rows_col = column![].spacing(0);

    for (i, stock) in state.stocks.iter().enumerate() {
        let is_last = i == state.stocks.len() - 1;
        rows_col = rows_col.push(stock_row(stock, is_last));
    }

    if state.stocks.is_empty() {
        rows_col = rows_col.push(
            container(
                text("No stocks configured. Click \"+ Add Stock\" to create one.")
                    .size(13)
                    .color(design::FG_MUTED),
            )
            .padding(Padding::from([32.0_f32, 16.0]))
            .center_x(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(design::SURFACE)),
                border: Border {
                    color: design::BORDER_STRONG,
                    width: 1.0,
                    radius: Radius {
                        top_left: 0.0,
                        top_right: 0.0,
                        bottom_right: 8.0,
                        bottom_left: 8.0,
                    },
                },
                ..container::Style::default()
            }),
        );
    }

    column![header, rows_col].spacing(0).into()
}

fn stock_row<'a>(stock: &'a Stock, is_last: bool) -> Element<'a, Message> {
    let radius = if is_last {
        Radius { top_left: 0.0, top_right: 0.0, bottom_right: 8.0, bottom_left: 8.0 }
    } else {
        Radius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 }
    };

    let max_w = 30.0_f32;
    let max_h = 22.0_f32;
    let aspect = stock.width_mm / stock.height_mm;
    let (thumb_w, thumb_h) = if aspect > max_w / max_h {
        (max_w, max_w / aspect)
    } else {
        (max_h * aspect, max_h)
    };

    let thumb = container(Space::new())
        .width(Length::Fixed(thumb_w))
        .height(Length::Fixed(thumb_h))
        .style(|_| container::Style {
            background: Some(Background::Color(Color::WHITE)),
            border: Border { color: design::BORDER_STRONG, width: 1.0, radius: 2.0.into() },
            ..container::Style::default()
        });

    let thumb_wrapper = container(thumb)
        .width(Length::Fixed(36.0))
        .height(Length::Fixed(30.0))
        .center_x(Length::Fixed(36.0))
        .center_y(Length::Fixed(30.0));

    let stock_id = stock.id;

    let row_el = row![
        thumb_wrapper,
        Space::new().width(12),
        text(stock.name.as_str()).size(13).color(design::FG).width(Length::Fill),
        dim_cell(stock.width_mm, Length::Fixed(70.0)),
        dim_cell(stock.height_mm, Length::Fixed(70.0)),
        dim_cell(stock.gap_mm, Length::Fixed(60.0)),
        dim_cell(stock.liner_left_mm, Length::Fixed(70.0)),
        dim_cell(stock.liner_right_mm, Length::Fixed(70.0)),
        row![
            icon_btn(
                icon(Icon::lucide().pencil(), 14, Some((design::FG_MUTED, design::FG_MUTED))),
                Message::OpenEditStock(stock_id),
            ),
            Space::new().width(4),
            icon_btn(
                icon(Icon::lucide().trash_2(), 14, Some((design::DANGER_FG, design::DANGER_FG))),
                Message::DeleteStock(stock_id),
            ),
        ]
        .align_y(Alignment::Center)
        .width(Length::Fixed(64.0)),
    ]
    .align_y(Alignment::Center)
    .padding(Padding::from([8.0_f32, 16.0]));

    container(row_el)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(design::SURFACE)),
            border: Border { color: design::BORDER, width: 1.0, radius },
            ..container::Style::default()
        })
        .into()
}

fn col_header<'a>(label: &'a str, width: Length) -> Element<'a, Message> {
    text(label)
        .size(10)
        .color(design::FG_SUBTLE)
        .font(iced::Font {
            weight: Weight::Semibold,
            ..crate::theme::layout::fonts::INTER
        })
        .width(width)
        .into()
}

fn dim_cell<'a>(val: f32, width: Length) -> Element<'a, Message> {
    text(format!("{:.1}", val))
        .size(12)
        .color(design::FG_MUTED)
        .font(crate::theme::layout::fonts::JETBRAINS_MONO)
        .width(width)
        .into()
}

fn icon_btn<'a>(icon_el: Element<'a, Message>, msg: Message) -> Element<'a, Message> {
    iced::widget::button(icon_el)
        .padding(Padding::from([4.0_f32, 6.0]))
        .on_press(msg)
        .style(|_theme, status| {
            let hov = matches!(status, iced::widget::button::Status::Hovered);
            iced::widget::button::Style {
                background: if hov { Some(Background::Color(design::HOVER)) } else { None },
                border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 4.0.into() },
                text_color: design::FG_MUTED,
                ..iced::widget::button::Style::default()
            }
        })
        .into()
}
