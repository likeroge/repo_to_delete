use std::sync::{
    Arc,
    mpsc::{self, Receiver, Sender},
};

use eframe::egui;
use sqlx::SqlitePool;

use crate::{
    app::{self, AppData},
    repositories::{
        CrudRepoTrait,
        flights::{Flight, FlightsRepository},
    },
};

pub mod about;

pub mod flight_form;
pub mod flights;
pub mod home;
pub mod users;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum FlightScreenStatus {
    NotLoaded,
    Loaded,
}

#[derive(Default, PartialEq, PartialOrd)]
pub enum Scene {
    #[default]
    Home,
    About,
    // FlightsScreen(flights::FlightsForm),
    FlightsScreen(FlightScreenStatus),
    FlightFormScreen,
    UsersScreen,
}

impl Scene {
    pub fn render(&mut self, ui: &mut egui::Ui, pool: Arc<SqlitePool>, app_data: &mut AppData) {
        let next = match self {
            Scene::Home => {
                home::render_home(ui);
                None
            }
            Scene::About => {
                about::render_about(ui);
                None
            }
            Scene::FlightsScreen(status) => {
                let repo = FlightsRepository::new(pool.clone());
                let handle = tokio::runtime::Handle::current();
                let (tx, rx) = mpsc::channel::<Vec<Flight>>();

                match *status {
                    FlightScreenStatus::NotLoaded => {
                        println!("NOT LOADED");
                        handle.spawn(async move {
                            let flights_from_db = repo.get_all().await.unwrap();
                            println!("Tokio spawn");

                            tx.send(flights_from_db).expect("Cant send");
                        });

                        let res = rx.recv().expect("Cant received");
                        *status = FlightScreenStatus::Loaded;
                        app_data.flights = res;
                    }
                    FlightScreenStatus::Loaded => {
                        ui.label("Loaded data scene");

                        if !app_data.flights.is_empty() {
                            flights::render_flights(ui, &app_data.flights);
                        }
                    }
                }

                None
            }
            Scene::UsersScreen => {
                users::render_users(ui);
                None
            }
            Scene::FlightFormScreen => {
                flight_form::render_flight_form(ui);
                None
            }
        };

        if let Some(next) = next {
            *self = next;
        }
    }
}
