use egui::{Color32, FontData, FontDefinitions, FontFamily, Visuals};
use std::{env, fs, path::PathBuf};

#[derive(Clone, Copy)]
pub struct Palette {
    pub mnemonic_underline: Color32,
    pub toolbar_bg: Color32,
    pub muted_text: Color32,
    pub subtle_text: Color32,
    pub dim_text: Color32,
    pub table_grid: Color32,
    pub table_header_text: Color32,
    pub table_header_bg: Color32,
    pub table_header_border: Color32,
    pub table_separator: Color32,
    pub table_selected_row: Color32,
    pub table_non_ms_even: Color32,
    pub table_non_ms_odd: Color32,
    pub table_even_row: Color32,
    pub table_odd_row: Color32,
    pub table_non_ms_name_text: Color32,
    pub table_primary_text: Color32,
    pub table_address_text: Color32,
    pub table_default_text: Color32,
    pub table_row_border: Color32,
    pub empty_state_text: Color32,
}

pub fn initialize_theme(ctx: &egui::Context, dark_mode: bool) {
    configure_fonts(ctx);
    apply_theme(ctx, dark_mode);
}

pub fn apply_theme(ctx: &egui::Context, dark_mode: bool) {
    let mut visuals = if dark_mode {
        dark_visuals()
    } else {
        light_visuals()
    };

    ctx.set_theme(if dark_mode {
        egui::Theme::Dark
    } else {
        egui::Theme::Light
    });

    // Tweak some defaults to keep panels and controls consistent with the selected mode.
    visuals.window_corner_radius = 6.0.into();
    visuals.menu_corner_radius = 6.0.into();
    visuals.widgets.active.corner_radius = 5.0.into();
    visuals.widgets.hovered.corner_radius = 5.0.into();
    visuals.widgets.inactive.corner_radius = 5.0.into();

    ctx.set_visuals(visuals);

    // Set font
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(13.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(16.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Monospace,
        egui::FontId::new(11.0, egui::FontFamily::Monospace),
    );
    ctx.set_style(style);
}

pub fn palette(dark_mode: bool) -> Palette {
    if dark_mode {
        Palette {
            mnemonic_underline: Color32::from_rgb(180, 180, 200),
            toolbar_bg: Color32::from_rgb(18, 18, 28),
            muted_text: Color32::from_rgb(150, 150, 170),
            subtle_text: Color32::from_rgb(100, 100, 120),
            dim_text: Color32::from_rgb(80, 80, 100),
            table_grid: Color32::from_rgb(40, 40, 55),
            table_header_text: Color32::from_rgb(200, 200, 220),
            table_header_bg: Color32::from_rgb(35, 35, 55),
            table_header_border: Color32::from_rgb(55, 55, 75),
            table_separator: Color32::from_rgb(60, 60, 90),
            table_selected_row: Color32::from_rgb(50, 80, 130),
            table_non_ms_even: Color32::from_rgb(32, 28, 22),
            table_non_ms_odd: Color32::from_rgb(36, 32, 26),
            table_even_row: Color32::from_rgb(22, 22, 32),
            table_odd_row: Color32::from_rgb(26, 26, 38),
            table_non_ms_name_text: Color32::from_rgb(220, 200, 150),
            table_primary_text: Color32::from_rgb(240, 240, 240),
            table_address_text: Color32::from_rgb(160, 180, 210),
            table_default_text: Color32::from_rgb(190, 190, 200),
            table_row_border: Color32::from_rgb(35, 35, 50),
            empty_state_text: Color32::from_rgb(120, 120, 120),
        }
    } else {
        Palette {
            mnemonic_underline: Color32::from_rgb(90, 110, 150),
            toolbar_bg: Color32::from_rgb(239, 243, 249),
            muted_text: Color32::from_rgb(96, 106, 122),
            subtle_text: Color32::from_rgb(120, 128, 142),
            dim_text: Color32::from_rgb(145, 150, 160),
            table_grid: Color32::from_rgb(210, 216, 226),
            table_header_text: Color32::from_rgb(78, 88, 104),
            table_header_bg: Color32::from_rgb(226, 232, 240),
            table_header_border: Color32::from_rgb(190, 198, 210),
            table_separator: Color32::from_rgb(186, 194, 206),
            table_selected_row: Color32::from_rgb(203, 220, 246),
            table_non_ms_even: Color32::from_rgb(249, 241, 230),
            table_non_ms_odd: Color32::from_rgb(245, 236, 224),
            table_even_row: Color32::from_rgb(252, 253, 255),
            table_odd_row: Color32::from_rgb(245, 248, 252),
            table_non_ms_name_text: Color32::from_rgb(130, 92, 36),
            table_primary_text: Color32::from_rgb(32, 39, 49),
            table_address_text: Color32::from_rgb(36, 88, 142),
            table_default_text: Color32::from_rgb(75, 84, 98),
            table_row_border: Color32::from_rgb(224, 229, 237),
            empty_state_text: Color32::from_rgb(130, 136, 145),
        }
    }
}

fn dark_visuals() -> Visuals {
    let mut visuals = Visuals::dark();

    // Preserve the existing dark mode colors exactly.
    visuals.override_text_color = Some(Color32::from_rgb(230, 230, 230));
    visuals.panel_fill = Color32::from_rgb(20, 20, 30);
    visuals.window_fill = Color32::from_rgb(25, 25, 35);
    visuals.extreme_bg_color = Color32::from_rgb(15, 15, 25);
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(50, 50, 70);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(70, 70, 100);
    visuals.widgets.active.bg_fill = Color32::from_rgb(80, 120, 180);
    visuals.widgets.inactive.bg_stroke.color = Color32::from_rgb(60, 60, 80);
    visuals.widgets.hovered.bg_stroke.color = Color32::from_rgb(100, 100, 140);
    visuals.selection.bg_fill = Color32::from_rgb(80, 120, 180);
    visuals.selection.stroke.color = Color32::from_rgb(120, 160, 220);
    visuals.hyperlink_color = Color32::from_rgb(100, 180, 255);

    visuals
}

fn light_visuals() -> Visuals {
    let mut visuals = Visuals::light();

    visuals.override_text_color = Some(Color32::from_rgb(32, 39, 49));
    visuals.panel_fill = Color32::from_rgb(248, 250, 253);
    visuals.window_fill = Color32::from_rgb(255, 255, 255);
    visuals.extreme_bg_color = Color32::from_rgb(236, 240, 246);
    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(248, 250, 253);
    visuals.widgets.noninteractive.bg_stroke.color = Color32::from_rgb(206, 214, 226);
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(242, 245, 250);
    visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(236, 240, 246);
    visuals.widgets.inactive.bg_stroke.color = Color32::from_rgb(196, 205, 219);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(232, 238, 248);
    visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(226, 233, 245);
    visuals.widgets.hovered.bg_stroke.color = Color32::from_rgb(132, 155, 191);
    visuals.widgets.active.bg_fill = Color32::from_rgb(214, 226, 244);
    visuals.widgets.active.weak_bg_fill = Color32::from_rgb(206, 220, 241);
    visuals.widgets.active.bg_stroke.color = Color32::from_rgb(64, 107, 171);
    visuals.selection.bg_fill = Color32::from_rgb(86, 130, 196);
    visuals.selection.stroke.color = Color32::from_rgb(38, 84, 148);
    visuals.hyperlink_color = Color32::from_rgb(28, 92, 176);
    visuals.faint_bg_color = Color32::from_rgb(244, 247, 251);
    visuals.code_bg_color = Color32::from_rgb(243, 246, 250);

    visuals
}

fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    install_cjk_fallback(&mut fonts);
    ctx.set_fonts(fonts);
}

fn install_cjk_fallback(fonts: &mut FontDefinitions) {
    for path in windows_cjk_font_candidates() {
        if try_register_font(fonts, "windows_cjk_fallback", path) {
            return;
        }
    }
}

fn try_register_font(fonts: &mut FontDefinitions, font_name: &str, path: PathBuf) -> bool {
    let Ok(bytes) = fs::read(&path) else {
        return false;
    };

    fonts
        .font_data
        .insert(font_name.to_owned(), FontData::from_owned(bytes).into());

    for family in [FontFamily::Proportional, FontFamily::Monospace] {
        if let Some(entries) = fonts.families.get_mut(&family) {
            if !entries.iter().any(|entry| entry == font_name) {
                // Keep egui's default Latin fonts first and use the Windows font as CJK fallback.
                entries.push(font_name.to_owned());
            }
        }
    }
    true
}

fn windows_cjk_font_candidates() -> impl Iterator<Item = PathBuf> {
    let windows_dir = env::var_os("WINDIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\Windows"));
    let fonts_dir = windows_dir.join("Fonts");

    [
        "simhei.ttf",
        "simsunb.ttf",
        "msyh.ttc",
        "simsun.ttc",
        "SimsunExtG.ttf",
    ]
    .into_iter()
    .map(move |file| fonts_dir.join(file))
}

pub struct Colors;

impl Colors {
    pub fn accent(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(100, 180, 255)
        } else {
            Color32::from_rgb(20, 95, 170)
        }
    }

    pub fn success(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(100, 200, 100)
        } else {
            Color32::from_rgb(25, 140, 70)
        }
    }

    pub fn warning(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(255, 180, 100)
        } else {
            Color32::from_rgb(190, 110, 20)
        }
    }

    pub fn error(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(255, 100, 100)
        } else {
            Color32::from_rgb(190, 45, 55)
        }
    }

    pub fn running(dark_mode: bool) -> Color32 {
        Self::success(dark_mode)
    }

    pub fn stopped(dark_mode: bool) -> Color32 {
        Self::warning(dark_mode)
    }

    pub fn signed(dark_mode: bool) -> Color32 {
        Self::success(dark_mode)
    }

    pub fn unsigned(dark_mode: bool) -> Color32 {
        Self::warning(dark_mode)
    }
}
