use crate::{
    objects::Object,
    ray::Ray,
    tuple::{Point, Vector},
    util::{RayTracerFloat, EPSILON},
};
use std::{fmt::Debug, rc::Rc};

#[derive(Debug)]
pub struct Precompute {
    pub intersection: Rc<Intersection>,
    pub world_point: Point,
    pub object_point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
    pub over_point: Point,
}

impl Precompute {
    pub fn new(
        i: Rc<Intersection>,
        p: Point,
        op: Point,
        e: Vector,
        n: Vector,
        inside: bool,
        over_point: Point,
    ) -> Self {
        Self {
            intersection: i,
            world_point: p,
            object_point: op,
            eyev: e,
            normalv: n,
            inside,
            over_point,
        }
    }
}

#[derive(Debug)]
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

    pub fn precompute_with(self: &Rc<Self>, r: &Ray) -> Precompute {
        let world_point = r.position(self.t);
        let eyev = -r.direction;
        let normalv = self.object.normal_at(world_point);
        let inside = normalv.dot(&eyev) < 0.;
        let over_point = world_point + normalv * EPSILON;

        Precompute::new(
            self.clone(),
            world_point,
            world_point.transform(&self.object.transform().try_inverse().unwrap()),
            -r.direction,
            if inside { -normalv } else { normalv },
            inside,
            over_point,
        )
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Intersections(Vec<Rc<Intersection>>);

impl Intersections {
    pub fn new(intersections: Vec<Rc<Intersection>>) -> Intersections {
        Intersections(intersections)
    }

    pub fn empty() -> Intersections {
        Self::default()
    }

    pub fn ints(&self) -> &Vec<Rc<Intersection>> {
        &self.0
    }

    pub fn hit(self: Rc<Self>) -> Option<Rc<Intersection>> {
        let mut nonnegative_t_ints: Vec<Rc<Intersection>> =
            self.0.iter().filter(|i| i.t >= 0.).cloned().collect();

        if nonnegative_t_ints.is_empty() {
            return None;
        }

        nonnegative_t_ints.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Some(nonnegative_t_ints.first().unwrap().clone())
    }
}
