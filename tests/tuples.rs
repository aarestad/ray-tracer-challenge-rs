use ray_tracer_challenge_rs::color::{self, Color};
use ray_tracer_challenge_rs::tuple::Tuple;
use ray_tracer_challenge_rs::util::approx;

use core::convert::Infallible;
use cucumber::{given, then, when, Parameter, World};
use futures_lite::future;
use std::fmt::{Debug, Display};
use std::{collections::HashMap, fmt::Formatter, str::FromStr};

#[derive(Debug, Default, World)]
struct TupleWorld {
    tuples: HashMap<String, Tuple>,
    colors: HashMap<String, Color>,
}

impl TupleWorld {
    fn get_tuple_or_panic(&self, tuple_name: &String) -> &Tuple {
        self.tuples
            .get(tuple_name)
            .expect(format!("missing tuple named {}", tuple_name).as_str())
    }

    fn get_color_or_panic(&self, color_name: &String) -> &Color {
        self.colors
            .get(color_name)
            .expect(format!("missing color named {}", color_name).as_str())
    }
}

#[derive(Debug, Parameter)]
#[param(regex = r"x|y|z|w")]
enum TupleProperty {
    X,
    Y,
    Z,
    W,
}

impl FromStr for TupleProperty {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => TupleProperty::X,
            "y" => TupleProperty::Y,
            "z" => TupleProperty::Z,
            "w" => TupleProperty::W,
            _ => unreachable!(),
        })
    }
}

impl Display for TupleProperty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

#[derive(Debug, Parameter)]
#[param(regex = r"red|green|blue")]
enum ColorProperty {
    Red,
    Green,
    Blue,
}

impl FromStr for ColorProperty {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "red" => ColorProperty::Red,
            "green" => ColorProperty::Green,
            "blue" => ColorProperty::Blue,
            _ => unreachable!(),
        })
    }
}

impl Display for ColorProperty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

#[derive(Debug, Parameter)]
#[param(regex = r"√(\d+)")]
struct Sqrt(f32);

impl FromStr for Sqrt {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Sqrt(f32::from_str(s).unwrap().sqrt()))
    }
}

impl Display for Sqrt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
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
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(AddSub::Add),
            "-" => Ok(AddSub::Sub),
            _ => unreachable!(),
        }
    }
}

impl Display for AddSub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AddSub::Add => "+",
            AddSub::Sub => "-",
        })
    }
}

impl FromStr for MulDiv {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(MulDiv::Mul),
            "/" => Ok(MulDiv::Div),
            _ => unreachable!(),
        }
    }
}

impl Display for MulDiv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MulDiv::Mul => "*",
            MulDiv::Div => "/",
        })
    }
}

#[given(regex = r"(\w+)\s*←\s*((tuple|point|vector).+)")]
fn new_tuple(world: &mut TupleWorld, tuple_name: String, tuple: Tuple) {
    world.tuples.insert(tuple_name, tuple);
}

#[then(expr = r"{word}.{tupleproperty} = {float}")]
fn assert_tuple_property(
    world: &mut TupleWorld,
    tuple_name: String,
    prop: TupleProperty,
    expected: f32,
) {
    let tuple = world.get_tuple_or_panic(&tuple_name);

    let actual = match prop {
        TupleProperty::X => tuple.x,
        TupleProperty::Y => tuple.y,
        TupleProperty::Z => tuple.z,
        TupleProperty::W => tuple.w,
    };

    assert!(
        expected == actual,
        "{}.{}: expected {} but got {}",
        tuple_name,
        prop,
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
        "expected {} {} {} to be {} but was {}",
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
        "expected -{} to be {} but was {}",
        tuple,
        expected,
        actual
    );
}

#[then(expr = r"magnitude\({word}\) = {float}")]
fn assert_magnitude_with_f32(world: &mut TupleWorld, tuple_name: String, expected: f32) {
    let tuple = world.get_tuple_or_panic(&tuple_name);
    let actual = tuple.magnitude();

    assert!(
        approx(actual, expected),
        "expected magnitude({}) to be {} but was {}",
        tuple,
        expected,
        actual
    );
}

#[then(expr = r"magnitude\({word}\) = {sqrt}")]
fn assert_magnitude_with_sqrt(world: &mut TupleWorld, tuple_name: String, expected: Sqrt) {
    let tuple = world.get_tuple_or_panic(&tuple_name);
    let actual = tuple.magnitude();

    assert!(
        approx(actual, expected.0),
        "expected magnitude({}) to be {} but was {}",
        tuple,
        expected,
        actual
    );
}

#[then(regex = r"normalize\((\w+)\)\s*=\s*(approximately)?\s*(.+)")]
fn assert_normalize_approx(
    world: &mut TupleWorld,
    tuple_name: String,
    approx: String,
    expected: Tuple,
) {
    let tuple = world.get_tuple_or_panic(&tuple_name);
    let actual = tuple.normalize();
    let approx_test = approx == "approximately";

    assert!(
        if approx_test {
            actual.approx_eq(expected)
        } else {
            actual == expected
        },
        "expected normalize({}) to be {}{} but was {}",
        tuple,
        if approx_test { "approximately " } else { "" },
        expected,
        actual
    );
}

#[when(expr = r"{word} ← normalize\({word}\)")]
fn when_normalizing_vec(
    world: &mut TupleWorld,
    result_tuple_name: String,
    source_tuple_name: String,
) {
    let source_tuple = world.get_tuple_or_panic(&source_tuple_name);
    world
        .tuples
        .insert(result_tuple_name, source_tuple.normalize());
}

#[then(expr = r"dot\({word}, {word}\) = {float}")]
fn assert_dot_product(world: &mut TupleWorld, lhs_name: String, rhs_name: String, expected: f32) {
    let lhs = world.get_tuple_or_panic(&lhs_name);
    let rhs = world.get_tuple_or_panic(&rhs_name);
    let actual = lhs.dot(*rhs);

    assert!(
        actual == expected,
        "expected {}.dot({}) to be {} but was {}",
        lhs_name,
        rhs_name,
        expected,
        actual
    );
}

#[then(expr = r"cross\({word}, {word}\) = {}")]
fn assert_cross_product(
    world: &mut TupleWorld,
    lhs_name: String,
    rhs_name: String,
    expected: Tuple,
) {
    let lhs = world.get_tuple_or_panic(&lhs_name);
    let rhs = world.get_tuple_or_panic(&rhs_name);
    let actual = lhs.cross(*rhs);

    assert!(
        actual == expected,
        "expected {}.cross({}) to be {} but was {}",
        lhs_name,
        rhs_name,
        expected,
        actual
    );
}

#[given(expr = r"{word} ← color\({float}, {float}, {float}\)")]
fn given_color(world: &mut TupleWorld, color_name: String, r: f32, g: f32, b: f32) {
    world.colors.insert(color_name, Color::new(r, g, b));
}

#[then(expr = r"{word}.{colorproperty} = {float}")]
fn assert_color_property(
    world: &mut TupleWorld,
    tuple_name: String,
    prop: ColorProperty,
    expected: f32,
) {
    let color = world.get_color_or_panic(&tuple_name);

    let actual = match prop {
        ColorProperty::Red => color.red(),
        ColorProperty::Green => color.green(),
        ColorProperty::Blue => color.blue(),
    };

    assert!(
        expected == actual,
        "{}.{}: expected {} but got {}",
        tuple_name,
        prop,
        expected,
        actual
    );
}

fn main() {
    future::block_on(TupleWorld::run("tests/features/tuples.feature"));
}
