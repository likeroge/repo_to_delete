use eframe::egui;

use crate::style;

pub fn render_home(ui: &mut egui::Ui) {
    ui.add_space(24.0);
    ui.vertical_centered(|ui| {
        ui.add(egui::Label::new(style::heading("Home screen")));
    });
}
