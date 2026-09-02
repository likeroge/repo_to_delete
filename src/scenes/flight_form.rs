use std::sync::mpsc::{Receiver, Sender};

use eframe::egui::{self};

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
    err_tx: &mut Sender<String>,
    err_rx: &mut Receiver<String>,
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

    let add_flight_button = style::nav_button("ADD FLIGHT");
    ui.label(current_err.to_string());

    if ui.add(add_flight_button).clicked() {
        // let err_tx = err_tx.clone();
        match validate_flight_dto(new_flight) {
            Ok(()) => {
                let flight_data = new_flight.clone();
                let err_tx = err_tx.clone();
                println!("inside tokio");
                tokio::spawn(async move {
                    match repo.create(&flight_data).await {
                        Ok(flight_in_db) => {
                            println!("{:?}", flight_in_db);
                            err_tx.send("Success".to_string()).unwrap();
                        }
                        Err(e) => {
                            println!("{}", e);
                            err_tx.send(format!("Error saving flight: {}", e)).unwrap();
                        }
                    }
                });
            }
            Err(e) => {
                let _: () = err_tx.send(format!("Error saving flight: {}", e)).unwrap();
            }
        };

        *current_err = err_rx.recv().unwrap();
    }
}

fn validate_flight_dto(flight: &FlightDTO) -> Result<(), &'static str> {
    if flight.arr.is_empty() {
        return Err("Arrival field is required");
    }
    if flight.dof.is_empty() {
        return Err("DOF field is required");
    }
    if flight.dep.is_empty() {
        return Err("Departure field is required");
    }
    if flight.pyld.is_empty() {
        return Err("Payload field is required");
    }
    if flight.flight_number.is_empty() {
        return Err("Flight Number field is required");
    }
    if flight.tail.is_empty() {
        return Err("Tail field is required");
    }
    Ok(())
}
