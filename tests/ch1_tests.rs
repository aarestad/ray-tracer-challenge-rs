use ray_tracer_challenge_rs::tuple::Tuple;

use cucumber::{given, then, World};
use std::collections::HashMap;

#[derive(Debug, Default, World)]
pub struct TupleWorld {
    tuples: HashMap<String, Tuple>,
}

#[given(regex = r"(\w+)\s*←\s*((tuple|point|vector).+)")]
fn new_tuple(world: &mut TupleWorld, tuple_name: String, tuple: Tuple) {
    world.tuples.insert(tuple_name, tuple);
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

#[then(regex = r"^(\w+) = (.+)")]
fn assert_tuple_equality(world: &mut TupleWorld, tuple_name: String, expected: Tuple) {
    let actual = world
        .tuples
        .get(&tuple_name)
        .expect(format!("missing tuple named {}", tuple_name).as_str());

    assert!(
        *actual == expected,
        "expected tuple {} to be {:?} but was {:?}",
        tuple_name,
        expected,
        actual,
    )
}

// #[then(expr = r"a1 + a2 = tuple(1, 1, 6, 1)")]
// fn assert_add()

fn main() {
    futures::executor::block_on(TupleWorld::run("tests/features/tuples.feature"));
}
