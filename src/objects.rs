use crate::intersection::Intersections;
use crate::material::Material;
use crate::ray::Ray;
use crate::transforms::{identity, Transform};
use crate::tuple::{Point, Vector};
use std::fmt::Debug;

mod plane;
mod sphere;
mod test_shape;

pub use crate::objects::plane::Plane;
pub use crate::objects::sphere::Sphere;
pub use crate::objects::test_shape::TestShape;

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

    fn intersections(&self, ray: &Ray) -> Intersections {
        let local_ray = ray.transform(&self.transform().try_inverse().unwrap());
        self.local_intersect(&local_ray)
    }

    fn transform(&self) -> &Transform;

    fn material(&self) -> &Material;

    fn normal_at(&self, p: Point) -> Vector {
        let inverse = &self.transform().try_inverse().unwrap();
        let local_point = p.transform(inverse);
        let local_normal = self.local_normal_at(&local_point);
        let world_normal = local_normal.transform(&inverse.transpose()).to_vector();

        world_normal.normalize()
    }
}

mod internal {
    use crate::{
        intersection::Intersections,
        ray::Ray,
        tuple::{Point, Vector},
    };

    pub trait PrivateObject {
        fn local_intersect(&self, local_ray: &Ray) -> Intersections;
        fn local_normal_at(&self, local_point: &Point) -> Vector;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ObjectProps {
    transform: Transform,
    material: Material,
}

impl Default for ObjectProps {
    fn default() -> Self {
        Self {
            transform: identity(),
            material: Material::default(),
        }
    }
}
