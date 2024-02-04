use egui::{vec2, Align2, Button, Color32, Label, RichText, Vec2};

use crate::apps::passgen::Passgen;
use crate::apps::unitconv::UnitConverter;

pub struct App {
    help_shown: bool,

    // apps
    passgen: Passgen,
    unitconv: UnitConverter,
}

impl App {
    pub const TITLE: &'static str = "Kult";

    // Global top bar
    const TOPBAR_HEIGHT: f32 = 27.0;
    const BTN_MIN_SIZE: Vec2 = vec2(0.0, Self::TOPBAR_HEIGHT - 3.0);

    #[allow(unused_variables)]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Setup code (runs once)

        if cfg!(target_family = "wasm") {
            cc.egui_ctx.set_pixels_per_point(2.0);
        } else {
            cc.egui_ctx.set_pixels_per_point(1.0);
        }

        Self {
            help_shown: false,
            passgen: Passgen::new(),
            unitconv: UnitConverter::new(),
        }
    }
    // UI drawing functions
    fn draw_top_bar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("TopBar")
            .exact_height(Self::TOPBAR_HEIGHT)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Logo
                    ui.heading(RichText::new(Self::TITLE).color(Color32::WHITE).strong());

                    if ui
                        .small_button(RichText::new("?").strong().color(Color32::GREEN))
                        .clicked()
                    {
                        self.help_shown = !self.help_shown;
                    }

                    if ui
                        .add(Button::new(self.passgen.title).min_size(Self::BTN_MIN_SIZE))
                        .clicked()
                    {
                        self.passgen.shown = !self.passgen.shown;
                    }

                    if ui
                        .add(Button::new(self.unitconv.title).min_size(Self::BTN_MIN_SIZE))
                        .clicked()
                    {
                        self.unitconv.shown = !self.unitconv.shown;
                    }
                });
            });
    }

    fn draw_windows(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.passgen.show(ctx, frame);
        self.unitconv.show(ctx, frame);

        egui::Window::new("Help")
            .open(&mut self.help_shown)
            .pivot(Align2::CENTER_CENTER)
            .show(ctx, |ui| {
                ui.label("Help here.");
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // This is where the main Gui code goes!
        self.draw_top_bar(ctx, frame);
        self.draw_windows(ctx, frame);
    }
}
