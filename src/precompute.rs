use std::rc::Rc;

use crate::{
    intersection::Intersection,
    objects::Object,
    tuple::{Point, Vector},
    util::RayTracerFloat,
};

#[derive(Debug)]
pub struct Precompute {
    pub t: RayTracerFloat,
    pub object: Rc<dyn Object>,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub reflectv: Vector,
    pub inside: bool,
    pub over_point: Point,
    pub under_point: Point,
    pub n1: RayTracerFloat,
    pub n2: RayTracerFloat,
}

impl Precompute {
    pub fn new(
        i: Rc<Intersection>,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        reflectv: Vector,
        inside: bool,
        over_point: Point,
        under_point: Point,
        n1: RayTracerFloat,
        n2: RayTracerFloat,
    ) -> Self {
        Self {
            t: i.t,
            object: i.object.clone(),
            point,
            eyev,
            normalv,
            reflectv,
            inside,
            over_point,
            under_point,
            n1,
            n2,
        }
    }

    pub fn schlick(&self) -> RayTracerFloat {
        let cos = self.eyev.dot(&self.normalv);

        if self.n1 > self.n2 {
            let n12 = self.n1 / self.n2;
            let sin2_t = n12.powi(2) * (1.0 - cos.powi(2));

            if sin2_t > 1.0 {
                1.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod test {
    use std::{f64::consts::SQRT_2, rc::Rc};

    use crate::{
        intersection::{Intersection, Intersections},
        objects::glass_sphere,
        ray::Ray,
        tuple::{Point, Vector},
    };

    #[test]
    fn schlick_total_internal() {
        let shape = Rc::new(glass_sphere());

        let r = Ray::new(
            Point::point(0., 0., SQRT_2 / 2.),
            Vector::vector(0., 1., 0.),
        );

        let xs = Intersections::new(vec![
            Intersection::new(-SQRT_2 / 2., shape.clone()).into(),
            Intersection::new(SQRT_2 / 2., shape.clone()).into(),
        ]);

        let comps = xs.ints()[1].clone().precompute_with(&r, xs.into());
        assert_eq!(comps.schlick(), 1.0);
    }
}
