use crate::{
    objects::Object,
    precompute::Precompute,
    ray::Ray,
    util::{RayTracerFloat, EPSILON},
};
use std::{fmt::Debug, rc::Rc};

#[derive(Debug)]
pub struct Intersection {
    pub t: RayTracerFloat,
    pub object: Rc<Object>,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl Intersection {
    pub fn new(t: RayTracerFloat, object: Rc<Object>) -> Self {
        Self { t, object }
    }

    pub fn precompute_with(self: Rc<Self>, r: &Ray, xs: Rc<Intersections>) -> Precompute {
        let world_point = r.position(self.t);
        let eyev = -r.direction;
        let normalv = self.object.normal_at(world_point);
        let inside = normalv.dot(&eyev) < 0.;
        let over_point = world_point + normalv * EPSILON;
        let under_point = world_point - (if inside { -normalv } else { normalv }) * EPSILON;
        let reflectv = r.direction.reflect(&normalv);

        let mut containers: Vec<Rc<Object>> = vec![];

        let mut n1 = 1.0;
        let mut n2 = 1.0;

        for i in xs.as_ref().ints() {
            let is_hit = self.as_ref() == i.as_ref();

            if is_hit {
                if containers.is_empty() {
                    n1 = 1.0;
                } else {
                    n1 = containers.last().unwrap().material().refractive;
                }
            }

            if containers.contains(&i.object) {
                containers.retain(|o| o.as_ref() != i.object.as_ref());
            } else {
                containers.push(i.object.clone());
            }

            if is_hit {
                if containers.is_empty() {
                    n2 = 1.0;
                } else {
                    n2 = containers.last().unwrap().material().refractive;
                }

                break;
            }
        }

        Precompute::new(
            self.clone(),
            world_point,
            eyev,
            if inside { -normalv } else { normalv },
            reflectv,
            inside,
            over_point,
            under_point,
            n1,
            n2,
        )
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Intersections(Vec<Rc<Intersection>>);

impl Intersections {
    pub fn new(intersections: Vec<Rc<Intersection>>) -> Intersections {
        Intersections(intersections)
    }

    pub const fn empty() -> Intersections {
        Intersections(vec![])
    }

    pub const fn ints(&self) -> &Vec<Rc<Intersection>> {
        &self.0
    }

    pub fn hit(self: Rc<Self>) -> Option<Rc<Intersection>> {
        let mut nonnegative_t_ints: Vec<Rc<Intersection>> =
            self.0.iter().filter(|i| i.t >= 0.).cloned().collect();

        if nonnegative_t_ints.is_empty() {
            return None;
        }

        nonnegative_t_ints.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Some(nonnegative_t_ints.first().unwrap().clone())
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::SQRT_2;
    use std::rc::Rc;

    use approx::assert_abs_diff_eq;

    use crate::{
        objects::custom_glass_sphere,
        objects::default_plane,
        ray::Ray,
        transforms::{scaling, translation},
        tuple::{Point, Vector},
        util::{RayTracerFloat, EPSILON},
    };

    use super::{Intersection, Intersections};

    #[test]
    fn precompute_reflectv() {
        let o = default_plane();
        let r = Ray::new(
            Point::point(0., 1., -1.),
            Vector::vector(0., -SQRT_2 / 2., SQRT_2 / 2.),
        );
        let i = Rc::new(Intersection::new(SQRT_2, Rc::new(o)));
        let comps = i
            .clone()
            .precompute_with(&r, Rc::new(Intersections::new(vec![i])));
        assert_abs_diff_eq!(comps.reflectv, Vector::vector(0., SQRT_2 / 2., SQRT_2 / 2.));
    }

    #[test]
    fn precompute_n1_n2() {
        // (intersection_idx, expected_n1, expected_n2)
        let examples: Vec<(usize, RayTracerFloat, RayTracerFloat)> = vec![
            (0, 1.0, 1.5),
            (1, 1.5, 2.0),
            (2, 2.0, 2.5),
            (3, 2.5, 2.5),
            (4, 2.5, 1.5),
            (5, 1.5, 1.0),
        ];

        let gs_a = Rc::new(custom_glass_sphere(scaling(2., 2., 2.), 1.5));
        let gs_b = Rc::new(custom_glass_sphere(translation(0., 0., -0.25), 2.0));
        let gs_c = Rc::new(custom_glass_sphere(translation(0., 0., 0.25), 2.5));

        let r = Ray::new(Point::point(0., 0., -4.), Vector::vector(0., 0., 1.));

        let xs = Rc::new(Intersections::new(vec![
            Rc::new(Intersection::new(2.0, gs_a.clone())),
            Rc::new(Intersection::new(2.75, gs_b.clone())),
            Rc::new(Intersection::new(3.25, gs_c.clone())),
            Rc::new(Intersection::new(4.75, gs_b)),
            Rc::new(Intersection::new(5.25, gs_c)),
            Rc::new(Intersection::new(6.0, gs_a)),
        ]));

        for example in examples {
            let comps = xs.ints()[example.0].clone().precompute_with(&r, xs.clone());
            assert_abs_diff_eq!(comps.n1, example.1);
            assert_abs_diff_eq!(comps.n2, example.2);
        }
    }

    #[test]
    fn precompute_under_point() {
        let r = Ray::new(Point::point(0., 0., -5.), Vector::vector(0., 0., 1.));
        let shape = Rc::new(custom_glass_sphere(translation(0., 0., -0.25), 2.0));
        let i = Rc::new(Intersection::new(5.0, shape));
        let xs = Intersections::new(vec![i.clone()]);
        let comps = i.precompute_with(&r, xs.into());
        assert!(comps.under_point.z() > EPSILON / 2.);
        assert!(comps.point.z() < comps.under_point.z());
    }
}
