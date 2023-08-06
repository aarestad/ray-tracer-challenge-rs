use crate::tuple::{Point, Vector};
use nalgebra::Matrix4;

#[derive(Debug, Copy, Clone, Default)]
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

    pub fn position(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }

    pub fn transform(&self, transform: &Matrix4<f32>) -> Ray {
        Ray::new(
            self.origin.transform(transform),
            self.direction.transform(transform),
        )
    }
}
