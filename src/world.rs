use std::rc::Rc;

use crate::{
    color::Color,
    intersection::{Intersection, Intersections, Precompute},
    light::PointLight,
    material::{Material, MaterialBuilder},
    objects::{Object, Sphere},
    ray::Ray,
    transforms::{identity, scaling},
    tuple::Tuple,
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

    pub fn default_world() -> World {
        World::new(
            vec![
                Rc::new(Sphere::new(
                    identity(),
                    MaterialBuilder::default()
                        .color(Color::new(0.8, 1., 0.6))
                        .diffuse(0.7)
                        .specular(0.2)
                        .build(),
                )),
                Rc::new(Sphere::new(scaling(0.5, 0.5, 0.5), Material::default())),
            ],
            PointLight::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.)),
        )
    }

    pub fn intersects_with(&self, r: &Ray) -> Intersections {
        let mut all_intersections: Vec<Intersection> = vec![];

        for o in &self.objects {
            o.intersections(r)
                .ints()
                .iter()
                .for_each(|i| all_intersections.push(i.clone()));
        }

        all_intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Intersections::new(all_intersections)
    }

    pub fn color_at(&self, comps: Precompute) -> Color {
        comps.intersection.object.material().lighting(
            self.light_source,
            comps.point,
            comps.eyev,
            comps.normalv,
        )
    }
}
