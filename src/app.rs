use std::sync::Arc;

use eframe::{App, egui::CentralPanel};
use sqlx::SqlitePool;

use crate::{components::navig_bar::NavigBar, scenes::Scene};

pub struct Appl {
    current_scene: Scene,
    pool: Arc<SqlitePool>,
}

impl Appl {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self {
            current_scene: Scene::Home,
            pool,
        }
    }
}

impl App for Appl {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let nav_bar = NavigBar::new(&mut self.current_scene);
            ui.add(nav_bar);
            let pool = self.pool.clone();

            // handle.spawn_blocking(self.current_scene.render(ui, pool));
            // handle.block_on(self.current_scene.render(ui, pool));

            self.current_scene.render(ui, pool);
        });
    }
}
