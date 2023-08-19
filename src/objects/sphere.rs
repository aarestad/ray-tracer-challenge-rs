use crate::intersection::{Intersection, Intersections};
use crate::material::{Material, MaterialBuilder};
use crate::ray::Ray;
use crate::transforms::{identity, Transform};
use crate::tuple::{Point, Vector};
use crate::util::RayTracerFloat;
use std::default::Default;
use std::fmt::Debug;
use std::rc::Rc;

use super::{Object, ObjectProps, PrivateObject};

#[derive(Debug, PartialEq)]
pub struct Sphere(ObjectProps);

impl Default for Sphere {
    fn default() -> Self {
        Self(ObjectProps {
            transform: identity(),
            material: Default::default(),
        })
    }
}

impl PrivateObject for Sphere {
    fn local_intersect(self: Rc<Self>, local_ray: &Ray) -> Intersections {
        let sphere_to_ray = local_ray.origin - Point::origin();
        let a = local_ray.direction.dot(&local_ray.direction);
        let b = 2. * local_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            return Intersections::empty();
        }

        Intersections::new(vec![
            Intersection::new((-b - discriminant.sqrt()) / (2. * a), self.clone()).into(),
            Intersection::new((-b + discriminant.sqrt()) / (2. * a), self).into(),
        ])
    }

    fn local_normal_at(&self, local_point: &Point) -> Vector {
        *local_point - Point::origin()
    }
}

impl Object for Sphere {
    fn new(transform: Transform, material: Rc<Material>) -> Self {
        Self(ObjectProps {
            transform,
            material,
        })
    }

    fn props(&self) -> &ObjectProps {
        &self.0
    }

    fn as_sphere(&self) -> &Sphere {
        self
    }

    fn transform(&self) -> &Transform {
        &self.0.transform
    }

    fn material(&self) -> &Rc<Material> {
        &self.0.material
    }
}

pub fn glass_sphere() -> Sphere {
    custom_glass_sphere(identity(), 1.5)
}

pub fn custom_glass_sphere(transform: Transform, refractive: RayTracerFloat) -> Sphere {
    Sphere::new(
        transform,
        Rc::new(
            MaterialBuilder::default()
                .transparency(1.0)
                .refractive(refractive)
                .build(),
        ),
    )
}

#[cfg(test)]
mod test {
    use super::{glass_sphere, Object};

    #[test]
    fn glass_sphere_properties() {
        let gs = glass_sphere();
        assert_eq!(gs.material().transparency, 1.0);
        assert_eq!(gs.material().refractive, 1.5);
    }
}
