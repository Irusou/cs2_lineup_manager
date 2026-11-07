use cs2_lineup_manager::domain::{
    lineup::Lineup,
    types::{Map, ThrowType, UtilityKind},
};
use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "CS2 Lineup Manager",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}

struct App {
    _filter: String,
    lineups: Vec<Lineup>,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            _filter: String::new(),
            lineups: vec![
                Lineup::new(
                    String::from("Window insta #1"),
                    Map::Mirage,
                    UtilityKind::Smoke,
                    ThrowType::WJumpthrow,
                ),
                Lineup::new(
                    String::from("Pop flash short"),
                    Map::Dust2,
                    UtilityKind::Flash,
                    ThrowType::Jumpthrow,
                ),
                Lineup::new(
                    String::from("Rockets moly"),
                    Map::Cache,
                    UtilityKind::Molotov,
                    ThrowType::Stand,
                ),
            ],
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Lineups");
            if !self.lineups.is_empty() {
                ui.columns(self.lineups.len(), |cols| {
                    for (i, lineup) in self.lineups.iter().enumerate() {
                        cols[i].label(lineup.to_string());
                    }
                })
            }
        });
    }
}
