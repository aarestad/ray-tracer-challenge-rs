use crate::ray::Ray;
use crate::tuple::Tuple;
use std::default::Default;

pub struct Intersection {
    times: Vec<f32>,
}

impl Intersection {
    pub fn new(times: Vec<f32>) -> Intersection {
        Intersection { times }
    }

    pub fn count(&self) -> usize {
        self.times.len()
    }
}

pub trait Intersectable {
    fn intersection(ray: Ray) -> Intersection;
}

#[derive(Debug)]
pub struct Sphere {
    origin: Tuple,
    radius: f32,
}

impl Sphere {
    fn new() -> Self {
        Self {
            origin: Default::default(),
            radius: 1.,
        }
    }
}

impl Intersectable for Sphere {
    fn intersection(ray: Ray) -> Intersection {
        Intersection::new(vec![])
    }
}
