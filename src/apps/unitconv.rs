use egui::Window;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum LengthUnit {
    Millimeter,
    Centimeter,
    Decimeter,
    Meter,
    Decameter,
    Hectometer,
    Kilometer,
    Inch,
    Foot,
    Yard,
    Mile,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum TimeUnit {
    Nanoseconds,
    Milliseconds,
    Deciseconds,
    Seconds,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    Decade,
}

struct Length(LengthUnit, f64);
impl Length {
    pub fn convert(self, to: LengthUnit) -> f64 {
        if self.0 == to {
            return self.1
        }
        let factor: f64 = match self.0 {
            LengthUnit::Millimeter => match to {
                LengthUnit::Centimeter => 0.1,
                LengthUnit::Decimeter => 0.01,
                LengthUnit::Meter => 0.001,
                
            }
            _ => 0.0
        };
        self.1 * factor
    }
}

struct Area(LengthUnit, f64);
struct Time(TimeUnit, f64);
struct Velocity {
    distance: LengthUnit,
    time: TimeUnit,
    value: f64
}

enum Unit {
    Length(Length),
    Area(Length),
    Time(Time),
    Velocity(Velocity),
}


///////////////////////////////////////////////////////////////////////////////////////////
pub struct UnitConverter {
    pub title: &'static str,
    pub shown: bool,
    from: Data,
    to: Data,
}

impl UnitConverter {
    pub const fn new() -> Self {
        Self {
            title: "Unit Converter 🔀",
            shown: false,
        }
    }

    pub fn convert(&mut self) {}

    pub fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        Window::new(self.title)
            .open(&mut self.shown)
            .show(ctx, |ui| {});
    }
}
