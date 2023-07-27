use crate::{objects::Object, ray::Ray, tuple::Tuple};
use std::{fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub struct Precompute {
    pub intersection: Intersection,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
}

impl Precompute {
    pub fn new(i: Intersection, p: Tuple, e: Tuple, n: Tuple) -> Self {
        Self {
            intersection: i,
            point: p,
            eyev: e,
            normalv: n,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f32,
    pub object: Rc<dyn Object>,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl Intersection {
    pub fn new(t: f32, object: Rc<dyn Object>) -> Self {
        Self { t, object }
    }

    pub fn precompute_with(&self, r: &Ray) -> Precompute {
        let p = r.position(self.t);

        Precompute::new(
            self.clone(),
            r.position(self.t),
            -r.direction,
            self.object.normal_at(p),
        )
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Intersections {
    intersections: Vec<Intersection>,
}

impl Intersections {
    pub fn new(intersections: Vec<Intersection>) -> Intersections {
        Intersections { intersections }
    }

    pub fn empty() -> Intersections {
        Self::default()
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
