use approx::{abs_diff_eq, AbsDiffEq};
use std::fmt::Debug;

use crate::{
    color::Color,
    transforms::{identity, Transform},
    tuple::Point,
    util::{RayTracerFloat, EPSILON},
};

pub trait Pattern: Debug {
    fn as_stripe(&self) -> &Stripe {
        panic!("not a stripe");
    }

    fn transform(&self) -> Option<&Transform> {
        None
    }

    fn local_color_at(&self, p: &Point) -> Color;

    fn color_at(&self, p: &Point) -> Color {
        let pattern_point = p.transform(
            &self
                .transform()
                .unwrap_or(&identity())
                .try_inverse()
                .unwrap(),
        );

        self.local_color_at(&pattern_point)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Solid(Color);

impl Solid {
    pub fn new(c: Color) -> Self {
        Self(c)
    }
}

impl Pattern for Solid {
    fn local_color_at(&self, _p: &Point) -> Color {
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
    pub even: Color,
    pub odd: Color,
    transform: Transform,
}

impl Stripe {
    pub fn new(even: Color, odd: Color, transform: Transform) -> Self {
        Self {
            even,
            odd,
            transform,
        }
    }
}

impl Pattern for Stripe {
    fn as_stripe(&self) -> &Stripe {
        self
    }

    fn transform(&self) -> Option<&Transform> {
        Some(&self.transform)
    }

    fn local_color_at(&self, p: &Point) -> Color {
        if p.x().floor() % 2. == 0. {
            self.even
        } else {
            self.odd
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::{
        color::{BLACK, WHITE},
        material::Material,
        objects::{Object, Sphere},
        transforms::{identity, scaling, translation},
        tuple::Point,
    };

    use super::{Pattern, Stripe};

    #[test]
    fn stripe_with_object_transform() {
        let p = Stripe::new(WHITE, BLACK, identity());
        let s = Sphere::new(scaling(2., 2., 2.), Rc::new(Material::default()));
        let point = Point::point(1.5, 0., 0.);
        let object_point = point.transform(&s.transform().try_inverse().unwrap());
        let c = p.color_at(&object_point);
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripe_with_pattern_transform() {
        let p = Stripe::new(WHITE, BLACK, scaling(2., 2., 2.));
        let s = Sphere::new(identity(), Rc::new(Material::default()));
        let point = Point::point(1.5, 0., 0.);
        let object_point = point.transform(&s.transform().try_inverse().unwrap());
        let c = p.color_at(&object_point);
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripe_with_pattern_and_obj_transform() {
        let p = Stripe::new(WHITE, BLACK, translation(0.5, 0., 0.));
        let s = Sphere::new(scaling(2., 2., 2.), Rc::new(Material::default()));
        let point = Point::point(2.5, 0., 0.);
        let object_point = point.transform(&s.transform().try_inverse().unwrap());
        let c = p.color_at(&object_point);
        assert_eq!(c, WHITE);
    }
}
