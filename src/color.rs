use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

use crate::tuple::Tuple;

#[derive(Debug, PartialEq, Copy, Clone)]
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

    pub fn approx_eq(&self, rhs: &Color) -> bool {
        self.0.approx_eq(&rhs.0)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red() + rhs.red(),
            self.green() + rhs.green(),
            self.blue() + rhs.blue(),
        )
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red() - rhs.red(),
            self.green() - rhs.green(),
            self.blue() - rhs.blue(),
        )
    }
}

/// Hadamard product of the underlying tuples
impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red() * rhs.red(),
            self.green() * rhs.green(),
            self.blue() * rhs.blue(),
        )
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::new(self.red() * rhs, self.green() * rhs, self.blue() * rhs)
    }
}
