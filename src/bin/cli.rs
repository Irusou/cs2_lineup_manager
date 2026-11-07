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
    _selected: Option<Lineup>,
    _filter: String,
    lineups: Vec<Lineup>,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            _selected: None,
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
        egui::Panel::top("header").show(ui, |ui| ui.label("Lineups"));
        egui::Panel::left("side_bar").show(ui, |ui| {
            ui.columns(1, |col| {
                if !self.lineups.is_empty() {
                    for (_, lineup) in self.lineups.iter().enumerate() {
                        col[0].label(lineup.to_string());
                    }
                } else {
                    col[0].label("No lineups available yet.");
                }
            })
        });
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("this is the central panel");
        });
    }
}
