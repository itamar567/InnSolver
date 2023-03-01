use crate::gui::app::App;

// Native
#[cfg(not(target_arch = "wasm32"))]
pub fn start() {
    let options = eframe::NativeOptions {
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        ..eframe::NativeOptions::default()
    };

    eframe::run_native("InnSolver", options, Box::new(|cc| Box::new(App::new(cc))))
        .expect("Couldn't start app");
}

// Web
#[cfg(target_arch = "wasm32")]
pub fn start() {
    // Log panics using `console.error`
    console_error_panic_hook::set_once();

    // Redirect tracing to `console.log`
    tracing_wasm::set_as_global_default();

    let options = eframe::WebOptions {
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        ..eframe::WebOptions::default()
    };

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web("container", options, Box::new(|cc| Box::new(App::new(cc))))
            .await
            .expect("Failed to start eframe");
    });
}
