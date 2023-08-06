use approx::{abs_diff_eq, AbsDiffEq};

use crate::{
    color::Color,
    tuple::Point,
    util::{RayTracerFloat, EPSILON},
};

pub trait Pattern {
    fn color_at(&self, p: &Point) -> Color;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Solid(Color);

impl Solid {
    pub fn new(c: Color) -> Self {
        Self(c)
    }
}

impl Pattern for Solid {
    fn color_at(&self, _p: &Point) -> Color {
        self.0
    }
}

impl AbsDiffEq for Solid {
    type Epsilon = RayTracerFloat;

    fn default_epsilon() -> Self::Epsilon {
        EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.0, other.0, epsilon = epsilon)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Stripe {
    a: Color,
    b: Color,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Stripe {
        Stripe { a, b }
    }
}

impl Pattern for Stripe {
    fn color_at(&self, p: &Point) -> Color {
        todo!()
    }
}
