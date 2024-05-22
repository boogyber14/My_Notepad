use eframe::egui::{self, CentralPanel, TextEdit};

struct MyApp {
    text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add(TextEdit::multiline(&mut self.text).desired_rows(20));
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My Notepad",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
