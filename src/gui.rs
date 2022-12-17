use eframe::egui;

#[derive(Default)]
pub struct MyApp {
    _allowed_to_close: bool,
    _show_confirmation_dialog: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        decklist_view(ctx, frame);
    }
}

pub fn decklist_view (context: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(context, |ui| {
        ui.columns(3, |columns| {
            for col in columns {
                for _ in 0..10 {
                    col.label("Hello Label Text");
                }
            }
        })
    });
}

