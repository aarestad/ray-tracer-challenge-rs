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

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.object.id() == other.object.id()
    }
}

#[derive(Debug, PartialEq)]
pub struct Intersections {
    intersections: Vec<Intersection>,
}

impl Intersections {
    pub fn new(intersections: Vec<Intersection>) -> Intersections {
        Intersections { intersections }
    }

    pub fn empty() -> Intersections {
        Intersections {
            intersections: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn ints(&self) -> &Vec<Intersection> {
        &self.intersections
    }

    pub fn hit(&self) -> Option<&Intersection> {
        let mut nonnegative_t_ints: Vec<&Intersection> =
            self.intersections.iter().filter(|i| i.t >= 0.).collect();

        if nonnegative_t_ints.is_empty() {
            return None;
        }

        nonnegative_t_ints.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Some(nonnegative_t_ints.first().unwrap())
    }
}

pub trait Intersectable: Debug {
    fn intersections(&self, ray: &Ray) -> Intersections;
    fn id(&self) -> i64;
}
