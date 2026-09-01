use eframe::egui::{self, Color32, RichText};

use crate::{
    models::flight_dto::FlightDTO,
    repositories::{CrudRepoTrait, flights::FlightsRepository},
    style,
};

pub fn render_flight_form(
    ui: &mut egui::Ui,
    new_flight: &mut FlightDTO,
    current_err: &mut String,
    repo: FlightsRepository,
) {
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
    ui.text_edit_singleline(&mut new_flight.pyld);

    let add_flight = style::nav_button("ADD FLIGHT");

    if ui.add(add_flight).clicked() {
        *current_err = String::new();
        if new_flight.arr.is_empty()
            || new_flight.dof.is_empty()
            || new_flight.dep.is_empty()
            || new_flight.pyld.is_empty()
            || new_flight.flight_number.is_empty()
            || new_flight.tail.is_empty()
        {
            println!("Wrong data");
            println!("{}", *current_err);

            if current_err.is_empty() {
                *current_err = String::from("Please fill all fields");
            }

            // repo.create(new_flight)
        }

        let f = new_flight.clone();
        let handle = tokio::runtime::Handle::current();
        handle.spawn(async move {
            let flight_in_db = repo.create(&f).await.unwrap();
            println!("{:?}", flight_in_db);
        });
    }

    let error_text = RichText::new(&*current_err).color(Color32::RED).size(22.0);
    ui.label(error_text);
}
