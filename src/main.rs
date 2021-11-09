use eframe::NativeOptions;
use app::App;

mod app;

fn main() {
    let options = NativeOptions::default();
    eframe::run_native(Box::new(App::default()), options);
}
