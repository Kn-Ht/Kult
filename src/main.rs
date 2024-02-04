// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod apps;
mod app;

use app::App;

fn main() {
    let native_options = eframe::NativeOptions {
        centered: true,
        default_theme: eframe::Theme::Dark,
        follow_system_theme: true,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        ..Default::default()
    };
    eframe::run_native(App::TITLE, native_options, Box::new(|cc| Box::new(App::new(cc)))).unwrap();
}