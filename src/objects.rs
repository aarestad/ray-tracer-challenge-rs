use crate::intersection::{Intersectable, Intersection, Intersections};
use crate::ray::Ray;
use crate::tuple::Tuple;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, Default)]
pub struct Sphere {
    center: Tuple,
    radius: f32,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Tuple::point(0., 0., 0.),
            radius: 1.,
        }
    }

    pub fn center(&self) -> &Tuple {
        &self.center
    }
}

impl Intersectable for Sphere {
    fn intersections(&self, ray: &Ray) -> Intersections {
        let sphere_to_ray = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            return Intersections::empty();
        }

        Intersections::new(vec![
            Intersection::new((-b - discriminant.sqrt()) / (2. * a), Rc::new(*self)),
            Intersection::new((-b + discriminant.sqrt()) / (2. * a), Rc::new(*self)),
        ])
    }

    fn id(&self) -> i64 {
        self.center.magnitude() as i64
    }
}
