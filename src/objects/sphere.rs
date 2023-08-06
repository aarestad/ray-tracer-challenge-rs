use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::ray::Ray;
use crate::transforms::{identity, Transform};
use crate::tuple::{Point, Vector};
use std::default::Default;
use std::fmt::Debug;
use std::rc::Rc;

use super::{Object, ObjectProps, PrivateObject};

// TODO get rid of Copy!
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere(ObjectProps);

impl Default for Sphere {
    fn default() -> Self {
        Self(ObjectProps {
            transform: identity(),
            material: Default::default(),
        })
    }
}

impl Sphere {
    pub fn new(transform: Transform, material: Material) -> Self {
        Self(ObjectProps {
            transform,
            material,
        })
    }
}

impl PrivateObject for Sphere {
    fn local_intersect(&self, local_ray: &Ray) -> Intersections {
        let sphere_to_ray = local_ray.origin - Point::origin();
        let a = local_ray.direction.dot(&local_ray.direction);
        let b = 2. * local_ray.direction.dot(&sphere_to_ray);
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

    fn local_normal_at(&self, local_point: &Point) -> Vector {
        *local_point - Point::origin()
    }
}

impl Object for Sphere {
    fn as_sphere(&self) -> &Sphere {
        self
    }

    fn transform(&self) -> &Transform {
        &self.0.transform
    }

    fn material(&self) -> &Material {
        &self.0.material
    }
}