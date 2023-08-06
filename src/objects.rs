use crate::intersection::Intersections;
use crate::material::Material;
use crate::ray::Ray;
use crate::transforms::Transform;
use crate::tuple::{Point, Vector};
use crate::util::RayTracerFloat;
use std::fmt::Debug;

mod sphere;
mod test_shape;

pub use crate::objects::sphere::Sphere;
pub use crate::objects::test_shape::TestShape;

pub trait Object: Debug {
    fn as_sphere(&self) -> &Sphere {
        panic!("not a Sphere");
    }

    fn as_test_shape(&self) -> &TestShape {
        panic!("not a TestShape");
    }

    fn intersections(&self, ray: &Ray) -> Intersections;
    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;
    fn normal_at(&self, p: Point) -> Vector;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ObjectProps {
    transform: Transform,
    material: Material,
}
