use eframe::NativeOptions;

use app::App;

mod app;
mod logger;

fn main() {
    log_panics::init();
    let _handle = logger::init().unwrap();

    let options = NativeOptions::default();
    let app = App::default();
    eframe::run_native(Box::new(app), options);
}
