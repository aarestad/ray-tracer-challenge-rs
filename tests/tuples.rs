use ray_tracer_challenge_rs::tuple::Tuple;

use cucumber::{given, then, World};
use std::collections::HashMap;

#[derive(Debug, Default, World)]
struct TupleWorld {
    tuples: HashMap<String, Tuple>,
}

impl TupleWorld {
    fn get_tuple_or_panic(&self, tuple_name: &String) -> &Tuple {
        self.tuples
            .get(tuple_name)
            .expect(format!("missing tuple named {}", tuple_name).as_str())
    }
}

#[given(regex = r"(\w+)\s*←\s*((tuple|point|vector).+)")]
fn new_tuple(world: &mut TupleWorld, tuple_name: String, tuple: Tuple) {
    world.tuples.insert(tuple_name, tuple);
}

#[then(expr = r"{word}.{word} = {float}")]
fn assert_field(world: &mut TupleWorld, tuple_name: String, field: String, expected: f32) {
    let tuple = world.get_tuple_or_panic(&tuple_name);

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
    let tuple = world.get_tuple_or_panic(&tuple_name);

    let is_proper_type = match tuple_type.as_str() {
        "point" => tuple.is_point(),
        "vector" => tuple.is_vector(),
        _ => panic!("unrecognized tuple type: {}", tuple_type),
    };

    let negate = negation.len() > 0;

    assert!(
        is_proper_type && !negate || !is_proper_type && negate,
        "expected {} {}to be a {} but was not",
        tuple_name,
        if negate { "not " } else { "" },
        tuple_type
    );
}

#[then(regex = r"^(\w+) = (.+)")]
fn assert_tuple_equality(world: &mut TupleWorld, tuple_name: String, expected: Tuple) {
    let actual = world.get_tuple_or_panic(&tuple_name);

    assert!(
        *actual == expected,
        "expected tuple {} to be {:?} but was {:?}",
        tuple_name,
        expected,
        actual,
    )
}

#[then(expr = r"{word} + {word} = {}")]
fn assert_add(world: &mut TupleWorld, lhs_name: String, rhs_name: String, expected: Tuple) {
    let lhs = world.get_tuple_or_panic(&lhs_name);
    let rhs = world.get_tuple_or_panic(&rhs_name);
    let actual = *lhs + *rhs;

    assert!(
        actual == expected,
        "expected {:?} + {:?} to be {:?} but was {:?}",
        lhs,
        rhs,
        expected,
        actual
    );
}

#[then(expr = r"{word} - {word} = {}")]
fn assert_sub(world: &mut TupleWorld, lhs_name: String, rhs_name: String, expected: Tuple) {
    let lhs = world.get_tuple_or_panic(&lhs_name);
    let rhs = world.get_tuple_or_panic(&rhs_name);
    let actual = *lhs - *rhs;

    assert!(
        actual == expected,
        "expected {:?} - {:?} to be {:?} but was {:?}",
        lhs,
        rhs,
        expected,
        actual
    );
}

#[then(expr = r"-{word} = {}")]
fn assert_neg(world: &mut TupleWorld, tuple_name: String, expected: Tuple) {
    let tuple = world.get_tuple_or_panic(&tuple_name);

    let actual = -*tuple;

    assert!(
        actual == expected,
        "expected -{:?} to be {:?} but was {:?}",
        tuple,
        expected,
        actual
    );
}

fn main() {
    futures::executor::block_on(TupleWorld::run("tests/features/tuples.feature"));
}
