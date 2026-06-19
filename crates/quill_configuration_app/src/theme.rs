aurora_fonts::font_families!("Inter", "JetBrains Mono");
aurora_iconify::icon_sets!("lucide", "material-symbols");

pub mod design {
    use iced::Color;

    pub const APP_BG: Color = Color::from_rgb8(0x0a, 0x0a, 0x0c);
    pub const SURFACE: Color = Color::from_rgb8(0x16, 0x16, 0x19);
    pub const SURFACE2: Color = Color::from_rgb8(0x1d, 0x1d, 0x21);
    pub const SURFACE_ALT: Color = Color::from_rgb8(0x1f, 0x1f, 0x24);
    pub const BORDER: Color = Color::from_rgba8(0xff, 0xff, 0xff, 0.07);
    pub const BORDER_STRONG: Color = Color::from_rgba8(0xff, 0xff, 0xff, 0.14);
    pub const FG: Color = Color::from_rgb8(0xf5, 0xf5, 0xf4);
    pub const FG_MUTED: Color = Color::from_rgb8(0xa1, 0xa1, 0xa6);
    pub const FG_SUBTLE: Color = Color::from_rgb8(0x6e, 0x6e, 0x76);
    pub const TOPBAR: Color = Color::from_rgb8(0x0d, 0x0d, 0x0f);
    pub const SIDEBAR: Color = Color::from_rgb8(0x0a, 0x0a, 0x0c);
    pub const INPUT_BG: Color = Color::from_rgb8(0x16, 0x16, 0x19);
    pub const INPUT_BORDER: Color = Color::from_rgba8(0xff, 0xff, 0xff, 0.12);
    pub const HOVER: Color = Color::from_rgba8(0xff, 0xff, 0xff, 0.06);
    pub const ACCENT: Color = Color::from_rgb8(0xe8, 0x55, 0x2e);
    pub const ACCENT_HOVER: Color = Color::from_rgb8(0xd8, 0x43, 0x1d);
    pub const ACCENT_SOFT: Color = Color::from_rgba8(0xe8, 0x55, 0x2e, 0.16);
    pub const SUCCESS_FG: Color = Color::from_rgb8(0x4a, 0xde, 0x80);
    pub const SUCCESS_BG: Color = Color::from_rgba8(0x4a, 0xde, 0x80, 0.14);
    pub const DANGER_FG: Color = Color::from_rgb8(0xff, 0x7a, 0x6b);
    pub const DANGER_BG: Color = Color::from_rgba8(0xf8, 0x71, 0x71, 0.16);
    pub const WARN_FG: Color = Color::from_rgb8(0xfb, 0xbf, 0x24);
    pub const WARN_BG: Color = Color::from_rgba8(0xfb, 0xbf, 0x24, 0.15);
    pub const INFO_FG: Color = Color::from_rgb8(0x6b, 0xb0, 0xff);
    pub const INFO_BG: Color = Color::from_rgba8(0x60, 0xa5, 0xfa, 0.14);
}

pub mod color {
	use iced::Color;

	// ─── Canvas ─────────────────────────────────────────────────────────────────
	pub const BACKGROUND: Color = Color::from_rgb8(0x00, 0x00, 0x00);
	pub const FOREGROUND: Color = Color::from_rgb8(0xf6, 0xf7, 0xf9);
	pub const DIVIDER: Color = Color::from_rgba8(0xff, 0xff, 0xff, 0.06);
	pub const FOCUS: Color = Color::from_rgb8(0x21, 0xc8, 0xff);

	// ─── Surface elevation ───────────────────────────────────────────────────────
	pub struct Surface;
	impl Surface {
		pub const _0: Color = Color::from_rgb8(0x05, 0x05, 0x05);
		pub const _1: Color = Color::from_rgb8(0x0b, 0x0b, 0x0c);
		pub const _2: Color = Color::from_rgb8(0x13, 0x13, 0x16);
		pub const _3: Color = Color::from_rgb8(0x1a, 0x1a, 0x1e);
	}

	// ─── Ink (text ramp) ─────────────────────────────────────────────────────────
	pub struct Ink;
	impl Ink {
		pub const DEFAULT: Color = Color::from_rgb8(0xf6, 0xf7, 0xf9);
		pub const _1: Color = Color::from_rgb8(0xf6, 0xf7, 0xf9);
		pub const _2: Color = Color::from_rgb8(0xae, 0xb1, 0xb8);
		pub const _3: Color = Color::from_rgb8(0x6b, 0x6e, 0x76);
		pub const _4: Color = Color::from_rgb8(0x45, 0x47, 0x4d);
	}

	// ─── Hairlines ───────────────────────────────────────────────────────────────
	pub struct Hairline;
	impl Hairline {
		pub const DEFAULT: Color = Color::from_rgba8(0xff, 0xff, 0xff, 0.06);
		pub const STRONG: Color = Color::from_rgba8(0xff, 0xff, 0xff, 0.12);
	}

	// ─── Accent (sky-blue) ───────────────────────────────────────────────────────
	// oklch(0.78 0.15 230) ≈ #21c8ff
	pub struct Accent;
	impl Accent {
		pub const DEFAULT: Color = Color::from_rgb8(0x21, 0xc8, 0xff);
		pub const SOFT: Color = Color::from_rgba8(0x21, 0xc8, 0xff, 0.16);
		pub const GLOW: Color = Color::from_rgba8(0x21, 0xc8, 0xff, 0.35);
		pub const INK: Color = Color::from_rgb8(0x00, 0x1a, 0x2c);
	}

	// ─── Semantic ────────────────────────────────────────────────────────────────
	// oklch(0.78 0.15 155) = #59d38c
	pub const OK: Color = Color::from_rgb8(0x59, 0xd3, 0x8c);
	// oklch(0.82 0.15 80)  = #f7b83d
	pub const WARN: Color = Color::from_rgb8(0xf7, 0xb8, 0x3d);
	// oklch(0.72 0.18 295) = #ae89ff
	pub const VIOLET: Color = Color::from_rgb8(0xae, 0x89, 0xff);
	// oklch(0.74 0.17 5)   = #ff779c
	pub const PINK: Color = Color::from_rgb8(0xff, 0x77, 0x9c);

	// ─── HeroUI semantic palette ─────────────────────────────────────────────────
	pub struct Primary;
	impl Primary {
		pub const DEFAULT: Color = Color::from_rgb8(0x21, 0xc8, 0xff);
		pub const FOREGROUND: Color = Color::from_rgb8(0x00, 0x1a, 0x2c);
		pub const _50: Color = Color::from_rgb8(0x03, 0x14, 0x1a);
		pub const _100: Color = Color::from_rgb8(0x05, 0x20, 0x29);
		pub const _200: Color = Color::from_rgb8(0x08, 0x30, 0x3d);
		pub const _300: Color = Color::from_rgb8(0x0c, 0x46, 0x59);
		pub const _400: Color = Color::from_rgb8(0x11, 0x64, 0x80);
		pub const _500: Color = Color::from_rgb8(0x21, 0xc8, 0xff);
		pub const _600: Color = Color::from_rgb8(0x00, 0xb4, 0xf0);
		pub const _700: Color = Color::from_rgb8(0x00, 0x9e, 0xd8);
		pub const _800: Color = Color::from_rgb8(0x00, 0x7e, 0xb7);
		pub const _900: Color = Color::from_rgb8(0x00, 0x51, 0x87);
	}

	pub struct Secondary;
	impl Secondary {
		pub const DEFAULT: Color = Color::from_rgb8(0xae, 0x89, 0xff);
		pub const FOREGROUND: Color = Color::from_rgb8(0xff, 0xff, 0xff);
	}

	pub struct Success;
	impl Success {
		pub const DEFAULT: Color = Color::from_rgb8(0x59, 0xd3, 0x8c);
		pub const FOREGROUND: Color = Color::from_rgb8(0xff, 0xff, 0xff);
	}

	pub struct Warning;
	impl Warning {
		pub const DEFAULT: Color = Color::from_rgb8(0xf7, 0xb8, 0x3d);
		pub const FOREGROUND: Color = Color::from_rgb8(0x00, 0x00, 0x00);
	}

	pub struct Danger;
	impl Danger {
		pub const DEFAULT: Color = Color::from_rgb8(0xff, 0x5f, 0x5b);
		pub const FOREGROUND: Color = Color::from_rgb8(0xff, 0xff, 0xff);
	}

	pub struct Default;
	impl Default {
		pub const DEFAULT: Color = Color::from_rgb8(0x1a, 0x1a, 0x1e);
		pub const FOREGROUND: Color = Color::from_rgb8(0xf6, 0xf7, 0xf9);
		pub const _50: Color = Color::from_rgb8(0x05, 0x05, 0x05);
		pub const _100: Color = Color::from_rgb8(0x0b, 0x0b, 0x0c);
		pub const _200: Color = Color::from_rgb8(0x13, 0x13, 0x16);
		pub const _300: Color = Color::from_rgb8(0x1a, 0x1a, 0x1e);
		pub const _400: Color = Color::from_rgb8(0x45, 0x47, 0x4d);
		pub const _500: Color = Color::from_rgb8(0x6b, 0x6e, 0x76);
		pub const _600: Color = Color::from_rgb8(0xae, 0xb1, 0xb8);
		pub const _700: Color = Color::from_rgb8(0xd1, 0xd3, 0xd8);
		pub const _800: Color = Color::from_rgb8(0xe8, 0xe9, 0xec);
		pub const _900: Color = Color::from_rgb8(0xf6, 0xf7, 0xf9);
	}

	// ─── Content layers ──────────────────────────────────────────────────────────
	pub struct Content1;
	impl Content1 {
		pub const DEFAULT: Color = Color::from_rgb8(0x0b, 0x0b, 0x0c);
		pub const FOREGROUND: Color = Color::from_rgb8(0xf6, 0xf7, 0xf9);
	}

	pub struct Content2;
	impl Content2 {
		pub const DEFAULT: Color = Color::from_rgb8(0x13, 0x13, 0x16);
		pub const FOREGROUND: Color = Color::from_rgb8(0xae, 0xb1, 0xb8);
	}

	pub struct Content3;
	impl Content3 {
		pub const DEFAULT: Color = Color::from_rgb8(0x1a, 0x1a, 0x1e);
		pub const FOREGROUND: Color = Color::from_rgb8(0x6b, 0x6e, 0x76);
	}

	pub struct Content4;
	impl Content4 {
		pub const DEFAULT: Color = Color::from_rgb8(0x24, 0x24, 0x29);
		pub const FOREGROUND: Color = Color::from_rgb8(0x45, 0x47, 0x4d);
	}

	// ─── Tag pills ───────────────────────────────────────────────────────────────
	// oklch approximations → hex
	pub struct Tag;
	impl Tag {
		// prod: oklch(0.20 0.05 25) / oklch(0.40 0.12 25) / oklch(0.78 0.20 25)
		pub const PROD_BG: Color = Color::from_rgb8(0x29, 0x0b, 0x0a);
		pub const PROD_BORDER: Color = Color::from_rgb8(0x7c, 0x25, 0x24);
		pub const PROD_FG: Color = Color::from_rgb8(0xff, 0x7a, 0x73);

		// stag: oklch(0.20 0.05 80) / oklch(0.40 0.12 80) / oklch(0.85 0.15 80)
		pub const STAG_BG: Color = Color::from_rgb8(0x22, 0x13, 0x00);
		pub const STAG_BORDER: Color = Color::from_rgb8(0x68, 0x3d, 0x00);
		pub const STAG_FG: Color = Color::from_rgb8(0xff, 0xc2, 0x49);

		// dev: oklch(0.20 0.05 155) / oklch(0.40 0.12 155) / oklch(0.85 0.15 155)
		pub const DEV_BG: Color = Color::from_rgb8(0x00, 0x1c, 0x0b);
		pub const DEV_BORDER: Color = Color::from_rgb8(0x00, 0x59, 0x28);
		pub const DEV_FG: Color = Color::from_rgb8(0x72, 0xea, 0xa2);
	}

	// ─── Syntax tokens ───────────────────────────────────────────────────────────
	pub struct Syntax;
	impl Syntax {
		pub const KEYWORD: Color = Color::from_rgb8(0xc1, 0x9c, 0xff); // oklch(0.78 0.18 295)
		pub const FN: Color = Color::from_rgb8(0x47, 0xdf, 0xff); // oklch(0.85 0.15 230)
		pub const STRING: Color = Color::from_rgb8(0xf4, 0xc9, 0x47); // oklch(0.85 0.15 90)
		pub const NUMBER: Color = Color::from_rgb8(0xff, 0xa4, 0x9b); // oklch(0.85 0.15 25)
		pub const COMMENT: Color = Color::from_rgb8(0x45, 0x47, 0x4d);
		pub const OPERATOR: Color = Color::from_rgb8(0xae, 0xb1, 0xb8);
		pub const PROPERTY: Color = Color::from_rgb8(0x72, 0xea, 0xa2); // oklch(0.85 0.15 155)
		pub const TYPE: Color = Color::from_rgb8(0x47, 0xdf, 0xff); // oklch(0.85 0.15 230)
		pub const TAG: Color = Color::from_rgb8(0xff, 0x86, 0xae); // oklch(0.80 0.18 5)
	}

	// ─── macOS traffic lights ─────────────────────────────────────────────────────
	pub struct Traffic;
	impl Traffic {
		pub const RED: Color = Color::from_rgb8(0xff, 0x5f, 0x57);
		pub const YELLOW: Color = Color::from_rgb8(0xfe, 0xbc, 0x2e);
		pub const GREEN: Color = Color::from_rgb8(0x28, 0xc8, 0x40);
	}
}

pub mod layout {
	pub mod fonts {
		use iced::Font;

		pub const JETBRAINS_MONO: Font = Font::with_name("JetBrains Mono");
		pub const INTER: Font = Font::with_name("Inter");
	}
}


// theme.rs or an icon module
use iced::widget::svg;
use iced::{Element, Length};

pub fn icon<'a, M: 'a>(
	markup: &'static str,
	size: impl Into<Length> + std::marker::Copy,
	colors: Option<(iced::Color, iced::Color)>,
) -> Element<'a, M> {
	let colors = colors.unwrap_or((color::Ink::DEFAULT, color::Ink::_2));
	svg(svg::Handle::from_memory(markup.as_bytes()))
		.width(size)
		.height(size)
		.style(move |_theme, status| svg::Style {
			color: Some(match status {
				svg::Status::Hovered => colors.0,
				_ => colors.1,
			}),
		})
		.into()
}