use nalgebra::Matrix4;

use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::ray::Ray;
use crate::tuple::Tuple;
use std::any::Any;
use std::default::Default;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Object: Debug {
    fn as_any(&self) -> &dyn Any;
    fn intersections(&self, ray: &Ray) -> Intersections;
    fn material(&self) -> Material;
    fn normal_at(&self, p: Tuple) -> Tuple;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    center: Tuple,
    transform: Matrix4<f32>,
    material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Tuple::point(0., 0., 0.),
            transform: Matrix4::identity(),
            material: Default::default(),
        }
    }
}

impl Sphere {
    pub fn new(transform: Matrix4<f32>, material: Material) -> Self {
        Self {
            center: Tuple::point(0., 0., 0.),
            transform,
            material,
        }
    }

    pub fn transform(&self) -> &Matrix4<f32> {
        &self.transform
    }
}

impl Object for Sphere {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn intersections(&self, ray: &Ray) -> Intersections {
        let transformed_ray = ray.transform(
            &self
                .transform
                .try_inverse()
                .expect("cannot invert transform"),
        );

        let sphere_to_ray = transformed_ray.origin - self.center;
        let a = transformed_ray.direction.dot(&transformed_ray.direction);
        let b = 2. * transformed_ray.direction.dot(&sphere_to_ray);
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

    fn material(&self) -> Material {
        self.material
    }

    fn normal_at(&self, p: Tuple) -> Tuple {
        let t = self.transform.try_inverse().expect("not invertible");
        let object_point = p.transform(&t);
        let object_normal = object_point - self.center;
        let world_normal = object_normal.transform(&t.transpose());
        world_normal.normalize()
    }
}
