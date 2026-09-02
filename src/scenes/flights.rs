use eframe::egui::{self, Label, ScrollArea, Ui};

use crate::{repositories::flights::Flight, style};

pub fn render_flights(ui: &mut Ui, flights: &Vec<Flight>) {
    ui.add_space(24.0);
    ui.vertical_centered(|ui| {
        ui.add(egui::Label::new(style::heading("All flights screen")));
    });

    ui.horizontal(|ui| {
        ui.strong("FLIGHT_NUMBER");
        ui.separator();
        ui.strong("DEPARTURE");
        ui.separator();
        ui.strong("ARRIVAL");

        // ui.label(flights.len().to_string());

        // ui.label(&flights[0].flight_number)
    });

    ScrollArea::vertical().show(ui, |ui| {
        for flight in flights {
            ui.horizontal(|ui| {
                ui.label(&flight.flight_number);
                ui.separator();
                ui.label(&flight.dep);
                ui.separator();
                ui.label(&flight.arr);
            });
        }
    });
}
