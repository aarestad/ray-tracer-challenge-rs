use crate::intersection::{Intersection, Intersections};
use crate::material::{Material, MaterialBuilder};
use crate::ray::Ray;
use crate::transforms::{identity, Transform};
use crate::tuple::{Point, Vector};
use crate::util::{RayTracerFloat, EPSILON};
use std::fmt::Debug;
use std::mem::swap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Object {
    Test(Transform, Material),
    Plane(Transform, Material),
    Sphere(Transform, Material),
    Cube(Transform, Material),
}

impl Object {
    pub fn transform(&self) -> &Transform {
        match self {
            Object::Test(t, _)
            | Object::Plane(t, _)
            | Object::Sphere(t, _)
            | Object::Cube(t, _) => t,
        }
    }

    pub fn material(&self) -> &Material {
        match self {
            Object::Test(_, m)
            | Object::Plane(_, m)
            | Object::Sphere(_, m)
            | Object::Cube(_, m) => m,
        }
    }

    pub fn intersections(self: Rc<Self>, ray: &Ray) -> Intersections {
        let local_ray = ray.transform(&self.transform().try_inverse().unwrap());

        match self.as_ref() {
            Object::Test(_, _) => Intersections::empty(),
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
            Object::Cube(_, _) => {
                /// Returns tmin, tmax for a particular axis's origin/direction components
                fn check_axis(
                    origin_component: RayTracerFloat,
                    direction_component: RayTracerFloat,
                ) -> (RayTracerFloat, RayTracerFloat) {
                    let tmin_numerator = -1.0 - origin_component;
                    let tmax_numerator = 1.0 - origin_component;

                    let (mut tmin, mut tmax) = if direction_component.abs() >= EPSILON {
                        (
                            tmin_numerator / direction_component,
                            tmax_numerator / direction_component,
                        )
                    } else {
                        (-RayTracerFloat::INFINITY, RayTracerFloat::INFINITY)
                    };

                    if tmin > tmax {
                        swap(&mut tmin, &mut tmax)
                    }

                    (tmin, tmax)
                }

                let (xtmin, xtmax) = check_axis(ray.origin.x(), ray.direction.x());
                let (ytmin, ytmax) = check_axis(ray.origin.y(), ray.direction.y());
                let (ztmin, ztmax) = check_axis(ray.origin.z(), ray.direction.z());

                let tmin = xtmin.max(ytmin.max(ztmin));
                let tmax = xtmax.min(ytmax.min(ztmax));

                Intersections::new(vec![
                    Intersection::new(tmin, self.clone()).into(),
                    Intersection::new(tmax, self).into(),
                ])
            }
        }
    }

    pub fn normal_at(&self, p: Point) -> Vector {
        let inverse = &self.transform().try_inverse().unwrap();
        let local_point = p.transform(inverse);

        let local_normal = match self {
            Object::Test(_, _) => local_point.to_vector(),
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
    Object::Test(identity(), Material::default())
}

pub fn default_sphere() -> Object {
    Object::Sphere(identity(), Material::default())
}

pub fn default_plane() -> Object {
    Object::Plane(identity(), Material::default())
}

pub fn glass_sphere() -> Object {
    custom_glass_sphere(identity(), 1.5)
}

pub fn default_cube() -> Object {
    Object::Cube(identity(), Material::default())
}

pub fn custom_glass_sphere(transform: Transform, refractive: RayTracerFloat) -> Object {
    Object::Sphere(
        transform,
        MaterialBuilder::default()
            .transparency(1.0)
            .refractive(refractive)
            .build(),
    )
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::{
        ray::Ray,
        tuple::{Point, Vector},
    };

    use super::{default_cube, glass_sphere};

    #[test]
    fn glass_sphere_properties() {
        let gs = glass_sphere();
        assert_eq!(gs.material().transparency, 1.0);
        assert_eq!(gs.material().refractive, 1.5);
    }

    #[test]
    fn ray_intersects_cube() {
        // (ray, t1, t2)
        let examples = vec![
            (
                Ray::new(Point::point(5.0, 0.5, 0.0), Vector::vector(-1.0, 0.0, 0.0)),
                4.0,
                6.0,
            ),
            (
                Ray::new(Point::point(-5.0, 0.5, 0.0), Vector::vector(1.0, 0.0, 0.0)),
                4.0,
                6.0,
            ),
            (
                Ray::new(Point::point(0.5, 5.0, 0.0), Vector::vector(0.0, -1.0, 0.0)),
                4.0,
                6.0,
            ),
            (
                Ray::new(Point::point(0.5, -5.0, 0.0), Vector::vector(0.0, 1.0, 0.0)),
                4.0,
                6.0,
            ),
            (
                Ray::new(Point::point(0.5, 0.0, 5.0), Vector::vector(0.0, 0.0, -1.0)),
                4.0,
                6.0,
            ),
            (
                Ray::new(Point::point(0.5, 0.0, -5.0), Vector::vector(0.0, 0.0, 1.0)),
                4.0,
                6.0,
            ),
            (
                Ray::new(Point::point(0.0, 0.5, 0.0), Vector::vector(0.0, 0.0, 1.0)),
                -1.0,
                1.0,
            ),
        ];

        let c = Rc::new(default_cube());

        for (r, t1, t2) in examples {
            let xs = c.clone().intersections(&r);
            assert_eq!(xs.ints().len(), 2);
            assert_eq!(xs.ints()[0].t, t1);
            assert_eq!(xs.ints()[1].t, t2);
        }
    }
}
