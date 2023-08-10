use std::rc::Rc;

use ray_tracer_challenge_rs::intersection::Intersections;

use cucumber::{given, then, when, World};
use futures_lite::future;
use testutils::world::RayTracerWorld;
use testutils::RayTracerFloat;

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
        Rc::new(Intersections::new(vec![int1.clone(), int2.clone()])),
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
        Rc::new(Intersections::new(vec![int1.clone(), int2.clone(), int3.clone(), int4.clone()])),
    );
}

#[then(regex = r"^(\w+)\.t = (\d+\.?\d+)")]
fn assert_t(world: &mut RayTracerWorld, int_name: String, expected_t: RayTracerFloat) {
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
