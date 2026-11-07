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
    selected: Option<usize>,
    _filter: String,
    lineups: Vec<Lineup>,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            selected: None,
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
        //creates the header of the app
        egui::Panel::top("header").show(ui, |ui| {
            ui.label("Lineups");
            if !self.lineups.is_empty() {
                // add a search bar
                // for now create a label to where the search bar will be
                ui.label("this will be an input to filter the lineups");
            }
        });

        // creates the sidebar where the lineups will be displayed
        egui::Panel::left("side_bar").show(ui, |ui| {
            if !self.lineups.is_empty() {
                for (i, lineup) in self.lineups.iter().enumerate() {
                    if ui.selectable_label(true, lineup.to_string()).clicked() {
                        self.selected = Option::Some(i);
                    };
                }
            } else {
                ui.label("No lineups available yet.");
            }
        });

        // main panel where a selected lineup will be shown along with the options to create and
        // delete a lineup
        // if a lineup is in display, its fields should be editable
        egui::CentralPanel::default().show(ui, |ui| {
            if self.selected.is_some() {
                match self.selected {
                    Some(index) => {
                        ui.label(self.lineups[index].to_string());
                    }
                    None => {}
                }
            }
        });
    }
}
