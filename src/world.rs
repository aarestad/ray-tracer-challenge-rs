use std::rc::Rc;

use crate::{
    color::{Color, BLACK, WHITE},
    intersection::{Intersection, Intersections},
    light::PointLight,
    material::{Material, MaterialBuilder},
    objects::{Object, Sphere},
    precompute::Precompute,
    ray::Ray,
    transforms::{identity, scaling},
    tuple::Point,
};

#[derive(Debug, Default)]
pub struct World {
    pub objects: Vec<Rc<dyn Object>>,
    pub light_source: PointLight,
}

impl World {
    pub fn new(objects: Vec<Rc<dyn Object>>, light_source: PointLight) -> Self {
        Self {
            objects,
            light_source,
        }
    }

    pub fn default_world() -> Self {
        World::new(
            vec![
                Rc::new(Sphere::new(
                    identity(),
                    Rc::new(
                        MaterialBuilder::default()
                            .color(Color::new(0.8, 1., 0.6))
                            .diffuse(0.7)
                            .specular(0.2)
                            .build(),
                    ),
                )),
                Rc::new(Sphere::new(
                    scaling(0.5, 0.5, 0.5),
                    Rc::new(Material::default()),
                )),
            ],
            PointLight::new(Point::point(-10., 10., -10.), Color::new(1., 1., 1.)),
        )
    }

    pub fn default_world_with_objects(objects: Vec<Rc<dyn Object>>) -> Self {
        let mut w = Self::default_world();
        w.objects = objects;
        w
    }

    pub fn intersects_with(&self, r: &Ray) -> Rc<Intersections> {
        let mut all_intersections: Vec<Rc<Intersection>> = vec![];

        for o in &self.objects {
            o.clone()
                .intersections(r)
                .ints()
                .iter()
                .for_each(|i| all_intersections.push(i.clone()));
        }

        all_intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Intersections::new(all_intersections).into()
    }

    pub fn is_shadowed(&self, p: &Point) -> bool {
        let v = self.light_source.position - *p;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(*p, direction);
        let intersections = self.intersects_with(&r);
        let h = intersections.hit();

        if let Some(hit) = h {
            hit.t < distance
        } else {
            false
        }
    }

    pub fn shade_hit(&self, comps: &Precompute, remaining: usize) -> Color {
        let surface = comps.object.material().lighting(
            comps.object.as_ref(),
            self.light_source,
            comps.point,
            comps.eyev,
            comps.normalv,
            self.is_shadowed(&comps.over_point),
        );

        let reflected = self.reflected_color_at(comps, remaining);
        let refracted = self.refracted_color_at(comps, remaining);

        let mat = comps.object.material();

        if mat.reflective > 0. && mat.transparency > 0. {
            let reflectance = comps.schlick();
            surface + (reflected * reflectance) + (refracted * (1.0 - reflectance))
        } else {
            surface + reflected + refracted
        }
    }

    pub fn color_at(&self, ray: &Ray, remaining: usize) -> Color {
        let xs = self.intersects_with(ray);

        if let Some(hit) = xs.clone().hit() {
            self.shade_hit(&hit.precompute_with(ray, xs.clone()), remaining)
        } else {
            BLACK
        }
    }

    pub fn reflected_color_at(&self, comps: &Precompute, remaining: usize) -> Color {
        let reflective = comps.object.material().reflective;

        if reflective == 0. || remaining == 0 {
            return BLACK;
        }

        if remaining == 0 {
            BLACK
        } else {
            let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
            let color = self.color_at(&reflect_ray, remaining - 1);
            color * reflective
        }
    }

    pub fn refracted_color_at(&self, comps: &Precompute, remaining: usize) -> Color {
        let n12 = comps.n1 / comps.n2;
        let cos_i = comps.eyev.dot(&comps.normalv);
        let sin2_t = n12.powi(2) * (1. - cos_i.powi(2));

        if remaining == 0 || sin2_t > 1. || comps.object.material().transparency == 0. {
            return BLACK;
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normalv * (n12 * cos_i - cos_t) - comps.eyev * n12;
        let refract_ray = Ray::new(comps.under_point, direction);

        self.color_at(&refract_ray, remaining - 1) * comps.object.material().transparency
    }

    #[cfg(test)]
    fn objects(&mut self) -> &mut Vec<Rc<dyn Object>> {
        &mut self.objects
    }
}

#[cfg(test)]
mod test {
    use std::{f64::consts::SQRT_2, rc::Rc};

    use approx::assert_abs_diff_eq;

    use crate::{
        color::{Color, BLACK, WHITE},
        intersection::{Intersection, Intersections},
        light::PointLight,
        material::{Material, MaterialBuilder},
        objects::{Object, Plane, Sphere},
        patterns::TestPattern,
        ray::Ray,
        transforms::{identity, scaling, translation},
        tuple::{Point, Vector},
    };

    use super::World;

    #[test]
    fn reflected_color_nonreflective_mat() {
        let mut w = World::default_world_with_objects(vec![
            Rc::new(Sphere::new(
                identity(),
                Rc::new(
                    MaterialBuilder::default()
                        .color(Color::new(0.8, 1., 0.6))
                        .diffuse(0.7)
                        .specular(0.2)
                        .build(),
                ),
            )),
            Rc::new(Sphere::new(
                scaling(0.5, 0.5, 0.5),
                Rc::new(MaterialBuilder::default().ambient(1.).build()),
            )),
        ]);

        let i = Rc::new(Intersection::new(1., w.objects()[1].clone()));

        let r = Ray::new(Point::point(0., 0., 0.), Vector::vector(0., 0., 1.));
        let comps = i
            .clone()
            .precompute_with(&r, Rc::new(Intersections::new(vec![i])));
        let color = w.reflected_color_at(&comps, 1);
        assert_eq!(color, BLACK);
    }

    #[test]
    fn reflected_color_reflective_mat() {
        let mut w = World::default_world();

        let s = Rc::new(Plane::new(
            translation(0., -1., 0.),
            Rc::new(MaterialBuilder::default().reflective(0.5).build()),
        ));

        w.objects.push(s.clone());

        let r = Ray::new(
            Point::point(0., 0., -3.),
            Vector::vector(0., -SQRT_2 / 2., SQRT_2 / 2.),
        );

        let i = Rc::new(Intersection::new(SQRT_2, s.clone()));
        let comps = i
            .clone()
            .precompute_with(&r, Rc::new(Intersections::new(vec![i])));
        let color = w.reflected_color_at(&comps, 1);
        assert_abs_diff_eq!(color, Color::new(0.19032, 0.2379, 0.14274));
    }

    #[test]
    fn shade_hit_reflective_mat() {
        let mut w = World::default_world();

        let s = Rc::new(Plane::new(
            translation(0., -1., 0.),
            Rc::new(MaterialBuilder::default().reflective(0.5).build()),
        ));

        w.objects.push(s.clone());

        let r = Ray::new(
            Point::point(0., 0., -3.),
            Vector::vector(0., -SQRT_2 / 2., SQRT_2 / 2.),
        );

        let i = Rc::new(Intersection::new(SQRT_2, s.clone()));
        let comps = i
            .clone()
            .precompute_with(&r, Rc::new(Intersections::new(vec![i])));
        let color = w.shade_hit(&comps, 1);
        assert_abs_diff_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
    }

    #[test]
    fn shade_hit_transparent_mat() {
        let mut w = World::default_world();

        let floor = Rc::new(Plane::new(
            translation(0., -1., 0.),
            Rc::new(
                MaterialBuilder::default()
                    .transparency(0.5)
                    .refractive(1.5)
                    .build(),
            ),
        ));

        let ball = Rc::new(Sphere::new(
            translation(0., -3.5, -0.5),
            Rc::new(
                MaterialBuilder::default()
                    .color(Color::new(1.0, 0.0, 0.0))
                    .ambient(0.5)
                    .build(),
            ),
        ));

        w.objects.push(floor.clone());
        w.objects.push(ball);

        let r = Ray::new(
            Point::point(0., 0., -3.),
            Vector::vector(0., -SQRT_2 / 2., SQRT_2 / 2.),
        );

        let xs = Intersections::new(vec![Intersection::new(SQRT_2, floor).into()]);
        let comps = xs.ints()[0].clone().precompute_with(&r, xs.into());
        assert_abs_diff_eq!(
            w.shade_hit(&comps, 5),
            Color::new(0.93642, 0.68642, 0.68642)
        );
    }

    #[test]
    fn shade_hit_transparent_and_reflective_mat() {
        let mut w = World::default_world();

        let floor = Rc::new(Plane::new(
            translation(0., -1., 0.),
            Rc::new(
                MaterialBuilder::default()
                    .reflective(0.5)
                    .transparency(0.5)
                    .refractive(1.5)
                    .build(),
            ),
        ));

        let ball = Rc::new(Sphere::new(
            translation(0., -3.5, -0.5),
            Rc::new(
                MaterialBuilder::default()
                    .color(Color::new(1.0, 0.0, 0.0))
                    .ambient(0.5)
                    .build(),
            ),
        ));

        w.objects.push(floor.clone());
        w.objects.push(ball);

        let r = Ray::new(
            Point::point(0., 0., -3.),
            Vector::vector(0., -SQRT_2 / 2., SQRT_2 / 2.),
        );

        let xs = Intersections::new(vec![Intersection::new(SQRT_2, floor).into()]);
        let comps = xs.ints()[0].clone().precompute_with(&r, xs.into());
        assert_abs_diff_eq!(
            w.shade_hit(&comps, 5),
            Color::new(0.93391, 0.69643, 0.69243)
        );
    }

    #[test]
    fn color_at_mutually_reflective_surfaces() {
        let mut w = World::default_world();
        w.light_source = PointLight::new(Point::point(0., 0., 0.), WHITE);

        let lower = Rc::new(Plane::new(
            translation(0., -1., 0.),
            Rc::new(MaterialBuilder::default().reflective(1.).build()),
        ));

        let upper = Rc::new(Plane::new(
            translation(0., 1., 0.),
            Rc::new(MaterialBuilder::default().reflective(1.).build()),
        ));

        w.objects.push(lower);
        w.objects.push(upper);

        let r = Ray::new(Point::point(0., 0., 0.), Vector::vector(0., 1., 0.));

        // this should terminate
        let c = w.color_at(&r, 1);
        println!("{:?}", c);
    }

    #[test]
    fn reflected_color_max_recursion() {
        let mut w = World::default_world();

        let p = Rc::new(Plane::new(
            translation(0., -1., 0.),
            Rc::new(MaterialBuilder::default().reflective(0.5).build()),
        ));
        w.objects.push(p.clone());

        let r = Ray::new(
            Point::point(0., 0., -3.),
            Vector::vector(0., -SQRT_2 / 2., SQRT_2 / 2.),
        );

        let i = Rc::new(Intersection::new(SQRT_2, p.clone()));
        let comps = i
            .clone()
            .precompute_with(&r, Rc::new(Intersections::new(vec![i])));
        let color = w.reflected_color_at(&comps, 0);
        assert_abs_diff_eq!(color, BLACK);
    }

    #[test]
    fn refracted_color_opaque_surface() {
        let w = World::default_world();
        let shape = w.objects[0].clone();
        let r = Ray::new(Point::point(0., 0., -5.), Vector::vector(0., 0., 1.));
        let xs = Intersections::new(vec![
            Intersection::new(4., shape.clone()).into(),
            Intersection::new(6., shape.clone()).into(),
        ]);

        let comps = xs.ints()[0].clone().precompute_with(&r, xs.into());
        assert_eq!(w.refracted_color_at(&comps, 5), BLACK);
    }

    #[test]
    fn refracted_color_max_recursion() {
        let shape = Rc::new(Sphere::new(
            identity(),
            Rc::new(
                MaterialBuilder::default()
                    .color(Color::new(0.8, 1., 0.6))
                    .diffuse(0.7)
                    .specular(0.2)
                    .transparency(1.0)
                    .refractive(1.5)
                    .build(),
            ),
        ));

        let shapes: Vec<Rc<dyn Object>> = vec![
            shape.clone(),
            Rc::new(Sphere::new(
                scaling(0.5, 0.5, 0.5),
                Rc::new(Material::default()),
            )),
        ];

        let w = World::default_world_with_objects(shapes);

        let r = Ray::new(Point::point(0., 0., -5.), Vector::vector(0., 0., 1.));

        let xs = Intersections::new(vec![
            Intersection::new(4., shape.clone()).into(),
            Intersection::new(6., shape.clone()).into(),
        ]);

        let comps = xs.ints()[0].clone().precompute_with(&r, xs.into());
        assert_eq!(w.refracted_color_at(&comps, 0), BLACK);
    }

    #[test]
    fn refracted_color_total_internal_refraction() {
        let shape = Rc::new(Sphere::new(
            identity(),
            Rc::new(
                MaterialBuilder::default()
                    .color(Color::new(0.8, 1., 0.6))
                    .diffuse(0.7)
                    .specular(0.2)
                    .transparency(1.0)
                    .refractive(1.5)
                    .build(),
            ),
        ));

        let shapes: Vec<Rc<dyn Object>> = vec![
            shape.clone(),
            Rc::new(Sphere::new(
                scaling(0.5, 0.5, 0.5),
                Rc::new(Material::default()),
            )),
        ];

        let w = World::default_world_with_objects(shapes);

        let r = Ray::new(
            Point::point(0., 0., SQRT_2 / 2.),
            Vector::vector(0., 1., 0.),
        );

        let xs = Intersections::new(vec![
            Intersection::new(-SQRT_2 / 2., shape.clone()).into(),
            Intersection::new(SQRT_2 / 2., shape.clone()).into(),
        ]);

        let comps = xs.ints()[1].clone().precompute_with(&r, xs.into());
        assert_eq!(w.refracted_color_at(&comps, 5), BLACK);
    }

    #[test]
    fn refracted_color_refracted_ray() {
        let shape_a = Rc::new(Sphere::new(
            identity(),
            Rc::new(
                MaterialBuilder::default()
                    .color(Color::new(0.8, 1., 0.6))
                    .diffuse(0.7)
                    .specular(0.2)
                    .ambient(1.0)
                    .pattern(Rc::new(TestPattern::default()))
                    .build(),
            ),
        ));

        let shape_b = Rc::new(Sphere::new(
            scaling(0.5, 0.5, 0.5),
            Rc::new(
                MaterialBuilder::default()
                    .transparency(1.0)
                    .refractive(1.5)
                    .build(),
            ),
        ));

        let shapes: Vec<Rc<dyn Object>> = vec![shape_a.clone(), shape_b.clone()];

        let w = World::default_world_with_objects(shapes);
        let r = Ray::new(Point::point(0., 0., 0.1), Vector::vector(0., 1., 0.));

        let xs = Intersections::new(vec![
            Intersection::new(-0.9899, shape_a.clone()).into(),
            Intersection::new(-0.4899, shape_b.clone()).into(),
            Intersection::new(0.4899, shape_b.clone()).into(),
            Intersection::new(0.9899, shape_a.clone()).into(),
        ]);

        let comps = xs.ints()[2].clone().precompute_with(&r, xs.into());
        assert_abs_diff_eq!(
            w.refracted_color_at(&comps, 5),
            Color::new(0., 0.99888, 0.04725),
        );
    }
}
