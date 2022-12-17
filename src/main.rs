use eframe::egui;

mod gui;
mod decklist;

fn main() {
    // Run GUI
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "egui test run",
        options,
        Box::new(|_cc| Box::new(gui::MyApp::default())),
    );
}
