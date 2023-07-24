use nalgebra::Matrix4;
use ray_tracer_challenge_rs::intersection::{Intersectable, Intersections};
use ray_tracer_challenge_rs::objects::Sphere;
use ray_tracer_challenge_rs::ray::Ray;
use ray_tracer_challenge_rs::transforms::translation;
use ray_tracer_challenge_rs::tuple::Tuple;

use cucumber::{given, then, when, World};
use futures_lite::future;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Default, World)]
struct SpheresWorld {
    spheres: HashMap<String, Sphere>,
    rays: HashMap<String, Ray>,
    intersectionses: HashMap<String, Intersections>,
    transforms: HashMap<String, Matrix4<f32>>,
}

impl SpheresWorld {
    fn get_sphere_or_panic(&self, sphere_name: &String) -> &Sphere {
        self.spheres
            .get(sphere_name)
            .expect(format!("missing sphere named {}", sphere_name).as_str())
    }

    fn get_mut_sphere_or_panic(&mut self, sphere_name: &String) -> &mut Sphere {
        self.spheres
            .get_mut(sphere_name)
            .expect(format!("missing sphere named {}", sphere_name).as_str())
    }

    fn get_ray_or_panic(&self, ray_name: &String) -> &Ray {
        self.rays
            .get(ray_name)
            .expect(format!("missing ray named {}", ray_name).as_str())
    }

    fn get_intersections_or_panic(&self, int_name: &String) -> &Intersections {
        self.intersectionses
            .get(int_name)
            .expect(format!("missing intersections named {}", int_name).as_str())
    }

    fn get_transform_or_panic(&self, trans_name: &String) -> Matrix4<f32> {
        self.transforms
            .get(trans_name)
            .expect(format!("missing transform named {}", trans_name).as_str())
            .clone()
    }
}

#[given(
    expr = r"{word} ← ray\(point\({float}, {float}, {float}\), vector\({float}, {float}, {float}\)\)"
)]
fn given_a_ray(
    world: &mut SpheresWorld,
    ray_name: String,
    ox: f32,
    oy: f32,
    oz: f32,
    dx: f32,
    dy: f32,
    dz: f32,
) {
    world.rays.insert(
        ray_name,
        Ray::new(Tuple::point(ox, oy, oz), Tuple::vector(dx, dy, dz)),
    );
}

#[given(expr = r"{word} ← sphere\(\)")]
fn given_a_sphere(world: &mut SpheresWorld, sphere_name: String) {
    world.spheres.insert(sphere_name, Sphere::new());
}

#[given(expr = r"{word} ← translation\({float}, {float}, {float}\)")]
fn given_a_translation(world: &mut SpheresWorld, trans_name: String, x: f32, y: f32, z: f32) {
    world.transforms.insert(trans_name, translation(x, y, z));
}

#[when(expr = r"{word} ← intersect\({word}, {word}\)")]
fn when_ray_intersects_sphere(
    world: &mut SpheresWorld,
    int_name: String,
    sphere_name: String,
    ray_name: String,
) {
    let sphere = world.get_sphere_or_panic(&sphere_name);
    let ray = world.get_ray_or_panic(&ray_name);
    world
        .intersectionses
        .insert(int_name, sphere.intersections(ray));
}

#[then(expr = r"{word}.count = {int}")]
fn assert_intersection_count(world: &mut SpheresWorld, int_name: String, expected: usize) {
    let intersects = world.get_intersections_or_panic(&int_name);

    assert!(intersects.ints().len() == expected);
}

#[then(expr = r"{word}[{int}] = {float}")]
fn assert_nth_intersection(world: &mut SpheresWorld, int_name: String, nth: usize, expected: f32) {
    let ints = world.get_intersections_or_panic(&int_name);

    let actual = &ints.ints()[nth];

    assert_eq!(actual.t, expected);
}

#[then(expr = r"{word}.transform = {word}")]
fn assert_transform(world: &mut SpheresWorld, sphere_name: String, trans_name: String) {
    let s = world.get_sphere_or_panic(&sphere_name);

    let t = if trans_name == "identity_matrix" {
        Matrix4::identity()
    } else {
        world.get_transform_or_panic(&trans_name)
    };

    assert_eq!(*s.transform(), t)
}

fn main() {
    future::block_on(SpheresWorld::run("tests/features/spheres.feature"));
}
