use ray_tracer_challenge_rs::intersection::{Intersection, Intersections};
use ray_tracer_challenge_rs::objects::Sphere;

use cucumber::{given, then, when, World};
use futures_lite::future;
use std::rc::Rc;
use testutils::world::RayTracerWorld;

#[given(expr = r"{word} ← sphere\(\)")]
fn given_a_sphere(world: &mut RayTracerWorld, sphere_name: String) {
    world.spheres.insert(sphere_name, Sphere::default());
}

#[given(expr = r"{word} ← intersection\({float}, {word}\)")]
#[when(expr = r"{word} ← intersection\({float}, {word}\)")]
fn when_intersection_created(
    world: &mut RayTracerWorld,
    int_name: String,
    t: f32,
    object_name: String,
) {
    let o = world.get_sphere_or_panic(&object_name);
    world
        .intersections
        .insert(int_name, Some(Intersection::new(t, Rc::new(*o))));
}

#[given(expr = r"{word} ← intersections\({word}, {word}\)")]
#[when(expr = r"{word} ← intersections\({word}, {word}\)")]
fn when_intersections_created(
    world: &mut RayTracerWorld,
    ints_name: String,
    int1_name: String,
    int2_name: String,
) {
    let int1 = world.get_int_or_panic(&int1_name).unwrap();
    let int2 = world.get_int_or_panic(&int2_name).unwrap();
    world
        .intersectionses
        .insert(ints_name, Intersections::new(vec![int1, int2]));
}

#[when(expr = r"{word} ← hit\({word}\)")]
fn when_hit_queried(world: &mut RayTracerWorld, hit_name: String, ints_name: String) {
    let i = world.get_ints_or_panic(&ints_name);
    let maybe_hit = i.hit();

    let hit = if let Some(i) = maybe_hit {
        Some(Intersection::new(i.t, i.object.clone()))
    } else {
        None
    };

    world.intersections.insert(hit_name, hit);
}

#[then(regex = r"^(\w+).t = (.+)")]
fn assert_t(world: &mut RayTracerWorld, int_name: String, expected_t: f32) {
    let i = world.get_int_or_panic(&int_name).unwrap();

    assert_eq!(i.t, expected_t);
}

#[then(regex = r"^(\w+)\.count = (\d+)$")]
fn assert_intersection_count(world: &mut RayTracerWorld, int_name: String, expected: usize) {
    let intersects = world.get_ints_or_panic(&int_name);

    assert_eq!(intersects.ints().len(), expected)
}

#[then(regex = r"^(\w+)\[(\d)\]\.t = (.+)")]
fn assert_nth_intersection(
    world: &mut RayTracerWorld,
    int_name: String,
    nth: usize,
    expected: f32,
) {
    let ints = world.get_ints_or_panic(&int_name);

    let actual = &ints.ints()[nth];

    assert_eq!(actual.t, expected);
}

#[then(regex = r"^([\w\d]+) = ([\w\d]+)$")]
fn assert_intersection_eq(world: &mut RayTracerWorld, lhs_name: String, rhs_name: String) {
    let lhs = world.get_int_or_panic(&lhs_name);
    let rhs = world.get_int_or_panic(&rhs_name);

    assert_eq!(lhs.unwrap(), rhs.unwrap());
}

#[then(expr = r"{word} is nothing")]
fn assert_no_intersection(world: &mut RayTracerWorld, int_name: String) {
    let i = world.get_int_or_panic(&int_name);

    assert!(i.is_none());
}

fn main() {
    future::block_on(RayTracerWorld::run("tests/features/intersections.feature"));
}
