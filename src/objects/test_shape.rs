use crate::{
    material::Material,
    transforms::{identity, Transform},
};

use super::Object;

// TODO get rid of Copy!
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TestShape {
    transform: Transform,
    material: Material,
}

impl Default for TestShape {
    fn default() -> Self {
        Self {
            transform: identity(),
            material: Material::default(),
        }
    }
}

impl TestShape {
    pub fn new(transform: Transform, material: Material) -> Self {
        Self {
            transform,
            material,
        }
    }
}

impl Object for TestShape {
    fn intersections(&self, ray: &crate::ray::Ray) -> crate::intersection::Intersections {
        todo!()
    }

    fn material(&self) -> &crate::material::Material {
        todo!()
    }

    fn material_mut(&mut self) -> &mut crate::material::Material {
        todo!()
    }

    fn normal_at(&self, p: crate::tuple::Point) -> crate::tuple::Vector {
        todo!()
    }
}
