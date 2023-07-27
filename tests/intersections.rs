use ray_tracer_challenge_rs::intersection::{Intersection, Intersections};

use cucumber::{given, then, when, World};
use futures_lite::future;
use std::rc::Rc;
use testutils::world::RayTracerWorld;

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
        .insert(int_name, Intersection::new(t, Rc::new(*o)));
}

#[given(expr = r"{word} ← intersections\({word}, {word}\)")]
#[when(expr = r"{word} ← intersections\({word}, {word}\)")]
fn when_intersections_created(
    world: &mut RayTracerWorld,
    ints_name: String,
    int1_name: String,
    int2_name: String,
) {
    let int1 = world.get_optional_int(&int1_name).unwrap();
    let int2 = world.get_optional_int(&int2_name).unwrap();
    world.intersectionses.insert(
        ints_name,
        Intersections::new(vec![int1.clone(), int2.clone()]),
    );
}

#[given(expr = r"{word} ← intersections\({word}, {word}, {word}, {word}\)")]
fn given_mega_intersections(
    world: &mut RayTracerWorld,
    ints_name: String,
    int1_name: String,
    int2_name: String,
    int3_name: String,
    int4_name: String,
) {
    let int1 = world.get_optional_int(&int1_name).unwrap();
    let int2 = world.get_optional_int(&int2_name).unwrap();
    let int3 = world.get_optional_int(&int3_name).unwrap();
    let int4 = world.get_optional_int(&int4_name).unwrap();
    world.intersectionses.insert(
        ints_name,
        Intersections::new(vec![int1.clone(), int2.clone(), int3.clone(), int4.clone()]),
    );
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

    world.intersections.insert(hit_name, hit.unwrap());
}

#[then(regex = r"^(\w+).t = (.+)")]
fn assert_t(world: &mut RayTracerWorld, int_name: String, expected_t: f32) {
    let i = world.get_optional_int(&int_name).unwrap();

    assert_eq!(i.t, expected_t);
}

#[then(regex = r"^([\w\d]+) = ([\w\d]+)$")]
fn assert_intersection_eq(world: &mut RayTracerWorld, lhs_name: String, rhs_name: String) {
    let lhs = world.get_optional_int(&lhs_name);
    let rhs = world.get_optional_int(&rhs_name);

    assert_eq!(lhs.unwrap(), rhs.unwrap());
}

#[then(expr = r"{word} is nothing")]
fn assert_no_intersection(world: &mut RayTracerWorld, int_name: String) {
    let i = world.get_optional_int(&int_name);

    assert!(i.is_none());
}

fn main() {
    future::block_on(RayTracerWorld::run("tests/features/intersections.feature"));
}
