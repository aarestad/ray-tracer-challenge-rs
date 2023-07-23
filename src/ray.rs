use crate::tuple::Tuple;

#[derive(Debug)]
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
}
