use std::{fmt::Debug, rc::Rc};

use crate::{
    color::Color,
    objects::Object,
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

    fn color_at(&self, object: &dyn Object, world_point: &Point) -> Color {
        let object_point = world_point.transform(&object.transform().try_inverse().unwrap());

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
