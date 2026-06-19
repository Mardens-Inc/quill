use crate::theme::{color, icon};
use iced::widget::{Row, button as btn};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding};

/// Visual style variants, mirroring HeroUI's button variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Tertiary,
    Outline,
    Ghost,
    Danger,
    DangerSoft,
}

/// Button sizing, mirroring HeroUI's `sm | md | lg`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Corner radius, independent of [`ButtonSize`].
///
/// `Full` is a fully rounded (pill) shape: Iced clamps each corner radius to
/// half the smallest dimension, so an oversized value rounds the ends completely.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonRadius {
    None,
    Small,
    Medium,
    Large,
    Full,
}

impl ButtonRadius {
    fn value(self) -> f32 {
        match self {
            ButtonRadius::None => 0.0,
            ButtonRadius::Small => 4.0,
            ButtonRadius::Medium => 8.0,
            ButtonRadius::Large => 12.0,
            // Far larger than any button height; Iced clamps it to a pill.
            ButtonRadius::Full => 9999.0,
        }
    }
}

impl ButtonSize {
    /// Inner padding. Icon-only buttons get uniform (square) padding.
    fn padding(self, icon_only: bool) -> Padding {
        if icon_only {
            match self {
                ButtonSize::Small => 6.0,
                ButtonSize::Medium => 8.0,
                ButtonSize::Large => 10.0,
            }
            .into()
        } else {
            match self {
                ButtonSize::Small => [6.0, 12.0],
                ButtonSize::Medium => [8.0, 16.0],
                ButtonSize::Large => [10.0, 20.0],
            }
            .into()
        }
    }

    fn radius(self) -> f32 {
        match self {
            ButtonSize::Small => 6.0,
            ButtonSize::Medium => 8.0,
            ButtonSize::Large => 10.0,
        }
    }
}

/// A composable, HeroUI-styled button builder.
///
/// Construct with [`button`], chain configuration, then drop it into any
/// widget tree — it converts to an [`Element`] via `Into`, so `row![button(..)]`
/// works directly. Message dispatch is handled by the underlying
/// `iced::widget::button` via [`Button::on_press`]; there is no `update` method.
pub struct Button<'a, M> {
    content: Element<'a, M>,
    variant: ButtonVariant,
    size: ButtonSize,
    /// `None` follows the size variant; `Some` overrides it.
    radius: Option<ButtonRadius>,
    start_icon: Option<Element<'a, M>>,
    end_icon: Option<Element<'a, M>>,
    is_icon_only: bool,
    full_width: bool,
    disabled: bool,
    pending: bool,
    on_press: Option<M>,
}

impl<'a, M: 'a> Button<'a, M> {
    pub fn new(content: impl Into<Element<'a, M>>) -> Self {
        Button {
            content: content.into(),
            variant: ButtonVariant::default(),
            size: ButtonSize::default(),
            radius: None,
            start_icon: None,
            end_icon: None,
            is_icon_only: false,
            full_width: false,
            disabled: false,
            pending: false,
            on_press: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }
    pub fn secondary(mut self) -> Self {
        self.variant = ButtonVariant::Secondary;
        self
    }
    pub fn tertiary(mut self) -> Self {
        self.variant = ButtonVariant::Tertiary;
        self
    }
    pub fn outline(mut self) -> Self {
        self.variant = ButtonVariant::Outline;
        self
    }
    pub fn ghost(mut self) -> Self {
        self.variant = ButtonVariant::Ghost;
        self
    }
    pub fn danger(mut self) -> Self {
        self.variant = ButtonVariant::Danger;
        self
    }
    pub fn danger_soft(mut self) -> Self {
        self.variant = ButtonVariant::DangerSoft;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn sm(mut self) -> Self {
        self.size = ButtonSize::Small;
        self
    }
    pub fn md(mut self) -> Self {
        self.size = ButtonSize::Medium;
        self
    }
    pub fn lg(mut self) -> Self {
        self.size = ButtonSize::Large;
        self
    }

    /// Override the corner radius independently of the size variant.
    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Fully rounded (pill) corners.
    pub fn rounded_full(mut self) -> Self {
        self.radius = Some(ButtonRadius::Full);
        self
    }

    /// Leading icon, rendered before the content.
    pub fn start_icon(mut self, markup: &'static str, size: impl Into<Length> + Copy) -> Self {
        self.start_icon = Some(icon(markup, size, None));
        self
    }

    /// Trailing icon, rendered after the content.
    pub fn end_icon(mut self, markup: &'static str, size: impl Into<Length> + Copy) -> Self {
        self.end_icon = Some(icon(markup, size, None));
        self
    }

    pub fn icon_only(mut self) -> Self {
        self.is_icon_only = true;
        self
    }

    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn pending(mut self, pending: bool) -> Self {
        self.pending = pending;
        self
    }

    pub fn on_press(mut self, message: M) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl<'a, M: Clone + 'a> From<Button<'a, M>> for Element<'a, M> {
    fn from(value: Button<'a, M>) -> Self {
        let Button {
            content,
            variant,
            size,
            radius,
            start_icon,
            end_icon,
            is_icon_only,
            full_width,
            disabled,
            pending,
            on_press,
        } = value;

        // `pending` blocks interaction like `disabled` (HeroUI: no pointer events while loading).
        let inert = disabled || pending;

        // Compose leading/trailing icons around the content when present.
        let inner: Element<'a, M> = if start_icon.is_some() || end_icon.is_some() {
            let mut row = Row::new().spacing(8).align_y(Alignment::Center);
            if let Some(start) = start_icon {
                row = row.push(start);
            }
            row = row.push(content);
            if let Some(end) = end_icon {
                row = row.push(end);
            }
            row.into()
        } else {
            content
        };

        // Explicit radius wins; otherwise derive from the size variant.
        let radius = radius
            .map(ButtonRadius::value)
            .unwrap_or_else(|| size.radius());

        let mut widget =
            btn(inner)
                .padding(size.padding(is_icon_only))
                .style(move |_theme, status| {
                    // A `None` handler also surfaces as `Disabled`; treat it like our flags.
                    let dim = inert || matches!(status, btn::Status::Disabled);
                    let (background, mut text_color, border_color, border_width) =
                        palette(variant, status);

                    let mut background = background;
                    if dim {
                        text_color.a *= 0.4;
                        background = background.map(|mut c| {
                            c.a *= 0.4;
                            c
                        });
                    }

                    btn::Style {
                        background: background.map(Background::Color),
                        text_color,
                        border: Border {
                            color: border_color,
                            width: border_width,
                            radius: radius.into(),
                        },
                        ..btn::Style::default()
                    }
                });

        if full_width {
            widget = widget.width(Length::Fill);
        }

        // Leaving `on_press` unset makes Iced render the button as disabled.
        if !inert {
            widget = widget.on_press_maybe(on_press);
        }

        widget.into()
    }
}

/// Returns `(background, text, border_color, border_width)` for a variant/state.
fn palette(variant: ButtonVariant, status: btn::Status) -> (Option<Color>, Color, Color, f32) {
    let hovered = matches!(status, btn::Status::Hovered);
    let pressed = matches!(status, btn::Status::Pressed);
    let active = hovered || pressed;

    match variant {
        ButtonVariant::Primary => {
            let bg = if pressed {
                color::Primary::_700
            } else if hovered {
                color::Primary::_600
            } else {
                color::Primary::DEFAULT
            };
            (
                Some(bg),
                color::Primary::FOREGROUND,
                Color::TRANSPARENT,
                0.0,
            )
        }
        ButtonVariant::Secondary => {
            let bg = if pressed {
                color::Surface::_2
            } else if hovered {
                color::Surface::_3
            } else {
                color::Default::DEFAULT
            };
            (Some(bg), color::Ink::DEFAULT, Color::TRANSPARENT, 0.0)
        }
        ButtonVariant::Tertiary => {
            let bg = if active {
                color::Surface::_2
            } else {
                color::Surface::_1
            };
            (Some(bg), color::Ink::DEFAULT, Color::TRANSPARENT, 0.0)
        }
        ButtonVariant::Outline => {
            let bg = active.then_some(color::Surface::_2);
            (bg, color::Ink::DEFAULT, color::Hairline::STRONG, 1.0)
        }
        ButtonVariant::Ghost => {
            let bg = active.then_some(color::Surface::_2);
            (bg, color::Ink::DEFAULT, Color::TRANSPARENT, 0.0)
        }
        ButtonVariant::Danger => {
            let bg = if active {
                Color::from_rgb8(0xe0, 0x4f, 0x4b)
            } else {
                color::Danger::DEFAULT
            };
            (Some(bg), color::Danger::FOREGROUND, Color::TRANSPARENT, 0.0)
        }
        ButtonVariant::DangerSoft => {
            let soft = Color {
                a: if active { 0.24 } else { 0.16 },
                ..color::Danger::DEFAULT
            };
            (Some(soft), color::Danger::DEFAULT, Color::TRANSPARENT, 0.0)
        }
    }
}

pub fn button<'a, M: 'a>(content: impl Into<Element<'a, M>>) -> Button<'a, M> {
    Button::new(content)
}
