use eframe::egui;
use eframe::egui::CtxRef;
use eframe::epi;
use eframe::epi::{Frame, Storage};

#[derive(Default)]
pub struct App {
    state: Option<State>,
}

impl epi::App for App {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        let state = self.state.as_ref().unwrap();
        match state {
            State::Empty => {
                let message = "Hello, World";
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label(message);
                    let panic = ui.button("Panic");
                    if panic.clicked() {
                        panic!("{}", message)
                    }
                });
            }
        }
    }

    fn setup(&mut self, _ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {
        self.state = Some(State::Empty);
        log::debug!("Setup completed");
    }

    fn on_exit(&mut self) {
        log::debug!("Shutting down");
    }

    fn name(&self) -> &str {
        "SQL Practice 5"
    }
}

pub enum State {
    Empty,
}
