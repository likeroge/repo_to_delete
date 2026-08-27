#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{
    Arc,
    mpsc::{self, Receiver, Sender},
};

use eframe::{NativeOptions, egui::vec2, run_native};

use crate::{app::Appl, db::init_db};

mod app;
mod components;
mod db;
mod models;
mod repositories;
mod scenes;
mod study;
mod style;

// pub struct Aaa {
// ch: (Sender<String>, Receiver<String>),
// }

// fn main() {
// let (tx, rx) = mpsc::channel();
// let ap = Aaa { ch: (tx, rx) };
// study::study_fn_main();
// }
//
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
