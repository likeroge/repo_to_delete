#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use eframe::{NativeOptions, egui::vec2, run_native};

use crate::{app::Appl, db::init_db};
mod app;
mod components;
mod db;
mod repositories;
mod scenes;
mod style;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = init_db().await.expect("Failed to init DB");
    let mut options = NativeOptions::default();
    options.viewport = options
        .viewport
        .with_inner_size(vec2(600.0, 600.0))
        .with_maximize_button(false)
        .with_resizable(false);

    let arc_pool = Arc::new(pool);
    let appl = Appl::new(arc_pool);

    run_native(
        "Desk app",
        options,
        Box::new(|cc| {
            style::apply_theme(&cc.egui_ctx);
            // Ok(Box::new(Appl::default()))
            Ok(Box::new(appl))
        }),
    )
    .expect("Cant run app");
}
