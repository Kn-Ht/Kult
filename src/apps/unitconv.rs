pub struct UnitConverter {
    pub title: &'static str,
    pub shown: bool,
    from: i32, // TODO: cargo add unit-convertions
    to: i32,
    value: f64
}

impl UnitConverter {
    pub const fn new() -> Self {
        Self {
            title: "Unit Converter ⚖️",
            shown: false,
            value: 0.621,
            from: 1,
            to: 1
        }
    }

    pub fn convert(&mut self) {
        
    }
}