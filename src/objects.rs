use nalgebra::Matrix4;

use crate::intersection::{Intersectable, Intersection, Intersections};
use crate::material::Material;
use crate::ray::Ray;
use crate::tuple::Tuple;
use std::default::Default;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Tuple,
    radius: f32,
    transform: Matrix4<f32>,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Tuple::point(0., 0., 0.),
            radius: 1.,
            transform: Matrix4::identity(),
            material: Default::default(),
        }
    }
}

impl Sphere {
    pub fn new(transform: Matrix4<f32>, material: Material) -> Self {
        Self {
            center: Tuple::point(0., 0., 0.),
            radius: 1.,
            transform,
            material,
        }
    }

    pub fn transform(&self) -> &Matrix4<f32> {
        &self.transform
    }

    pub fn normal_at(&self, p: Tuple) -> Tuple {
        let t = self.transform.try_inverse().expect("not invertible");
        let object_point = p.transform(&t);
        let object_normal = object_point - self.center;
        let world_normal = object_normal.transform(&t.transpose());
        return world_normal.normalize();
    }
}

impl Intersectable for Sphere {
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

    fn id(&self) -> i64 {
        self.center.magnitude() as i64
    }
}
