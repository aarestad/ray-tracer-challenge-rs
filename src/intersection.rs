use crate::ray::Ray;
use std::{fmt::Debug, rc::Rc};

#[derive(Debug)]
pub struct Intersection {
    pub t: f32,
    pub object: Rc<dyn Intersectable>,
}

impl Intersection {
    pub fn new(t: f32, object: Rc<dyn Intersectable>) -> Self {
        Self { t, object }
    }
}

pub trait Intersectable: Debug {
    fn intersections(&self, ray: &Ray) -> Option<(Intersection, Intersection)>
    where
        Self: Sized;
}
