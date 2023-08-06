use crate::{
    objects::Object,
    ray::Ray,
    tuple::{Point, Vector},
    util::RayTracerFloat,
};
use std::{fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub struct Precompute {
    pub intersection: Intersection,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
}

impl Precompute {
    pub fn new(i: Intersection, p: Point, e: Vector, n: Vector, inside: bool) -> Self {
        Self {
            intersection: i,
            point: p,
            eyev: e,
            normalv: n,
            inside,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: RayTracerFloat,
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
    pub fn new(t: RayTracerFloat, object: Rc<dyn Object>) -> Self {
        Self { t, object }
    }

    pub fn precompute_with(&self, r: &Ray) -> Precompute {
        let point = r.position(self.t);
        let eyev = -r.direction;
        let normalv = self.object.normal_at(point);
        let inside = normalv.dot(&eyev) < 0.;

        Precompute::new(
            self.clone(),
            point,
            -r.direction,
            if inside { -normalv } else { normalv },
            inside,
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
