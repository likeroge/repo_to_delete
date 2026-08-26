use eframe::egui::{self, Grid, Label, Ui};

use crate::{repositories::flights::Flight, style};

pub fn render_flights(ui: &mut Ui, flights: &Vec<Flight>) {
    ui.add_space(24.0);
    ui.vertical_centered(|ui| {
        ui.add(egui::Label::new(style::heading("All flights screen")));
    });

    ui.horizontal(|ui| {
        ui.strong("left");
        ui.separator();
        ui.strong("center");
        ui.separator();
        ui.strong("right");

        ui.label(flights.len().to_string());

        ui.label(&flights[0].flight_number)
    });
}
