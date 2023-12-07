use eframe::egui;
use database::{db_instance, KwissDatabase};
use ui::MainUi;

mod database;
mod ui;


fn main() {
    let app = App {
        database: db_instance(),
        gui: MainUi::new(),
    };
    app.run();
}

struct App {
    database: KwissDatabase,
    gui: MainUi,
}

impl App {
    pub fn run(&self) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(320.0, 240.0)),
            ..Default::default()
        };
        let words = self.database.get_entries("words").unwrap();t
        eframe::run_native(
            "CrabKwiss",
            options,
            Box::new(|_cc| MainUi::new(words)),
        )
    }
}


