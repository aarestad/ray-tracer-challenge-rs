use approx::{abs_diff_eq, AbsDiffEq};
use std::fmt::Debug;

use crate::{
    color::{Color},
    tuple::Point,
    util::{RayTracerFloat, EPSILON},
};

pub trait Pattern: Debug {
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
    even: Color,
    odd: Color,
}

impl Stripe {
    pub fn new(even: Color, odd: Color) -> Self {
        Self { even, odd }
    }
}

impl Pattern for Stripe {
    fn color_at(&self, p: &Point) -> Color {
        if p.x().floor() % 2. == 0. { self.even } else { self.odd }
    }
}
