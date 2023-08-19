use crate::intersection::{Intersection, Intersections};
use crate::material::{Material, MaterialBuilder};
use crate::ray::Ray;
use crate::transforms::{identity, Transform};
use crate::tuple::{Point, Vector};
use crate::util::{RayTracerFloat, EPSILON};
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Object {
    TestShape(Transform, Rc<Material>),
    Plane(Transform, Rc<Material>),
    Sphere(Transform, Rc<Material>),
    Cube(Transform, Rc<Material>),
}

impl Object {
    pub fn transform(&self) -> &Transform {
        match self {
            Object::TestShape(t, _) => &t,
            Object::Plane(t, _) => &t,
            Object::Sphere(t, _) => &t,
            Object::Cube(t, _) => &t,
        }
    }

    pub fn material(&self) -> &Rc<Material> {
        match self {
            Object::TestShape(_, m) => m,
            Object::Plane(_, m) => m,
            Object::Sphere(_, m) => m,
            Object::Cube(_, m) => m,
        }
    }

    pub fn intersections(self: Rc<Self>, ray: &Ray) -> Intersections {
        let local_ray = ray.transform(&self.transform().try_inverse().unwrap());

        match self.as_ref() {
            Object::TestShape(_, _) => Intersections::empty(),
            Object::Plane(_, _) => {
                if local_ray.direction.y().abs() < EPSILON {
                    return Intersections::empty();
                }

                let t = -local_ray.origin.y() / local_ray.direction.y();

                Intersections::new(vec![Intersection::new(t, self).into()])
            }
            Object::Sphere(_, _) => {
                let sphere_to_ray = local_ray.origin - Point::origin();
                let a = local_ray.direction.dot(&local_ray.direction);
                let b = 2. * local_ray.direction.dot(&sphere_to_ray);
                let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

                let discriminant = b.powi(2) - 4. * a * c;

                if discriminant < 0. {
                    return Intersections::empty();
                }

                Intersections::new(vec![
                    Intersection::new((-b - discriminant.sqrt()) / (2. * a), self.clone()).into(),
                    Intersection::new((-b + discriminant.sqrt()) / (2. * a), self).into(),
                ])
            }
            Object::Cube(_, _) => todo!(),
        }
    }

    pub fn normal_at(&self, p: Point) -> Vector {
        let inverse = &self.transform().try_inverse().unwrap();
        let local_point = p.transform(inverse);

        let local_normal = match self {
            Object::TestShape(_, _) => local_point.to_vector(),
            Object::Plane(_, _) => Vector::vector(0., 1., 0.),
            Object::Sphere(_, _) => local_point - Point::origin(),
            Object::Cube(_, _) => todo!(),
        };

        let world_normal = local_normal.transform(&inverse.transpose()).to_vector();

        world_normal.normalize()
    }
}

// TODO cfg(test)
pub fn default_test_shape() -> Object {
    Object::TestShape(identity(), Material::default().into())
}

pub fn default_sphere() -> Object {
    Object::Sphere(identity(), Material::default().into())
}

pub fn default_plane() -> Object {
    Object::Plane(identity(), Material::default().into())
}

pub fn glass_sphere() -> Object {
    custom_glass_sphere(identity(), 1.5)
}

pub fn custom_glass_sphere(transform: Transform, refractive: RayTracerFloat) -> Object {
    Object::Sphere(
        transform,
        MaterialBuilder::default()
            .transparency(1.0)
            .refractive(refractive)
            .build()
            .into(),
    )
}

#[cfg(test)]
mod test {
    use super::glass_sphere;

    #[test]
    fn glass_sphere_properties() {
        let gs = glass_sphere();
        assert_eq!(gs.material().transparency, 1.0);
        assert_eq!(gs.material().refractive, 1.5);
    }
}
