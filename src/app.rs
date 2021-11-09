use eframe::egui;
use eframe::egui::CtxRef;
use eframe::epi;
use eframe::epi::Frame;

#[derive(Default)]
pub struct App;

impl epi::App for App {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, World");
        });
    }

    fn name(&self) -> &str {
        "SQL Practice 5"
    }
}
