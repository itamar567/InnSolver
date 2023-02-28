use crate::gui::app::App;

pub fn start() {
    let options = eframe::NativeOptions::default();

    eframe::run_native("InnSolver", options, Box::new(|cc| Box::new(App::new(cc))))
        .expect("Couldn't start app");
}
