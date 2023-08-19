use crate::intersection::Intersections;
use crate::material::Material;
use crate::ray::Ray;
use crate::transforms::{identity, Transform};
use crate::tuple::{Point, Vector};
use std::fmt::Debug;
use std::rc::Rc;

mod plane;
mod sphere;
mod test_shape;

pub use plane::Plane;
pub use sphere::{custom_glass_sphere, Sphere};
pub use test_shape::TestShape;

use self::internal::PrivateObject;

pub trait Object: Debug + PrivateObject {
    // TODO macro-ize this
    fn as_sphere(&self) -> &Sphere {
        panic!("not a Sphere");
    }

    fn as_test_shape(&self) -> &TestShape {
        panic!("not a TestShape");
    }

    fn as_plane(&self) -> &Plane {
        panic!("not a Plane");
    }

    fn new(transform: Transform, material: Rc<Material>) -> Self
    where
        Self: Sized;

    fn transform(&self) -> &Transform;

    fn material(&self) -> &Rc<Material>;

    fn props(&self) -> &ObjectProps;

    fn intersections(self: Rc<Self>, ray: &Ray) -> Intersections {
        let local_ray = ray.transform(&self.transform().try_inverse().unwrap());
        self.local_intersect(&local_ray)
    }

    fn normal_at(&self, p: Point) -> Vector {
        let inverse = &self.transform().try_inverse().unwrap();
        let local_point = p.transform(inverse);
        let local_normal = self.local_normal_at(&local_point);
        let world_normal = local_normal.transform(&inverse.transpose()).to_vector();

        world_normal.normalize()
    }
}

impl PartialEq for dyn Object {
    fn eq(&self, other: &Self) -> bool {
        self.props() == other.props()
    }
}

mod internal {
    use std::rc::Rc;

    use crate::{
        intersection::Intersections,
        ray::Ray,
        tuple::{Point, Vector},
    };

    pub trait PrivateObject {
        fn local_intersect(self: Rc<Self>, local_ray: &Ray) -> Intersections;
        fn local_normal_at(&self, local_point: &Point) -> Vector;
    }
}

#[derive(Debug, PartialEq)]
pub struct ObjectProps {
    transform: Transform,
    material: Rc<Material>,
}

impl Default for ObjectProps {
    fn default() -> Self {
        Self {
            transform: identity(),
            material: Rc::new(Material::default()),
        }
    }
}
