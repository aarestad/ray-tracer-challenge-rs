use nalgebra::Matrix4;

use crate::intersection::Intersections;
use crate::material::Material;
use crate::ray::Ray;
use crate::tuple::{Point, Vector};
use crate::util::RayTracerFloat;
use std::fmt::Debug;

mod sphere;

pub use crate::objects::sphere::Sphere;

pub trait Object: Debug {
    fn as_sphere(&self) -> &Sphere {
        panic!("not a sphere");
    }

    fn intersections(&self, ray: &Ray) -> Intersections;
    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;
    fn normal_at(&self, p: Point) -> Vector;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ObjectProps {
    transform: Matrix4<RayTracerFloat>,
    material: Material,
}
