use std::rc::Rc;

use crate::{
    intersection::{Intersection, Intersections},
    material::Material,
    ray::Ray,
    transforms::Transform,
    tuple::{Point, Vector},
    util::EPSILON,
};

use super::{Object, ObjectProps, PrivateObject};

// TODO get rid of copyyyy
#[derive(Debug, Copy, Clone)]
pub struct Plane(ObjectProps);

impl PrivateObject for Plane {
    fn local_intersect(&self, local_ray: &Ray) -> Intersections {
        if local_ray.direction.y() < EPSILON {
            return Intersections::empty();
        }

        let t = -local_ray.origin.y() / local_ray.direction.y();

        Intersections::new(vec![Intersection::new(t, Rc::new(*self))])
    }

    fn local_normal_at(&self, _local_point: &Point) -> Vector {
        Vector::vector(0., 1., 0.)
    }
}

impl Object for Plane {
    fn transform(&self) -> &Transform {
        &self.0.transform
    }

    fn material(&self) -> &Material {
        &self.0.material
    }
}
