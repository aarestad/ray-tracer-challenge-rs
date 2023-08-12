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
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub reflectv: Vector,
    pub inside: bool,
    pub over_point: Point,
}

impl Precompute {
    pub fn new(
        i: Rc<Intersection>,
        p: Point,
        e: Vector,
        n: Vector,
        r: Vector,
        inside: bool,
        over_point: Point,
    ) -> Self {
        Self {
            intersection: i,
            point: p,
            eyev: e,
            normalv: n,
            reflectv: r,
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
        let reflectv = r.direction.reflect(&normalv);
        let inside = normalv.dot(&eyev) < 0.;
        let over_point = world_point + normalv * EPSILON;

        Precompute::new(
            self.clone(),
            world_point,
            eyev,
            if inside { -normalv } else { normalv },
            reflectv,
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

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use approx::assert_abs_diff_eq;

    use crate::{
        objects::Plane,
        ray::Ray,
        tuple::{Point, Vector},
        util::RayTracerFloat,
    };

    use super::Intersection;

    const SQRT_2: RayTracerFloat = 1.4142135623730951;

    #[test]
    fn precompute_reflectv() {
        let o = Plane::default();
        let r = Ray::new(
            Point::point(0., 1., -1.),
            Vector::vector(0., -SQRT_2 / 2., SQRT_2 / 2.),
        );
        let i = Rc::new(Intersection::new(SQRT_2, Rc::new(o)));
        let comps = i.precompute_with(&r);
        assert_abs_diff_eq!(comps.reflectv, Vector::vector(0., SQRT_2 / 2., SQRT_2 / 2.));
    }
}
