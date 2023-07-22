use ray_tracer_challenge_rs::tuple::Tuple;

use cucumber::{given, then, Parameter, World};
use futures_lite::future;
use std::{collections::HashMap, ops::AddAssign, str::FromStr};

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

#[derive(Debug, Parameter)]
#[param(regex = r"[+-]")]
enum AddSub {
    Add,
    Sub,
}

#[derive(Debug, Parameter)]
#[param(regex = r"[*/]")]
enum MulDiv {
    Mul,
    Div,
}

#[derive(Debug)]
struct OpParseErr;

impl FromStr for AddSub {
    type Err = OpParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(AddSub::Add),
            "-" => Ok(AddSub::Sub),
            _ => Err(OpParseErr),
        }
    }
}

impl ToString for AddSub {
    fn to_string(&self) -> String {
        match self {
            AddSub::Add => "+".into(),
            AddSub::Sub => "-".into(),
        }
    }
}

impl FromStr for MulDiv {
    type Err = OpParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(MulDiv::Mul),
            "/" => Ok(MulDiv::Div),
            _ => Err(OpParseErr),
        }
    }
}

impl ToString for MulDiv {
    fn to_string(&self) -> String {
        match self {
            MulDiv::Mul => "*".into(),
            MulDiv::Div => "/".into(),
        }
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

#[then(expr = r"{word} {addsub} {word} = {}")]
fn assert_addsub(
    world: &mut TupleWorld,
    lhs_name: String,
    op: AddSub,
    rhs_name: String,
    expected: Tuple,
) {
    let lhs = world.get_tuple_or_panic(&lhs_name);
    let rhs = world.get_tuple_or_panic(&rhs_name);

    let actual = match op {
        AddSub::Add => *lhs + *rhs,
        AddSub::Sub => *lhs - *rhs,
    };

    assert!(
        actual == expected,
        "expected {:?} {:?} {:?} to be {:?} but was {:?}",
        lhs,
        op,
        rhs,
        expected,
        actual
    );
}

#[then(expr = r"{word} {muldiv} {float} = {}")]
fn assert_muldiv(world: &mut TupleWorld, lhs_name: String, op: MulDiv, rhs: f32, expected: Tuple) {
    let lhs = world.get_tuple_or_panic(&lhs_name);

    let actual = match op {
        MulDiv::Mul => *lhs * rhs,
        MulDiv::Div => *lhs / rhs,
    };

    assert!(
        actual == expected,
        "expected {:?} {:?} {:?} to be {:?} but was {:?}",
        lhs,
        op,
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
    future::block_on(TupleWorld::run("tests/features/tuples.feature"));
}
