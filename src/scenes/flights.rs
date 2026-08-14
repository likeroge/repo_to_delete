use std::{os::windows::thread, sync::Arc};

use eframe::egui::{self, Grid, Label, Ui};
use sqlx::SqlitePool;

use crate::{
    repositories::{self, CrudRepoTrait, flights::FlightsRepository},
    style,
};

pub fn render_flights(ui: &mut Ui, pool: Arc<SqlitePool>) {
    ui.add_space(24.0);
    ui.vertical_centered(|ui| {
        ui.add(egui::Label::new(style::heading("All flights screen")));
    });

    ui.horizontal(|ui| {
        ui.strong("left");
        ui.separator();
        ui.strong("center");
        ui.separator();
        ui.strong("right");
    });
    let repo = FlightsRepository::new(pool);
    draw_flights_table(repo, ui);

    // let handle = tokio::runtime::Handle::current();
    // handle.spawn(draw_flights_table(repo, ui));

    // draw_flights_table(repo, ui).await;

    // let rt = tokio::runtime::Runtime::new().unwrap();
    // rt.block_on(draw_flights_table(repo, ui));

    // draw_flights_table(repo, ui);

    // match repo.get_all().await {
    //     Ok(data) => {
    //         for i in &data {
    //             Grid::new("MyNewTable")
    //                 .num_columns(3)
    //                 .spacing([20.0, 6.0])
    //                 .striped(true)
    //                 .show(ui, |ui| {
    //                     ui.add_sized([30.0, 30.0], Label::new(i.dep.to_string()));
    //                     ui.add_sized([30.0, 30.0], Label::new(i.arr.to_string()));
    //                     ui.add_sized([30.0, 30.0], Label::new(i.flight_number.to_string()));
    //                 });
    //         }
    //     }
    //     Err(_) => {
    //         ui.label("NO DATA");
    //     }
    // }
}

async fn draw_flights_table(repo: FlightsRepository, ui: &mut Ui) {
    match repo.get_all().await {
        Ok(data) => {
            for i in &data {
                Grid::new("MyNewTable")
                    .num_columns(3)
                    .spacing([20.0, 6.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.add_sized([30.0, 30.0], Label::new(i.dep.to_string()));
                        ui.add_sized([30.0, 30.0], Label::new(i.arr.to_string()));
                        ui.add_sized([30.0, 30.0], Label::new(i.flight_number.to_string()));
                    });
            }
        }
        Err(_) => {
            ui.label("NO DATA");
        }
    }
}
