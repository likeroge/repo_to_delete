use eframe::egui::{self, Color32, CornerRadius, RichText, Stroke};

pub const BG: Color32 = Color32::from_rgb(8, 12, 20);
pub const PANEL: Color32 = Color32::from_rgb(13, 18, 30);
pub const PRIMARY: Color32 = Color32::from_rgb(90, 170, 255);
pub const BRIGHT: Color32 = Color32::from_rgb(160, 205, 255);
pub const MUTED: Color32 = Color32::from_rgb(70, 120, 180);
pub const ERROR: Color32 = Color32::from_rgb(255, 110, 110);

pub fn apply_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    visuals.dark_mode = true;
    visuals.override_text_color = Some(PRIMARY);
    visuals.hyperlink_color = PRIMARY;

    visuals.panel_fill = PANEL;
    visuals.window_fill = PANEL;
    visuals.extreme_bg_color = BG;
    visuals.faint_bg_color = Color32::from_rgb(10, 15, 26);
    visuals.code_bg_color = Color32::from_rgb(24, 34, 54);
    visuals.window_stroke = Stroke::new(1.0, Color32::from_rgb(36, 58, 92));

    let noninteractive = &mut visuals.widgets.noninteractive;
    noninteractive.fg_stroke.color = PRIMARY;
    noninteractive.bg_fill = PANEL;
    noninteractive.weak_bg_fill = PANEL;
    noninteractive.bg_stroke.color = MUTED;

    let inactive = &mut visuals.widgets.inactive;
    inactive.fg_stroke.color = PRIMARY;
    inactive.weak_bg_fill = Color32::from_rgb(22, 34, 54);
    inactive.bg_fill = Color32::from_rgb(22, 34, 54);
    inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(50, 100, 170));
    inactive.corner_radius = CornerRadius::same(6);

    let hovered = &mut visuals.widgets.hovered;
    hovered.fg_stroke.color = BRIGHT;
    hovered.weak_bg_fill = Color32::from_rgb(30, 48, 78);
    hovered.bg_fill = Color32::from_rgb(30, 48, 78);
    hovered.bg_stroke = Stroke::new(1.0, PRIMARY);
    hovered.corner_radius = CornerRadius::same(8);

    let active = &mut visuals.widgets.active;
    active.fg_stroke.color = BRIGHT;
    active.weak_bg_fill = Color32::from_rgb(38, 60, 96);
    active.bg_fill = Color32::from_rgb(38, 60, 96);
    active.bg_stroke = Stroke::new(1.0, BRIGHT);
    active.corner_radius = CornerRadius::same(8);

    let open = &mut visuals.widgets.open;
    open.fg_stroke.color = BRIGHT;
    open.weak_bg_fill = Color32::from_rgb(28, 44, 72);
    open.bg_fill = Color32::from_rgb(28, 44, 72);

    ctx.set_visuals(visuals);
}

pub fn heading(text: impl Into<String>) -> RichText {
    RichText::new(text).color(PRIMARY).size(30.0).strong()
}

pub fn label(text: impl Into<String>) -> RichText {
    RichText::new(text).color(PRIMARY).size(16.0)
}

pub fn nav_button(text: &str) -> egui::Button<'static> {
    egui::Button::new(RichText::new(text).color(PRIMARY).size(15.0).strong())
        .corner_radius(8)
        .min_size(egui::vec2(96.0, 34.0))
}
