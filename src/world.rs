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

    pub fn intersects_with(self: &Rc<Self>, r: &Ray) -> Rc<Intersections> {
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

    pub fn is_shadowed(self: &Rc<Self>, p: &Point) -> bool {
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

    pub fn shade_hit(self: &Rc<Self>, comps: &Precompute) -> Color {
        comps.intersection.object.material().lighting(
            comps.intersection.object.as_ref(),
            self.light_source,
            comps.point,
            comps.eyev,
            comps.normalv,
            self.is_shadowed(&comps.over_point),
        )
    }

    pub fn color_at(self: &Rc<Self>, ray: &Ray) -> Color {
        if let Some(hit) = self.intersects_with(ray).hit() {
            self.shade_hit(&hit.precompute_with(ray))
        } else {
            BLACK
        }
    }
}
