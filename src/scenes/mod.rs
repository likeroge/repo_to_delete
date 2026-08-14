use std::sync::Arc;

use eframe::egui;
use sqlx::SqlitePool;

pub mod about;
// pub mod flights;
// pub mod flights_table;
pub mod flights;
pub mod home;

#[derive(Default)]
pub enum Scene {
    #[default]
    Home,
    About,
    // FlightsScreen(flights::FlightsForm),
    FlightsScreen,
}

impl Scene {
    pub fn render(&mut self, ui: &mut egui::Ui, pool: Arc<SqlitePool>) {
        let next = match self {
            Scene::Home => {
                home::render_home(ui);
                None
            }
            Scene::About => {
                about::render_about(ui);
                None
            }
            Scene::FlightsScreen => {
                // tokio::spawn(flights::render_flights(ui, pool));
                flights::render_flights(ui, pool);
                None
            } // Scene::Flights(form) => {
              //     if flights::render_form(ui, form, pool) {
              //         Some(Scene::FlightsTable(flights_table::FlightsTable::default()))
              //     } else {
              //         None
              //     }
              // }
              // Scene::FlightsTable(table) => {
              //     flights_table::render_table(ui, table, pool);
              //     None
              // }
        };

        if let Some(next) = next {
            *self = next;
        }
    }
}
