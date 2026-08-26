use eframe::egui::Widget;

use crate::{scenes::Scene, style};

pub struct NavigBar<'a>(pub &'a mut Scene);

impl<'a> NavigBar<'a> {
    // pub fn new(scene: &'a mut Scene) -> Self {
    // Self(scene)
    // }
}

impl<'a> Widget for NavigBar<'a> {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        ui.horizontal(|ui| {
            if ui.add(style::nav_button("HOME")).clicked() {
                *self.0 = Scene::Home;
            }

            if ui.add(style::nav_button("FLIGHTS")).clicked() {
                *self.0 = Scene::FlightsScreen(crate::scenes::FlightScreenStatus::NotLoaded);
            }

            if ui.add(style::nav_button("USERS")).clicked() {
                *self.0 = Scene::UsersScreen;
            }

            if ui.add(style::nav_button("ABOUT")).clicked() {
                *self.0 = Scene::About;
            }

            if ui.add(style::nav_button("EXIT")).clicked() {
                ui.ctx()
                    .send_viewport_cmd(eframe::egui::ViewportCommand::Close);
            }

            if *self.0 == Scene::FlightsScreen(crate::scenes::FlightScreenStatus::Loaded) {
                if ui.add(style::nav_button("Flight_FORM")).clicked() {
                    *self.0 = Scene::FlightFormScreen;
                }
            }
        })
        .response
    }
}
