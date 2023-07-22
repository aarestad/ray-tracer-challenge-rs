use std::fmt::Display;

use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Color(Tuple);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "color({}, {}, {})",
            self.red(),
            self.green(),
            self.blue()
        ))
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(Tuple::new(r, g, b, 1.0))
    }

    pub fn red(&self) -> f32 {
        self.0.x
    }

    pub fn green(&self) -> f32 {
        self.0.y
    }

    pub fn blue(&self) -> f32 {
        self.0.z
    }

    pub fn alpha(&self) -> f32 {
        self.0.w
    }
}
