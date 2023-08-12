use std::rc::Rc;

use crate::{
    intersection::{Intersection, Intersections},
    material::Material,
    ray::Ray,
    transforms::{identity, Transform},
    tuple::{Point, Vector},
    util::EPSILON,
};

use super::{Object, ObjectProps, PrivateObject};

#[derive(Debug)]
pub struct Plane(ObjectProps);

impl Default for Plane {
    fn default() -> Self {
        Self(ObjectProps {
            transform: identity(),
            material: Default::default(),
        })
    }
}

impl PrivateObject for Plane {
    fn local_intersect(self: Rc<Self>, local_ray: &Ray) -> Intersections {
        if local_ray.direction.y().abs() < EPSILON {
            return Intersections::empty();
        }

        let t = -local_ray.origin.y() / local_ray.direction.y();

        Intersections::new(vec![Intersection::new(t, self).into()])
    }

    fn local_normal_at(&self, _local_point: &Point) -> Vector {
        Vector::vector(0., 1., 0.)
    }
}

impl Object for Plane {
    fn new(transform: Transform, material: Rc<Material>) -> Self {
        Self(ObjectProps {
            transform,
            material,
        })
    }

    fn as_plane(&self) -> &Plane {
        self
    }

    fn transform(&self) -> &Transform {
        &self.0.transform
    }

    fn material(&self) -> &Rc<Material> {
        &self.0.material
    }
}
