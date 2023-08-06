use crate::transforms::Transform;
use crate::tuple::{Point, Vector};
use crate::util::RayTracerFloat;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        assert!(origin.is_point());
        assert!(direction.is_vector());
        Ray { origin, direction }
    }

    pub fn position(&self, t: RayTracerFloat) -> Point {
        self.origin + self.direction * t
    }

    pub fn transform(&self, transform: &Transform) -> Ray {
        Ray::new(
            self.origin.transform(transform),
            self.direction.transform(transform),
        )
    }
}
