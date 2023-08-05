use crate::tuple::Tuple;
use nalgebra::Matrix4;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        assert!(origin.is_point());
        assert!(direction.is_vector());
        Ray { origin, direction }
    }

    pub fn position(&self, t: f32) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(&self, transform: &Matrix4<f32>) -> Ray {
        Ray::new(
            self.origin.transform(transform),
            self.direction.transform(transform),
        )
    }
}
