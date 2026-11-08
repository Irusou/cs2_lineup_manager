use cs2_lineup_manager::domain::{
    lineup::Lineup,
    types::{Map, ThrowType, UtilityKind},
};
use eframe::egui;
use uuid::Uuid;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "CS2 Lineup Manager",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}

struct SideBar {
    min: f32,
    max: f32,
}

struct App {
    selected: Option<Uuid>,
    _filter: String,
    lineups: Vec<Lineup>,
    sidebar: SideBar,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            sidebar: SideBar {
                min: 150.0,
                max: 200.0,
            },
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
        egui::Panel::left("side_bar")
            .min_size(self.sidebar.min)
            .max_size(self.sidebar.max)
            .show(ui, |ui| {
                if !self.lineups.is_empty() {
                    for lineup in self.lineups.iter() {
                        let selected = self.selected == Some(lineup.id());
                        if ui.selectable_label(selected, lineup.as_label()).clicked() {
                            println!("{:?}", lineup); // remove on last compilation
                            self.selected = Some(lineup.id());
                        }
                    }
                } else {
                    ui.label("No lineups available yet.");
                }
            });

        // main panel where a selected lineup will be shown along with the options to create and
        // delete a lineup
        // if a lineup is in display, its fields should be editable
        egui::CentralPanel::default().show(ui, |ui| match self.selected {
            Some(id) => match self.lineups.iter().find(|lineup| lineup.id() == id) {
                Some(lineup) => ui.label(lineup.to_string()),
                None => ui.label("Lineup not found"),
            },
            None => ui.label("Select a lineup"),
        });
    }
}
