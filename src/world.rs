use std::rc::Rc;

use crate::{
    color::Color,
    light::PointLight,
    material::{Material, MaterialBuilder},
    objects::{Object, Sphere},
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
}
