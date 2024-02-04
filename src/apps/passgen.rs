use egui::{vec2, Button, Color32, RichText, SelectableLabel, Slider, TextBuffer, TextEdit};

pub struct Passgen {
    pub shown: bool,
    pub password: String,
    pub title: &'static str,
    generator: passwords::PasswordGenerator,
    symbols_str: String,
    strength_test_str: String,
    analyzed_password: passwords::AnalyzedPassword,
    scored_password: u32,
}

impl Passgen {
    pub const MAX_LEN: usize = 50;
    const DEFAULT_SYMBOLS: &'static str = "@%()+-";

    pub fn new() -> Self {
        let generator = passwords::PasswordGenerator::default()
            .symbols(Self::DEFAULT_SYMBOLS)
            .length(15);

        // So we never have to allocate more capacity
        let mut password = String::with_capacity(Self::MAX_LEN);
        password.replace_with(&generator.generate_one().unwrap_or_default());

        let dummy_password = "abc123";
        let analyzed_password = passwords::analyzer::analyze(dummy_password);

        Self {
            shown: false,
            title: "Password Generator 🔐",
            scored_password: passwords::scorer::score(&analyzed_password) as u32,
            analyzed_password,
            password,
            generator,
            symbols_str: Self::DEFAULT_SYMBOLS.to_string(),
            strength_test_str: dummy_password.to_string(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut window_open = self.shown;

        egui::Window::new(self.title)
            .open(&mut window_open)
            .default_size(vec2(236.3, 89.0))
            .show(ctx, |ui| {
                let avail_size = ui.available_size_before_wrap();
                let mut input_changed = false;

                // Length slider
                ui.style_mut().spacing.slider_width = avail_size.x - 100.0;
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Length:").strong());
                    if ui
                        .add(Slider::new(&mut self.generator.length, 0..=Self::MAX_LEN))
                        .changed()
                    {
                        input_changed = true;
                    }
                });

                // Symbols input
                ui.horizontal(|ui| {
                    if ui
                        .checkbox(&mut self.generator.symbols_enabled, "Symbols")
                        .changed()
                        || ui
                            .add_enabled(
                                self.generator.symbols_enabled,
                                TextEdit::singleline(&mut self.symbols_str)
                                    .char_limit(27)
                                    .desired_width(avail_size.x - 100.0)
                                    .hint_text("Example: @#$%"),
                            )
                            .changed()
                    {
                        input_changed = true;
                    }
                    if ui.button("↺").clicked() {
                        //self.symbols_str.replace_with(Self::DEFAULT_SYMBOLS);
                        input_changed = true;
                    }
                });

                ui.horizontal(|ui| {
                    if ui
                        .checkbox(&mut self.generator.numbers, "Numbers")
                        .changed()
                    {
                        input_changed = true;
                    }
                    ui.add_space(avail_size.x - 112.0);
                    if ui.add(Button::new(RichText::new("copy").strong()).fill(Color32::DARK_GREEN)).clicked() {
                        ui.output_mut(|o| o.copied_text = self.password.to_string());
                    }
                });

                // Password Output
                if ui
                    .add_sized(
                        vec2(ui.available_width(), 16.0),
                        SelectableLabel::new(
                            true,
                            RichText::new(&self.password).size(16.0).strong(),
                        ),
                    )
                    .clicked()
                {
                    ui.output_mut(|o| o.copied_text = self.password.to_string());
                }

                let mut reanalyze = false;

                // regenerate if input (length, etc.) has changed
                if input_changed {
                    if let Some(ref mut s) = self.generator.symbols {
                        *s = self.symbols_str.chars().collect::<Vec<_>>();
                    }
                    self.generate_password();
                    self.strength_test_str.replace_with(&self.password);
                }

                // (Maybe collapsed) Password rater
                ui.separator();
                ui.collapsing("Password Strength Test", |ui| {
                    ui.horizontal(|ui| {
                        reanalyze = ui
                            .add(
                                TextEdit::singleline(&mut self.strength_test_str)
                                    .hint_text("Example: abc123"),
                            )
                            .changed();

                        if ui.button("use generated").clicked() {
                            self.strength_test_str.replace_with(&self.password);
                            reanalyze = true;
                        }

                        if reanalyze || input_changed {
                            self.analyzed_password =
                                passwords::analyzer::analyze(&self.strength_test_str);
                            self.scored_password =
                                passwords::scorer::score(&self.analyzed_password) as u32;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.heading(RichText::new("Rating: ").strong());
                        ui.heading(match self.scored_password {
                            s @ 0..=19 => RichText::new(format!("{s}/100 (very dangerous)"))
                                .color(Color32::DARK_RED),
                            s @ 20..=39 => {
                                RichText::new(format!("{s}/100 (dangerous)")).color(Color32::RED)
                            }
                            s @ 40..=59 => RichText::new(format!("{s}/100 (very weak)"))
                                .color(Color32::LIGHT_RED),
                            s @ 60..=79 => {
                                RichText::new(format!("{s}/100 (weak)")).color(Color32::KHAKI)
                            }
                            s @ 80..=89 => {
                                RichText::new(format!("{s}/100 (good)")).color(Color32::LIGHT_GREEN)
                            }
                            s @ 90..=94 => {
                                RichText::new(format!("{s}/100 (strong)")).color(Color32::GREEN)
                            }
                            s @ 95..=99 => RichText::new(format!("{s}/100 (very strong)"))
                                .color(Color32::GREEN),
                            _ => RichText::new("100/100 (invulnerable)").color(Color32::LIGHT_BLUE),
                        })
                    });
                });
            });
        self.shown = window_open;
    }

    fn generate_password(&mut self) {
        self.password = self.generator.generate_one().unwrap_or_default();
    }
}
