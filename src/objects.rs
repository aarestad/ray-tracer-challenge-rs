use crate::ray::Ray;
use crate::tuple::Tuple;
use std::default::Default;

#[derive(Debug)]
pub struct Intersection(Vec<f32>);

impl Intersection {
    pub fn new(times: Vec<f32>) -> Self {
        Self(times)
    }

    pub fn intersections(&self) -> &Vec<f32> {
        &self.0
    }
}

pub trait Intersectable {
    fn intersection(&self, ray: &Ray) -> Intersection;
}

#[derive(Debug)]
pub struct Sphere {
    origin: Tuple,
    radius: f32,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            origin: Default::default(),
            radius: 1.,
        }
    }
}

impl Intersectable for Sphere {
    fn intersection(&self, ray: &Ray) -> Intersection {
        Intersection::new(vec![])
    }
}
