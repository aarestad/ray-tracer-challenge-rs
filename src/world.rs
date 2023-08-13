use std::rc::Rc;

use crate::{
    color::{Color, BLACK},
    intersection::{Intersection, Intersections, Precompute},
    light::PointLight,
    material::{Material, MaterialBuilder},
    objects::{Object, Sphere},
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
        let surface = comps.intersection.object.material().lighting(
            comps.intersection.object.as_ref(),
            self.light_source,
            comps.point,
            comps.eyev,
            comps.normalv,
            self.is_shadowed(&comps.over_point),
        );

        let reflected = self.reflected_color_at(comps, remaining);
        surface + reflected
    }

    pub fn color_at(&self, ray: &Ray, remaining: usize) -> Color {
        if let Some(hit) = self.intersects_with(ray).hit() {
            self.shade_hit(&hit.precompute_with(ray), remaining)
        } else {
            BLACK
        }
    }

    pub fn reflected_color_at(&self, comps: &Precompute, remaining: usize) -> Color {
        let reflective = comps.intersection.object.material().reflective;

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

    #[cfg(test)]
    fn objects(&mut self) -> &mut Vec<Rc<dyn Object>> {
        &mut self.objects
    }
}

#[cfg(test)]
mod tests {
    use std::{f64::consts::SQRT_2, rc::Rc};

    use approx::assert_abs_diff_eq;

    use crate::{
        color::{Color, BLACK, WHITE},
        intersection::Intersection,
        light::PointLight,
        material::MaterialBuilder,
        objects::{Object, Plane, Sphere},
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
        let comps = i.precompute_with(&r);
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
        let comps = i.precompute_with(&r);
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
        let comps = i.precompute_with(&r);
        let color = w.shade_hit(&comps, 1);
        assert_abs_diff_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
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

        // this should terminate at some point...
        let c = w.color_at(&r, 1);
        println!("{:?}", c);
    }

    #[test]
    fn reflected_color_max_recusion() {
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
        let comps = i.precompute_with(&r);
        let color = w.reflected_color_at(&comps, 0);
        assert_abs_diff_eq!(color, BLACK);
    }
}
