use super::{internal::PrivateObject, Object};

#[derive(Debug, PartialEq)]
pub struct Cube {}

impl PrivateObject for Cube {
    fn local_intersect(
        self: std::rc::Rc<Self>,
        local_ray: &crate::ray::Ray,
    ) -> crate::intersection::Intersections {
        todo!()
    }

    fn local_normal_at(&self, local_point: &crate::tuple::Point) -> crate::tuple::Vector {
        todo!()
    }
}

impl Object for Cube {
    fn new(
        transform: crate::transforms::Transform,
        material: std::rc::Rc<crate::material::Material>,
    ) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn transform(&self) -> &crate::transforms::Transform {
        todo!()
    }

    fn material(&self) -> &std::rc::Rc<crate::material::Material> {
        todo!()
    }

    fn props(&self) -> &super::ObjectProps {
        todo!()
    }
}
