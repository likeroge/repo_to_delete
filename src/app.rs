use std::sync::Arc;

use eframe::{App, egui::CentralPanel};
use sqlx::SqlitePool;

use crate::{
    components::navig_bar::NavigBar, models::flight_dto::FlightDTO, repositories::flights::Flight,
    scenes::Scene,
};

#[derive(Default, Debug)]
pub struct AppData {
    pub flights: Vec<Flight>,
    pub new_flight_form: FlightDTO,
}

pub struct Appl {
    pub current_scene: Scene,
    pub pool: Arc<SqlitePool>,
    pub app_data: AppData,
}

impl Appl {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self {
            current_scene: Scene::Home,
            pool,
            app_data: AppData::default(),
        }
    }
}

impl App for Appl {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let nav_bar = {
                let scene = &mut self.current_scene;
                NavigBar(scene)
            };
            ui.add(nav_bar);
            let pool = self.pool.clone();

            // handle.spawn_blocking(self.current_scene.render(ui, pool));
            // handle.block_on(self.current_scene.render(ui, pool));

            self.current_scene.render(ui, pool, &mut self.app_data);
        });
    }
}
