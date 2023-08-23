use crate::intersection::{Intersection, Intersections};
use crate::material::{Material, MaterialBuilder};
use crate::ray::Ray;
use crate::transforms::{identity, Transform};
use crate::tuple::{Point, Tuple, Vector};
use crate::util::{RayTracerFloat, EPSILON};
use std::fmt::Debug;
use std::mem::swap;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    // TODO cfg[test]
    Test,
    Plane,
    Sphere,
    Cube,
    Group(Vec<Rc<Object>>),
    // min_y, max_y (both exclusive), closed
    Cylinder(RayTracerFloat, RayTracerFloat, bool),
    DoubleNappedCone(RayTracerFloat, RayTracerFloat, bool),
}

#[derive(Debug, Clone)]
pub struct Object {
    pub transform: Transform,
    pub material: Material,
    obj_type: ObjectType,
    parent: Weak<Object>,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform && self.material == other.material
        // the "parent" check below prevents this equality check from recursing infinitely
        && self.obj_type == other.obj_type
        // reference equality for parent to short circuit inifinite recursion
        && self.parent.ptr_eq(&other.parent)
    }
}

impl Object {
    pub fn test(transform: Transform, material: Material) -> Self {
        Self {
            transform,
            material,
            obj_type: ObjectType::Test,
            parent: Weak::new(),
        }
    }

    pub fn plane(transform: Transform, material: Material) -> Self {
        Self {
            transform,
            material,
            obj_type: ObjectType::Plane,
            parent: Weak::new(),
        }
    }

    pub fn sphere(transform: Transform, material: Material) -> Self {
        Self {
            transform,
            material,
            obj_type: ObjectType::Sphere,
            parent: Weak::new(),
        }
    }

    pub fn cube(transform: Transform, material: Material) -> Self {
        Self {
            transform,
            material,
            obj_type: ObjectType::Cube,
            parent: Weak::new(),
        }
    }

    pub fn group(transform: Transform, mut children: Vec<Rc<Object>>) -> Rc<Self> {
        let mut new_group = Rc::new(Self {
            transform,
            material: Material::default(),
            obj_type: ObjectType::Group(vec![]),
            parent: Weak::new(),
        });

        // SAFETY: Only called during single-threaded initialization; in particular, the children
        // may have other referents, but they will not be dereferenced during this block
        for child in children.iter_mut() {
            unsafe {
                // Required because the child may itself be a group with its own child-referents
                Rc::get_mut_unchecked(child).parent = Rc::downgrade(&new_group);

                // Required because there will always be >1 reference to new_group at this point
                match &mut Rc::get_mut_unchecked(&mut new_group).obj_type {
                    ObjectType::Group(children) => children.push(child.clone()),
                    _ => unreachable!(),
                }
            }
        }

        new_group
    }

    pub fn cylinder(
        transform: Transform,
        material: Material,
        min_y: RayTracerFloat,
        max_y: RayTracerFloat,
        closed: bool,
    ) -> Self {
        Self {
            transform,
            material,
            obj_type: ObjectType::Cylinder(min_y, max_y, closed),
            parent: Weak::new(),
        }
    }

    pub fn cone(
        transform: Transform,
        material: Material,
        min_y: RayTracerFloat,
        max_y: RayTracerFloat,
        closed: bool,
    ) -> Self {
        Self {
            transform,
            material,
            obj_type: ObjectType::DoubleNappedCone(min_y, max_y, closed),
            parent: Weak::new(),
        }
    }

    pub fn intersections(self: Rc<Self>, ray: &Ray) -> Intersections {
        // "un-transforms" the ray so it's relative to the origin-centered,
        // unit-1-sized default for this Object
        let local_ray = ray.transform(&self.transform.try_inverse().unwrap());

        match &self.obj_type {
            ObjectType::Test => Intersections::empty(),
            ObjectType::Plane => {
                if local_ray.direction.y().abs() < EPSILON {
                    return Intersections::empty();
                }

                let t = -local_ray.origin.y() / local_ray.direction.y();

                Intersections::new(vec![Intersection::new(t, self).into()])
            }
            ObjectType::Sphere => {
                let sphere_to_ray = local_ray.origin - Point::origin();
                let a = local_ray.direction.dot(&local_ray.direction);
                let b = 2. * local_ray.direction.dot(&sphere_to_ray);
                let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

                let discriminant = b.powi(2) - 4. * a * c;

                if discriminant < 0.0 {
                    return Intersections::empty();
                }

                Intersections::new(vec![
                    Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), self.clone()).into(),
                    Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), self).into(),
                ])
            }
            ObjectType::Cube => {
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
                        (
                            tmin_numerator * RayTracerFloat::INFINITY,
                            tmax_numerator * RayTracerFloat::INFINITY,
                        )
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

                if tmin > tmax {
                    Intersections::empty()
                } else {
                    Intersections::new(vec![
                        Intersection::new(tmin, self.clone()).into(),
                        Intersection::new(tmax, self).into(),
                    ])
                }
            }
            ObjectType::Cylinder(min_y, max_y, closed) => {
                let a = local_ray.direction.x().powi(2) + local_ray.direction.z().powi(2);
                let mut intersections: Vec<Rc<Intersection>> = vec![];

                if a.abs() >= EPSILON {
                    // ray is not parallel to the y axis
                    let b = 2.0 * local_ray.origin.x() * local_ray.direction.x()
                        + 2.0 * local_ray.origin.z() * local_ray.direction.z();
                    let c = local_ray.origin.x().powi(2) + local_ray.origin.z().powi(2) - 1.0;
                    let discriminant = b.powi(2) - 4.0 * a * c;

                    if discriminant < 0.0 {
                        return Intersections::empty();
                    }

                    let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
                    let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

                    let y0 = local_ray.origin.y() + t0 * local_ray.direction.y();
                    let y1 = local_ray.origin.y() + t1 * local_ray.direction.y();

                    if *min_y < y0 && y0 < *max_y {
                        intersections.push(Intersection::new(t0, self.clone()).into());
                    }

                    if *min_y < y1 && y1 < *max_y {
                        intersections.push(Intersection::new(t1, self.clone()).into());
                    }
                }

                fn ray_within_cylinder_at_t(ray: &Ray, t: RayTracerFloat) -> bool {
                    let x = ray.origin.x() + t * ray.direction.x();
                    let z = ray.origin.z() + t * ray.direction.z();
                    x.powi(2) + z.powi(2) <= 1.0
                }

                if *closed && local_ray.direction.y().abs() >= EPSILON {
                    let tmin = (min_y - local_ray.origin.y()) / local_ray.direction.y();
                    let tmax = (max_y - local_ray.origin.y()) / local_ray.direction.y();

                    if ray_within_cylinder_at_t(&local_ray, tmin) {
                        intersections.push(Intersection::new(tmin, self.clone()).into());
                    }

                    if ray_within_cylinder_at_t(&local_ray, tmax) {
                        intersections.push(Intersection::new(tmax, self.clone()).into());
                    }
                }

                Intersections::new(intersections)
            }
            ObjectType::DoubleNappedCone(min_y, max_y, closed) => {
                let a = local_ray.direction.x().powi(2) - local_ray.direction.y().powi(2)
                    + local_ray.direction.z().powi(2);

                let b = 2.0 * local_ray.origin.x() * local_ray.direction.x()
                    - 2.0 * local_ray.origin.y() * local_ray.direction.y()
                    + 2.0 * local_ray.origin.z() * local_ray.direction.z();

                let c = local_ray.origin.x().powi(2) - local_ray.origin.y().powi(2)
                    + local_ray.origin.z().powi(2);

                let mut intersections: Vec<Rc<Intersection>> = vec![];

                if a.abs() < EPSILON {
                    if b.abs() >= EPSILON {
                        // ray is parallel to the "pointy part" of the cone
                        let t = -c / (2.0 * b);
                        intersections.push(Intersection::new(t, self.clone()).into());
                    }
                } else {
                    // ray intersects cone "normally"
                    let discriminant = b.powi(2) - 4.0 * a * c;

                    if discriminant < 0.0 {
                        return Intersections::empty();
                    }

                    let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
                    let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

                    let y0 = local_ray.origin.y() + t0 * local_ray.direction.y();
                    let y1 = local_ray.origin.y() + t1 * local_ray.direction.y();

                    if *min_y < y0 && y0 < *max_y {
                        intersections.push(Intersection::new(t0, self.clone()).into());
                    }

                    if *min_y < y1 && y1 < *max_y {
                        intersections.push(Intersection::new(t1, self.clone()).into());
                    }
                }

                fn ray_within_cone_at_t(ray: &Ray, t: RayTracerFloat, y: RayTracerFloat) -> bool {
                    let x = ray.origin.x() + t * ray.direction.x();
                    let z = ray.origin.z() + t * ray.direction.z();
                    x.powi(2) + z.powi(2) <= y.abs()
                }

                if *closed && local_ray.direction.y().abs() >= EPSILON {
                    let tmin = (min_y - local_ray.origin.y()) / local_ray.direction.y();
                    let tmax = (max_y - local_ray.origin.y()) / local_ray.direction.y();

                    if ray_within_cone_at_t(&local_ray, tmin, *min_y) {
                        intersections.push(Intersection::new(tmin, self.clone()).into());
                    }

                    if ray_within_cone_at_t(&local_ray, tmax, *max_y) {
                        intersections.push(Intersection::new(tmax, self.clone()).into());
                    }
                }

                Intersections::new(intersections)
            }
            ObjectType::Group(children) => {
                let mut all_intersections: Vec<Rc<Intersection>> = vec![];

                for c in children {
                    let ints = c.clone().intersections(&local_ray);

                    for c in ints.ints() {
                        all_intersections.push(c.clone());
                    }
                }

                Intersections::new(all_intersections)
            }
        }
    }

    pub fn world_point_to_local(&self, world_point: Point) -> Point {
        let p = if let Some(parent) = self.parent.upgrade() {
            parent.world_point_to_local(world_point)
        } else {
            world_point
        };

        let inverse = self.transform.try_inverse().unwrap();
        p.transform(&inverse)
    }

    fn local_normal_to_world(&self, local_normal: Vector) -> Vector {
        // (0,0,0).norm() == (0/0, 0/0, 0/0) == (NaN, NaN, NaN), so don't try to norm it
        if local_normal == Tuple::origin().to_vector() {
            return local_normal;
        }

        let inverse_transpose = self.transform.try_inverse().unwrap().transpose();
        let transformed_norm = local_normal
            .transform(&inverse_transpose)
            .to_vector()
            .normalize();

        if let Some(parent) = self.parent.upgrade() {
            parent.local_normal_to_world(transformed_norm)
        } else {
            transformed_norm
        }
    }

    pub fn normal_at(&self, p: Point) -> Vector {
        let local_point = self.world_point_to_local(p);

        let local_normal = match self.obj_type {
            ObjectType::Test => local_point.to_vector(),
            ObjectType::Plane => Vector::vector(0., 1., 0.),
            ObjectType::Sphere => local_point - Point::origin(),
            ObjectType::Cube => {
                let x = local_point.x().abs();
                let y = local_point.y().abs();
                let z = local_point.z().abs();
                let maxc = x.max(y.max(z));

                match maxc {
                    _ if maxc == x => Vector::vector(local_point.x(), 0.0, 0.0),
                    _ if maxc == y => Vector::vector(0.0, local_point.y(), 0.0),
                    _ if maxc == z => Vector::vector(0.0, 0.0, local_point.z()),
                    _ => unreachable!(),
                }
            }
            ObjectType::Cylinder(min_y, max_y, _) => {
                let dist = local_point.x().powi(2) + local_point.z().powi(2);

                if dist < 1.0 && local_point.y() > max_y - EPSILON {
                    Vector::vector(0., 1., 0.)
                } else if dist < 1.0 && local_point.y() < min_y + EPSILON {
                    Vector::vector(0., -1., 0.)
                } else {
                    Vector::vector(local_point.x(), 0.0, local_point.z())
                }
            }
            ObjectType::DoubleNappedCone(min_y, max_y, _) => {
                let dist = local_point.x().powi(2) + local_point.z().powi(2);

                if dist < 1.0 && local_point.y() > max_y - EPSILON {
                    Vector::vector(0., 1., 0.)
                } else if dist < 1.0 && local_point.y() < min_y + EPSILON {
                    Vector::vector(0., -1., 0.)
                } else {
                    let y_abs = (local_point.x().powi(2) + local_point.z().powi(2)).sqrt();
                    let y = if local_point.y() > 0.0 { -y_abs } else { y_abs };

                    Vector::vector(local_point.x(), y, local_point.z())
                }
            }
            ObjectType::Group(..) => unimplemented!(),
        };

        self.local_normal_to_world(local_normal)
    }
}

// TODO cfg(test)
pub fn default_test_shape() -> Object {
    Object::test(identity(), Material::default())
}

pub fn default_sphere() -> Object {
    Object::sphere(identity(), Material::default())
}

pub fn default_plane() -> Object {
    Object::plane(identity(), Material::default())
}

pub fn custom_glass_sphere(transform: Transform, refractive: RayTracerFloat) -> Object {
    Object::sphere(
        transform,
        MaterialBuilder::default()
            .transparency(1.0)
            .refractive(refractive)
            .build(),
    )
}

#[cfg(test)]
mod test {
    use std::{
        f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, SQRT_2},
        rc::Rc,
    };

    use approx::assert_abs_diff_eq;

    use crate::{
        material::Material,
        objects::{default_sphere, ObjectType},
        ray::Ray,
        transforms::{identity, rotation, scaling, translation, RotationAxis},
        tuple::{Point, Vector},
        util::{test::glass_sphere, RayTracerFloat, EPSILON},
    };

    use super::{default_test_shape, Object};

    impl ObjectType {
        pub fn children(&self) -> &Vec<Rc<Object>> {
            match self {
                Self::Group(children) => children,
                _ => panic!("not a group"),
            }
        }
    }

    fn default_cube() -> Object {
        Object::cube(identity(), Material::default())
    }

    fn default_cylinder() -> Object {
        Object::cylinder(
            identity(),
            Material::default(),
            -RayTracerFloat::INFINITY,
            RayTracerFloat::INFINITY,
            false,
        )
    }

    fn default_cone() -> Object {
        Object::cone(
            identity(),
            Material::default(),
            -RayTracerFloat::INFINITY,
            RayTracerFloat::INFINITY,
            false,
        )
    }

    #[test]
    fn glass_sphere_properties() {
        let gs = glass_sphere();
        assert_eq!(gs.material.transparency, 1.0);
        assert_eq!(gs.material.refractive, 1.5);
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

    #[test]
    fn ray_misses_cube() {
        let examples = vec![
            Ray::new(
                Point::point(-2.0, 0.0, 0.0),
                Vector::vector(0.2673, 0.5345, 0.8018),
            ),
            Ray::new(
                Point::point(0.0, -2.0, 0.0),
                Vector::vector(0.8018, 0.2673, 0.5345),
            ),
            Ray::new(
                Point::point(0.0, 0.0, -2.0),
                Vector::vector(0.5345, 0.8018, 0.2673),
            ),
            Ray::new(Point::point(2.0, 0.0, 2.0), Vector::vector(0.0, 0.0, -1.0)),
            Ray::new(Point::point(0.0, 2.0, 2.0), Vector::vector(0.0, -1.0, 0.0)),
            Ray::new(Point::point(2.0, 2.0, 0.0), Vector::vector(-1.0, 0.0, 0.0)),
        ];

        let c = Rc::new(default_cube());

        for r in examples {
            let xs = c.clone().intersections(&r);
            assert_eq!(xs.ints().len(), 0);
        }
    }

    #[test]
    fn cube_surface_normal() {
        // (point, normal)
        let examples = vec![
            (Point::point(1.0, 0.5, -0.8), Vector::vector(1., 0., 0.)),
            (Point::point(-1., -0.2, 0.9), Vector::vector(-1., 0., 0.)),
            (Point::point(-0.4, 1., -0.1), Vector::vector(0., 1., 0.)),
            (Point::point(0.3, -1., -0.7), Vector::vector(0., -1., 0.)),
            (Point::point(-0.6, 0.3, 1.), Vector::vector(0., 0., 1.)),
            (Point::point(0.4, 0.4, -1.), Vector::vector(0., 0., -1.)),
            (Point::point(1.0, 1.0, 1.0), Vector::vector(1., 0., 0.)),
            (Point::point(-1., -1., -1.), Vector::vector(-1., 0., 0.)),
        ];

        let c = Rc::new(default_cube());

        for (point, expected) in examples {
            let normal = c.normal_at(point);
            assert_abs_diff_eq!(normal, expected);
        }
    }

    #[test]
    fn ray_misses_cylinder() {
        // (origin, direction)
        let examples = vec![
            (Point::point(1.0, 0.0, 0.0), Vector::vector(0., 1., 0.)),
            (Point::point(0.0, 0.0, 0.0), Vector::vector(0., 1., 0.)),
            (Point::point(1.0, 0.0, -5.0), Vector::vector(1., 1., 1.)),
        ];

        let cyl = Rc::new(default_cylinder());

        for (origin, direction) in examples {
            let norm_direction = direction.normalize();
            let r = Ray::new(origin, norm_direction);
            assert_eq!(cyl.clone().intersections(&r).ints().len(), 0);
        }
    }

    #[test]
    fn ray_hits_cylinder() {
        // (origin, direction, t0, t1)
        let examples = vec![
            (
                Point::point(1.0, 0.0, -5.0),
                Vector::vector(0., 0., 1.),
                5.0,
                5.0,
            ),
            (
                Point::point(0.0, 0.0, -5.0),
                Vector::vector(0., 0., 1.),
                4.0,
                6.0,
            ),
            (
                Point::point(0.5, 0.0, -5.0),
                Vector::vector(0.1, 1., 1.),
                6.80798,
                7.08872,
            ),
        ];

        let cyl = Rc::new(default_cylinder());

        for (origin, direction, t0, t1) in examples {
            let norm_direction = direction.normalize();
            let r = Ray::new(origin, norm_direction);
            let xs = cyl.clone().intersections(&r);
            assert_eq!(xs.ints().len(), 2);
            assert_abs_diff_eq!(xs.ints()[0].t, t0, epsilon = EPSILON);
            assert_abs_diff_eq!(xs.ints()[1].t, t1, epsilon = EPSILON);
        }
    }

    #[test]
    fn cylinder_norm() {
        // (point, normal)
        let examples = vec![
            (Point::point(1.0, 0.0, 0.0), Vector::vector(1., 0., 0.)),
            (Point::point(0.0, 5.0, -1.0), Vector::vector(0., 0., -1.)),
            (Point::point(0.0, -2.0, 1.0), Vector::vector(0., 0., 1.)),
            (Point::point(-1.0, 1.0, 0.0), Vector::vector(-1., 0., 0.)),
        ];

        let cyl = Rc::new(default_cylinder());

        for (point, normal) in examples {
            assert_abs_diff_eq!(cyl.normal_at(point), normal);
        }
    }

    #[test]
    fn constrained_cylinder_intersection() {
        // (point, direction, count)
        let examples = vec![
            (Point::point(0.0, 1.5, 0.0), Vector::vector(0.1, 1., 0.), 0),
            (Point::point(0.0, 3.0, -5.0), Vector::vector(0., 0., 1.), 0),
            (Point::point(0.0, 0.0, -5.0), Vector::vector(0., 0., 1.), 0),
            (Point::point(0.0, 2.0, -5.0), Vector::vector(0., 0., 1.), 0),
            (Point::point(0.0, 1.0, -5.0), Vector::vector(0., 0., 1.), 0),
            (Point::point(0.0, 1.5, -2.0), Vector::vector(0., 0., 1.), 2),
        ];

        let cyl = Rc::new(Object::cylinder(
            identity(),
            Material::default(),
            1.0,
            2.0,
            false,
        ));

        for (point, direction, count) in examples {
            let norm_dir = direction.normalize();
            let r = Ray::new(point, norm_dir);
            assert_eq!(cyl.clone().intersections(&r).ints().len(), count);
        }
    }

    #[test]
    fn intersecting_closed_cyl_caps() {
        // (point, direction, count)
        let examples = vec![
            (Point::point(0.0, 3.0, 0.0), Vector::vector(0., -1., 0.), 2),
            (Point::point(0.0, 3.0, -2.0), Vector::vector(0., -1., 2.), 2),
            (Point::point(0.0, 4.0, -2.0), Vector::vector(0., -1., 1.), 2),
            (Point::point(0.0, 0.0, -2.0), Vector::vector(0., 1., 2.), 2),
            (Point::point(0.0, -1.0, -2.0), Vector::vector(0., 1., 1.), 2),
        ];

        let cyl = Rc::new(Object::cylinder(
            identity(),
            Material::default(),
            1.0,
            2.0,
            true,
        ));

        for (idx, (point, direction, count)) in examples.into_iter().enumerate() {
            let norm_dir = direction.normalize();
            let r = Ray::new(point, norm_dir);
            assert_eq!(
                cyl.clone().intersections(&r).ints().len(),
                count,
                "case {} failed",
                idx
            );
        }
    }

    #[test]
    fn cylinder_cap_norm() {
        // (point, normal)
        let examples = vec![
            (Point::point(0.0, 1.0, 0.0), Vector::vector(0., -1., 0.)),
            (Point::point(0.5, 1.0, 0.0), Vector::vector(0., -1., 0.)),
            (Point::point(0.0, 1.0, 0.5), Vector::vector(0., -1., 0.)),
            (Point::point(0.0, 2.0, 0.0), Vector::vector(0., 1., 0.)),
            (Point::point(0.0, 2.0, 0.0), Vector::vector(0., 1., 0.)),
            (Point::point(0.0, 2.0, 0.5), Vector::vector(0., 1., 0.)),
        ];

        let cyl = Rc::new(Object::cylinder(
            identity(),
            Material::default(),
            1.0,
            2.0,
            true,
        ));

        for (point, normal) in examples {
            assert_abs_diff_eq!(cyl.normal_at(point), normal);
        }
    }

    #[test]
    fn cone_intersection() {
        // (origin, direction, t0, t1)
        let examples = vec![
            (
                Point::point(0.0, 0.0, -5.0),
                Vector::vector(0., 0., 1.),
                5.0,
                5.0,
            ),
            (
                Point::point(0.0, 0.0, -5.0),
                Vector::vector(1.0, 1.0, 1.0),
                8.66025,
                8.66025,
            ),
            (
                Point::point(1.0, 1.0, -5.0),
                Vector::vector(-0.5, -1., 1.),
                4.55006,
                49.44994,
            ),
        ];

        let cone = Rc::new(default_cone());

        for (origin, direction, t0, t1) in examples {
            let norm_direction = direction.normalize();
            let r = Ray::new(origin, norm_direction);
            let xs = cone.clone().intersections(&r);
            assert_eq!(xs.ints().len(), 2);
            assert_abs_diff_eq!(xs.ints()[0].t, t0, epsilon = EPSILON);
            assert_abs_diff_eq!(xs.ints()[1].t, t1, epsilon = EPSILON);
        }
    }

    #[test]
    fn cone_intersection_ray_parallel() {
        let cone = Rc::new(default_cone());
        let r = Ray::new(
            Point::point(0., 0., -1.0),
            Vector::vector(0., 1., 1.).normalize(),
        );
        let xs = cone.intersections(&r);
        assert_eq!(xs.ints().len(), 1);
        assert_abs_diff_eq!(xs.ints()[0].t, 0.35355, epsilon = EPSILON);
    }

    #[test]
    fn intersecting_closed_cone_caps() {
        // (point, direction, count)
        let examples = vec![
            (Point::point(0.0, 0.0, -5.0), Vector::vector(0., 1., 0.), 0),
            (Point::point(0.0, 0.0, -0.25), Vector::vector(0., 1., 1.), 2),
            (Point::point(0.0, 0.0, -0.25), Vector::vector(0., 1., 0.), 4),
        ];

        let cone = Rc::new(Object::cone(
            identity(),
            Material::default(),
            -0.5,
            0.5,
            true,
        ));

        for (idx, (point, direction, count)) in examples.into_iter().enumerate() {
            let norm_dir = direction.normalize();
            let r = Ray::new(point, norm_dir);
            assert_eq!(
                cone.clone().intersections(&r).ints().len(),
                count,
                "case {} failed",
                idx
            );
        }
    }

    #[test]
    fn cone_normal() {
        // (point, normal)
        let examples = vec![
            // the normal at the point in the middle
            (Point::point(0.0, 1.0, 0.0), Vector::vector(0., 0., 0.)),
            (
                Point::point(1.0, 1.0, 1.0),
                Vector::vector(0.5, -SQRT_2 / 2.0, 0.5),
            ),
            (
                Point::point(-1.0, -1.0, 0.0),
                Vector::vector(-FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.),
            ),
        ];

        let cone = default_cone();

        for (point, normal) in examples {
            assert_abs_diff_eq!(cone.normal_at(point), normal);
        }
    }

    #[test]
    fn add_shape_to_group() {
        let t1 = default_test_shape();
        let t2 = default_test_shape();

        let g = Object::group(identity(), vec![t1.into(), t2.into()]);
        assert!(g.obj_type.children().len() == 2);
        assert!(g.parent.upgrade().is_none());

        for c in g.obj_type.children() {
            assert_eq!(c.obj_type, ObjectType::Test);
            assert_eq!(c.parent.upgrade().unwrap(), g);
        }
    }

    #[test]
    fn intersec_with_empty_group() {
        let g = Object::group(identity(), vec![]);
        let r = Ray::new(Point::origin(), Vector::vector(0., 0., 1.));
        assert!(g.intersections(&r).ints().is_empty());
    }

    #[test]
    fn intersect_with_nonempty_group() {
        let s1 = Rc::new(default_sphere());
        let s2 = Rc::new(Object::sphere(
            translation(0.0, 0.0, -3.0),
            Material::default(),
        ));
        let s3 = Rc::new(Object::sphere(
            translation(5.0, 0.0, 0.0),
            Material::default(),
        ));

        let g = Object::group(identity(), vec![s1.clone(), s2.clone(), s3.clone()]);

        let r = Ray::new(Point::point(0.0, 0.0, -5.0), Vector::vector(0.0, 0.0, 1.0));
        let xs = g.intersections(&r);

        let ints = xs.ints();
        assert_eq!(ints.len(), 4);
        assert_eq!(ints[0].object, s2, "ints[0]");
        assert_eq!(ints[1].object, s2, "ints[1]");
        assert_eq!(ints[2].object, s1, "ints[2]");
        assert_eq!(ints[3].object, s1, "ints[3]");
    }

    #[test]
    fn intersect_transformed_group() {
        let s = Rc::new(Object::sphere(
            translation(5.0, 0.0, 0.0),
            Material::default(),
        ));
        let g = Object::group(scaling(2.0, 2.0, 2.0), vec![s]);
        let r = Ray::new(
            Point::point(10.0, 0.0, -10.0),
            Vector::vector(0.0, 0.0, 1.0),
        );
        let xs = g.intersections(&r);
        assert_eq!(xs.ints().len(), 2);
    }

    #[test]
    fn world_point_to_local() {
        let s = Rc::new(Object::sphere(
            translation(5.0, 0.0, 0.0),
            Material::default(),
        ));

        let _group = Object::group(
            rotation(RotationAxis::Y, FRAC_PI_2),
            vec![Object::group(scaling(2.0, 2.0, 2.0), vec![s.clone()])],
        );

        assert_abs_diff_eq!(
            s.world_point_to_local(Point::point(-2.0, 0.0, -10.0)),
            Point::point(0.0, 0.0, -1.0)
        );
    }

    #[test]
    fn local_normal_to_world() {
        let s = Rc::new(Object::sphere(
            translation(5.0, 0.0, 0.0),
            Material::default(),
        ));

        let _group = Object::group(
            rotation(RotationAxis::Y, FRAC_PI_2),
            vec![Object::group(scaling(1.0, 2.0, 3.0), vec![s.clone()])],
        );

        let frac_sqrt_3_3 = 3.0f64.sqrt() / 3.0;
        let local_normal = Vector::vector(frac_sqrt_3_3, frac_sqrt_3_3, frac_sqrt_3_3);
        assert_abs_diff_eq!(
            s.local_normal_to_world(local_normal),
            Vector::vector(0.2857, 0.4286, -0.8571)
        );
    }

    #[test]
    fn normal_for_child_object() {
        let s = Rc::new(Object::sphere(
            translation(5.0, 0.0, 0.0),
            Material::default(),
        ));

        let _group = Object::group(
            rotation(RotationAxis::Y, FRAC_PI_2),
            vec![Object::group(scaling(1.0, 2.0, 3.0), vec![s.clone()])],
        );

        let global_point = Point::point(1.7321, 1.1547, -5.5774);
        assert_abs_diff_eq!(
            s.normal_at(global_point),
            Vector::vector(0.2857, 0.4286, -0.8571)
        );
    }
}
