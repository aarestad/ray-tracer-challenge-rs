use std::collections::HashMap;

use cucumber::{gherkin::Step, given, then, when, World};
use futures_lite::future;
use ray_tracer_challenge_rs::ray::Ray;
use ray_tracer_challenge_rs::tuple::Tuple;

#[derive(Debug, Default, World)]
struct RaysWorld {
    tuples: HashMap<String, Tuple>,
    rays: HashMap<String, Ray>,
}

impl RaysWorld {
    fn get_tuple_or_panic(&self, tuple_name: &String) -> &Tuple {
        self.tuples
            .get(tuple_name)
            .expect(format!("missing tuple named {}", tuple_name).as_str())
    }

    fn get_ray_or_panic(&self, ray_name: &String) -> &Ray {
        self.rays
            .get(ray_name)
            .expect(format!("missing ray named {}", ray_name).as_str())
    }
}

// TODO move to shared lib
#[given(regex = r"(\w+)\s*←\s*((tuple|point|vector).+)")]
fn new_tuple(world: &mut RaysWorld, tuple_name: String, tuple: Tuple) {
    world.tuples.insert(tuple_name, tuple);
}

#[given(
    expr = r"{word} ← ray\(point\({float}, {float}, {float}\), vector\({float}, {float}, {float}\)\)"
)]
fn given_a_ray(
    world: &mut RaysWorld,
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

#[when(expr = r"{word} ← ray\({word}, {word}\)")]
fn when_ray_constructed(
    world: &mut RaysWorld,
    ray_name: String,
    origin_name: String,
    dir_name: String,
) {
    let origin = world.get_tuple_or_panic(&origin_name);
    let direction = world.get_tuple_or_panic(&dir_name);
    world.rays.insert(ray_name, Ray::new(*origin, *direction));
}

#[then(expr = r"{word}.origin = {word}")]
fn assert_origin(world: &mut RaysWorld, ray_name: String, origin_name: String) {
    let ray = world.get_ray_or_panic(&ray_name);
    let origin = world.get_tuple_or_panic(&origin_name);

    assert_eq!(&ray.origin, origin);
}

#[then(expr = r"{word}.direction = {word}")]
fn assert_direction(world: &mut RaysWorld, ray_name: String, direction_name: String) {
    let ray = world.get_ray_or_panic(&ray_name);
    let direction = world.get_tuple_or_panic(&direction_name);

    assert_eq!(&ray.direction, direction);
}

#[then(expr = r"position\({word}, {float}\) = point\({float}, {float}, {float}\)")]
fn assert_position(world: &mut RaysWorld, ray_name: String, t: f32, px: f32, py: f32, pz: f32) {
    let ray = world.get_ray_or_panic(&ray_name);
    let expected = Tuple::point(px, py, pz);
    let actual = ray.position(t);

    assert!(expected.approx_eq(&actual));
}

fn main() {
    future::block_on(RaysWorld::run("tests/features/rays.feature"));
}
