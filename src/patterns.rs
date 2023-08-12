use std::fmt::Debug;

use crate::{
    color::Color,
    transforms::{identity, Transform},
    tuple::Point,
};

mod solid;
mod stripe;

pub use solid::Solid;
pub use stripe::Stripe;

pub trait Pattern: Debug {
    fn as_stripe(&self) -> &Stripe {
        panic!("not a stripe");
    }

    fn transform(&self) -> Option<&Transform> {
        None
    }

    fn local_color_at(&self, p: &Point) -> Color;

    fn color_at(&self, object_point: &Point) -> Color {
        let pattern_point = object_point.transform(
            &self
                .transform()
                .unwrap_or(&identity())
                .try_inverse()
                .unwrap(),
        );

        self.local_color_at(&pattern_point)
    }
}
