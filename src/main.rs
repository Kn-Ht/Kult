// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate uom;

mod app;
mod apps;

use app::App;

#[cfg(not(target_family = "wasm"))]
fn main() {
    let native_options = eframe::NativeOptions {
        centered: true,
        default_theme: eframe::Theme::Dark,
        follow_system_theme: true,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        ..Default::default()
    };
    eframe::run_native(
        App::TITLE,
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .unwrap();
}

#[cfg(target_family = "wasm")]
fn main() {
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "glcanvas", // hardcode it
                web_options,
                Box::new(|cc| Box::new(App::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
