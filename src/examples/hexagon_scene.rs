use std::{f64::consts::{FRAC_PI_6, FRAC_PI_2, FRAC_PI_3}, rc::Rc};

use crate::{objects::Object, transforms::{translation, scaling, rotation, RotationAxis, Transform, identity}, material::Material};

fn hexagon_corner() -> Object {
    Object::sphere(translation(0., 0., -1.) * scaling(0.25, 0.25, 0.25), Material::default())
}

fn hexagon_edge() -> Object {
    let transform = translation(0.0, 0.0, -1.0)
    * rotation(RotationAxis::Y, -FRAC_PI_6)
    * rotation(RotationAxis::Z, -FRAC_PI_2)
    * scaling(0.25, 1.0, 0.25);

    Object::cylinder(transform, Material::default(), 0.0, 1.0, false)
}

fn hexagon_side(transform: Transform) -> Rc<Object> {
    Object::group(transform, vec![hexagon_corner().into(), hexagon_edge().into()])
}

pub fn hexagon() -> Rc<Object> {
    let mut sides: Vec<Rc<Object>> = vec![];

    for n in 0..=5 {
        sides.push(hexagon_side(rotation(RotationAxis::Y, (n as f64) * FRAC_PI_3)));
    }

    Object::group(translation(0.0, 0.8, 0.0) * rotation(RotationAxis::X, -FRAC_PI_6), sides)
}