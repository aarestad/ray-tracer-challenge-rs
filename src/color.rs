use std::{
    fmt::Display,
    num::ParseFloatError,
    ops::{Add, Mul, Sub},
    str::FromStr,
};

use approx::{abs_diff_eq, AbsDiffEq};
use regex::Regex;

use crate::util::EPSILON;
use crate::{tuple::Point, util::RayTracerFloat};

pub const BLACK: Color = Color::new(0., 0., 0.);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color(Point);

impl Default for Color {
    fn default() -> Self {
        Self::new(0., 0., 0.)
    }
}

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

impl AbsDiffEq for Color {
    type Epsilon = RayTracerFloat;

    fn default_epsilon() -> Self::Epsilon {
        EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.0, other.0, epsilon = epsilon)
    }
}

impl Color {
    pub const fn new(r: RayTracerFloat, g: RayTracerFloat, b: RayTracerFloat) -> Color {
        Color(Point::point(r, g, b))
    }

    pub fn red(&self) -> RayTracerFloat {
        self.0.x()
    }

    pub fn green(&self) -> RayTracerFloat {
        self.0.y()
    }

    pub fn blue(&self) -> RayTracerFloat {
        self.0.z()
    }

    #[allow(dead_code)]
    pub fn alpha(&self) -> RayTracerFloat {
        self.0.w()
    }

    pub fn as_ppm_string(&self) -> String {
        format!(
            "{} {} {}",
            (self.red() * 255.).round() as u8,
            (self.green() * 255.).round() as u8,
            (self.blue() * 255.).round() as u8,
        )
    }
}

impl FromStr for Color {
    type Err = String;

    /// Converts from "color(r, g, b)" to a Color
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parens_contents_re = Regex::new(r"color\((.+)\)").expect("bad regex");

        let Some(args_group) = parens_contents_re.captures(s) else {
            return Err(s.to_string());
        };

        let args_str = &args_group[1];

        let args: Vec<Result<RayTracerFloat, ParseFloatError>> = args_str
            .replace(' ', "")
            .split(',')
            .map(RayTracerFloat::from_str)
            .collect();

        if args.iter().any(|a| a.is_err()) {
            return Err(s.to_string());
        }

        Ok(Color::new(
            *args[0].as_ref().unwrap(),
            *args[1].as_ref().unwrap(),
            *args[2].as_ref().unwrap(),
        ))
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

impl Mul<RayTracerFloat> for Color {
    type Output = Self;

    fn mul(self, rhs: RayTracerFloat) -> Self::Output {
        Color::new(self.red() * rhs, self.green() * rhs, self.blue() * rhs)
    }
}
