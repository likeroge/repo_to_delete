use eframe::egui::{self, Button};

use crate::{models::flight_dto::FlightDTO, style};

pub fn render_flight_form(ui: &mut egui::Ui, new_flight: &mut FlightDTO) {
    ui.add_space(24.0);
    ui.vertical_centered(|ui| {
        ui.add(egui::Label::new(style::heading("Flight form screen")));
    });

    ui.label("Departure");
    ui.text_edit_singleline(&mut new_flight.dep);
    ui.label("Arrival");
    ui.text_edit_singleline(&mut new_flight.arr);
    ui.label("DOF");
    ui.text_edit_singleline(&mut new_flight.dof);
    ui.label("Flight Number");
    ui.text_edit_singleline(&mut new_flight.flight_number);
    ui.label("Tail");
    ui.text_edit_singleline(&mut new_flight.tail);
    ui.label("Payload");
    ui.text_edit_singleline(&mut new_flight.pyld.to_string());

    let add_flight = style::nav_button("ADD FLIGHT");

    if ui.add(add_flight).clicked() {
        println!("{:?}", new_flight);
    }
}
