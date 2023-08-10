use std::rc::Rc;

use crate::{
    intersection::Intersections,
    material::Material,
    ray::Ray,
    transforms::Transform,
    tuple::{Point, Vector},
};

use super::{Object, ObjectProps, PrivateObject};

// TODO get rid of Copy!
#[derive(Debug, Default)]
pub struct TestShape(ObjectProps);

impl TestShape {
    pub fn new(transform: Transform, material: Rc<Material>) -> Self {
        Self(ObjectProps {
            transform,
            material,
        })
    }
}

impl PrivateObject for TestShape {
    fn local_intersect(self: Rc<Self>, _local_ray: &Ray) -> Intersections {
        Intersections::empty()
    }

    fn local_normal_at(&self, local_point: &Point) -> Vector {
        local_point.to_vector()
    }
}

impl Object for TestShape {
    fn as_test_shape(&self) -> &TestShape {
        self
    }
    fn transform(&self) -> &Transform {
        &self.0.transform
    }

    fn material(&self) -> &Rc<Material> {
        &self.0.material
    }
}
