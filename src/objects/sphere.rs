use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::ray::Ray;
use crate::transforms::{identity, Transform};
use crate::tuple::{Point, Vector};
use std::default::Default;
use std::fmt::Debug;
use std::rc::Rc;

use super::{Object, ObjectProps};

// TODO get rid of Copy!
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    center: Point,
    props: ObjectProps,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Point::point(0., 0., 0.),
            props: ObjectProps {
                transform: identity(),
                material: Default::default(),
            },
        }
    }
}

impl Sphere {
    pub fn new(transform: Transform, material: Material) -> Self {
        Self {
            center: Point::point(0., 0., 0.),
            props: ObjectProps {
                transform,
                material,
            },
        }
    }
}

impl Object for Sphere {
    fn as_sphere(&self) -> &Sphere {
        self
    }

    fn intersections(&self, ray: &Ray) -> Intersections {
        let transformed_ray = ray.transform(
            &self
                .props
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

    fn transform(&self) -> &Transform {
        &self.props.transform
    }

    fn material(&self) -> &Material {
        &self.props.material
    }

    fn normal_at(&self, p: Point) -> Vector {
        let t = self
            .props
            .transform
            .try_inverse()
            .expect("transform not invertible");

        let object_point = p.transform(&t);
        let object_normal = object_point - self.center;
        let world_normal = object_normal.transform(&t.transpose());
        world_normal.normalize()
    }
}
