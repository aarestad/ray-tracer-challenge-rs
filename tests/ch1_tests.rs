use ray_tracer_challenge_rs::tuple::Tuple;

use cucumber::{given, then, World};
use std::collections::HashMap;

#[derive(Debug, Default, World)]
pub struct TupleWorld {
    tuples: HashMap<String, Tuple>,
}

#[given(expr = r"{word} ← tuple\({float}, {float}, {float}, {float}\)")]
fn new_tuple(world: &mut TupleWorld, tuple_name: String, x: f32, y: f32, z: f32, w: f32) {
    world.tuples.insert(tuple_name, Tuple { x, y, z, w });
}

#[given(expr = r"{word} ← point\({float}, {float}, {float}\)")]
fn new_point(world: &mut TupleWorld, point_name: String, x: f32, y: f32, z: f32) {
    world.tuples.insert(point_name, Tuple::point(x, y, z));
}

#[given(expr = r"{word} ← vector\({float}, {float}, {float}\)")]
fn new_vector(world: &mut TupleWorld, vector_name: String, x: f32, y: f32, z: f32) {
    world.tuples.insert(vector_name, Tuple::vector(x, y, z));
}

#[then(expr = r"{word}.{word} = {float}")]
fn assert_field(world: &mut TupleWorld, tuple_name: String, field: String, expected: f32) {
    let tuple = world
        .tuples
        .get(&tuple_name)
        .expect(format!("missing tuple named {}", tuple_name).as_str());

    let actual = match field.as_str() {
        "x" => tuple.x,
        "y" => tuple.y,
        "z" => tuple.z,
        "w" => tuple.w,
        _ => panic!("unrecognized field name: {}", field),
    };

    assert!(
        expected == actual,
        "{}.{}: expected {} but got {}",
        tuple_name,
        field,
        expected,
        actual
    );
}

#[then(regex = r"(\w+) is (not )?a (\w+)")]
fn assert_tuple_type(
    world: &mut TupleWorld,
    tuple_name: String,
    negation: String,
    tuple_type: String,
) {
    let tuple = world
        .tuples
        .get(&tuple_name)
        .expect(format!("missing tuple named {}", tuple_name).as_str());

    let mut is_proper_type = match tuple_type.as_str() {
        "point" => tuple.is_point(),
        "vector" => tuple.is_vector(),
        _ => panic!("unrecognized tuple type: {}", tuple_type),
    };

    if negation.len() > 0 {
        is_proper_type = !is_proper_type;
    }

    assert!(
        is_proper_type,
        "expected {} to be a {} but was not",
        tuple_name, tuple_type
    );
}

#[then(expr = r"{word} = tuple\({float}, {float}, {float}, {float}\)")]
fn assert_tuple_equality(
    world: &mut TupleWorld,
    tuple_name: String,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let actual = world
        .tuples
        .get(&tuple_name)
        .expect(format!("missing tuple named {}", tuple_name).as_str());

    let expected = Tuple { x, y, z, w };

    assert!(
        *actual == expected,
        "expected tuple {} to be {:?} but was {:?}",
        tuple_name,
        expected,
        actual,
    )
}

fn main() {
    futures::executor::block_on(TupleWorld::run("tests/features/tuples.feature"));
}
