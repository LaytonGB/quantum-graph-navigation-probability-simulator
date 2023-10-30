#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// TODO move right-click context menu stuff into `context_menu.rs`
// TODO right-click -> Add Node by coordinates
// TODO clean all imports/exports
// TODO add "add graph" setting under canvas (place graph without clearing
// canvas)
// TODO add a right-panel for setting simulation mode (linear / quantum)
// TODO add code for a linear random walk

// GraphNode lookup problem
// - likely requires HashMap to lookup placed nodes
// - HashMap requires Eq and Hash traits
// - floats do not have these traits
// - - problem solved in this crate by implementing those traits manually
// - - C:\Users\layto\.cargo\registry\src\index.crates.io-6f17d22bba15001f\
//   epaint-0.23.0\src\util\ordered_float.rs
// - for now, will be searching coords from vec

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([400.0, 300.0].into()),
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "Eframe GUI Test",
        native_options,
        Box::new(|cc| Box::new(eframe_gui_test::EframeApp::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(eframe_gui_test::EframeApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
