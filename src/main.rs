use eframe::NativeOptions;

use app::App;

mod app;
mod logger;

fn main() {
    log_panics::init();
    let _handle = logger::init().unwrap();

    let options = NativeOptions::default();
    eframe::run_native(Box::new(App::default()), options);
}
