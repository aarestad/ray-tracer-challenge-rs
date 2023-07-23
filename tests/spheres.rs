use ray_tracer_challenge_rs::objects::{Intersectable, Intersection, Sphere};
use ray_tracer_challenge_rs::ray::Ray;
use ray_tracer_challenge_rs::tuple::Tuple;

use cucumber::{given, then, when, World};
use futures_lite::future;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Default, World)]
struct SpheresWorld {
    spheres: HashMap<String, Sphere>,
    rays: HashMap<String, Ray>,
    intersections: HashMap<String, Option<(Intersection, Intersection)>>,
}

impl SpheresWorld {
    fn get_sphere_or_panic(&self, sphere_name: &String) -> &Sphere {
        self.spheres
            .get(sphere_name)
            .expect(format!("missing sphere named {}", sphere_name).as_str())
    }

    fn get_ray_or_panic(&self, ray_name: &String) -> &Ray {
        self.rays
            .get(ray_name)
            .expect(format!("missing ray named {}", ray_name).as_str())
    }

    fn get_intersection_or_panic(
        &self,
        int_name: &String,
    ) -> &Option<(Intersection, Intersection)> {
        self.intersections
            .get(int_name)
            .expect(format!("missing intersection named {}", int_name).as_str())
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
        .intersections
        .insert(int_name, sphere.intersections(ray));
}

#[then(expr = r"{word}.count = {int}")]
fn assert_intersection_count(world: &mut SpheresWorld, int_name: String, expected: usize) {
    let intersect = world.get_intersection_or_panic(&int_name);

    if expected == 0 {
        assert!(intersect.is_none())
    }

    // asserting the length of a tuple is redundant
}

#[then(expr = r"{word}[{int}] = {float}")]
fn assert_nth_intersection(world: &mut SpheresWorld, int_name: String, nth: usize, expected: f32) {
    let (i0, i1) = world
        .get_intersection_or_panic(&int_name)
        .as_ref()
        .expect("no intersection");

    let actual = match nth {
        0 => i0,
        1 => i1,
        _ => panic!("bad nth value"),
    };

    assert_eq!(actual.t, expected);
}

fn main() {
    future::block_on(SpheresWorld::run("tests/features/spheres.feature"));
}
