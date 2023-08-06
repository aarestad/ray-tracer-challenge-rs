use crate::{material::Material, transforms::Transform};

use super::{Object, ObjectProps};

// TODO get rid of Copy!
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct TestShape(ObjectProps);

impl TestShape {
    pub fn new(transform: Transform, material: Material) -> Self {
        Self(ObjectProps {
            transform,
            material,
        })
    }
}

impl Object for TestShape {
    fn as_test_shape(&self) -> &TestShape {
        self
    }

    fn intersections(&self, ray: &crate::ray::Ray) -> crate::intersection::Intersections {
        todo!()
    }

    fn transform(&self) -> &Transform {
        &self.0.transform
    }

    fn material(&self) -> &crate::material::Material {
        &self.0.material
    }

    fn normal_at(&self, p: crate::tuple::Point) -> crate::tuple::Vector {
        todo!()
    }
}
