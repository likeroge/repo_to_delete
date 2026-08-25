use eframe::egui::{self, Grid, RichText};

use crate::style;

pub fn render_users(ui: &mut egui::Ui) {
    ui.add_space(24.0);
    ui.vertical_centered(|ui| {
        ui.add(egui::Label::new(style::heading("Users screen")));
        ui.label("Scene is prepared for rendering the users table.");
    });

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(12.0);

    Grid::new("users_table_preview")
        .num_columns(4)
        .striped(true)
        .spacing([24.0, 8.0])
        .show(ui, |ui| {
            ui.label(RichText::new("ID").strong());
            ui.label(RichText::new("Name").strong());
            ui.label(RichText::new("Email").strong());
            ui.label(RichText::new("Role").strong());
            ui.end_row();

            ui.label("-");
            ui.label("No users loaded");
            ui.label("-");
            ui.label("-");
            ui.end_row();
        });
}
