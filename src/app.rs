use eframe::egui;
use eframe::egui::CtxRef;
use eframe::epi;
use eframe::epi::Frame;

#[derive(Default)]
pub struct App;

impl epi::App for App {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        let message = "Hello, World";
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(message);
            let panic = ui.button("Panic");
            if panic.clicked() {
                panic!("{}", message)
            }
        });
    }

    fn name(&self) -> &str {
        "SQL Practice 5"
    }
}
