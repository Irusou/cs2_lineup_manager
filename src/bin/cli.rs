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

#[derive(Default)]
struct App {}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Hello world");
            ui.columns(3, |cols| {
                cols[0].label(
                    Lineup::new(
                        String::from("Window insta #1"),
                        Map::Mirage,
                        UtilityKind::Smoke,
                        ThrowType::WJumpthrow,
                    )
                    .to_string(),
                );
                cols[1].label(
                    Lineup::new(
                        String::from("Pop flash short"),
                        Map::Dust2,
                        UtilityKind::Flash,
                        ThrowType::Jumpthrow,
                    )
                    .to_string(),
                );
                cols[2].label(
                    Lineup::new(
                        String::from("Rockets moly"),
                        Map::Cache,
                        UtilityKind::Molotov,
                        ThrowType::Stand,
                    )
                    .to_string(),
                );
            })
        });
    }
}
