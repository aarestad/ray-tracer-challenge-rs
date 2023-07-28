use ray_tracer_challenge_rs::color::Color;
use ray_tracer_challenge_rs::tuple::Tuple;
use ray_tracer_challenge_rs::util::EPSILON;
use testutils::world::RayTracerWorld;

use approx::{abs_diff_eq, assert_abs_diff_eq};
use cucumber::{then, when, World};
use futures_lite::future;
use std::str::FromStr;

use testutils::parameters::{AddSub, ColorProperty, MulDiv, TupleProperty};

#[then(expr = r"{word}.{tupleproperty} = {float}")]
fn assert_tuple_property(
    world: &mut RayTracerWorld,
    tuple_name: String,
    prop: TupleProperty,
    expected: f32,
) {
    let tuple = world.get_tuple_or_panic(&tuple_name);

    let actual = match prop {
        TupleProperty::X => tuple.x(),
        TupleProperty::Y => tuple.y(),
        TupleProperty::Z => tuple.z(),
        TupleProperty::W => tuple.w(),
    };

    assert_eq!(expected, actual);
}

#[then(regex = r"(\w+) is (not )?a (\w+)")]
fn assert_tuple_type(
    world: &mut RayTracerWorld,
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

#[then(regex = r"^(\w+) = ((tuple|point).+)")]
fn assert_tuple_equality(world: &mut RayTracerWorld, tuple_name: String, expected: Tuple) {
    let actual = world.get_tuple_or_panic(&tuple_name);

    assert!(
        *actual == expected,
        "expected tuple {} to be {:?} but was {:?}",
        tuple_name,
        expected,
        actual,
    )
}

#[then(regex = r"(\w+) (\+|-) (\w+) = ((tuple|point|vector).+)")]
fn assert_addsub(
    world: &mut RayTracerWorld,
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

#[then(regex = r"(\w+) (\*|/) (\d+(?:\.\d+)?) = ((tuple|point|vector).+)")]
fn assert_muldiv(
    world: &mut RayTracerWorld,
    lhs_name: String,
    op: MulDiv,
    rhs: f32,
    expected: Tuple,
) {
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
fn assert_neg(world: &mut RayTracerWorld, tuple_name: String, expected: Tuple) {
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
fn assert_magnitude_with_f32(world: &mut RayTracerWorld, tuple_name: String, expected: f32) {
    let tuple = world.get_tuple_or_panic(&tuple_name);
    let actual = tuple.magnitude();

    assert_abs_diff_eq!(actual, expected, epsilon = EPSILON);
}

#[then(regex = r"normalize\((\w+)\)\s*=\s*(approximately)?\s*(.+)")]
fn assert_normalize_approx(
    world: &mut RayTracerWorld,
    tuple_name: String,
    approx: String,
    expected: Tuple,
) {
    let tuple = world.get_tuple_or_panic(&tuple_name);
    let actual = tuple.normalize();
    let approx_test = approx == "approximately";

    assert!(
        if approx_test {
            abs_diff_eq!(actual, &expected)
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
    world: &mut RayTracerWorld,
    result_tuple_name: String,
    source_tuple_name: String,
) {
    let source_tuple = world.get_tuple_or_panic(&source_tuple_name);
    world
        .tuples
        .insert(result_tuple_name, source_tuple.normalize());
}

#[then(expr = r"dot\({word}, {word}\) = {float}")]
fn assert_dot_product(
    world: &mut RayTracerWorld,
    lhs_name: String,
    rhs_name: String,
    expected: f32,
) {
    let lhs = world.get_tuple_or_panic(&lhs_name);
    let rhs = world.get_tuple_or_panic(&rhs_name);
    let actual = lhs.dot(rhs);

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
    world: &mut RayTracerWorld,
    lhs_name: String,
    rhs_name: String,
    expected: Tuple,
) {
    let lhs = world.get_tuple_or_panic(&lhs_name);
    let rhs = world.get_tuple_or_panic(&rhs_name);
    let actual = lhs.cross(rhs);

    assert!(
        actual == expected,
        "expected {}.cross({}) to be {} but was {}",
        lhs_name,
        rhs_name,
        expected,
        actual
    );
}

#[then(expr = r"{word}.{colorproperty} = {float}")]
fn assert_color_property(
    world: &mut RayTracerWorld,
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

    assert_eq!(expected, actual);
}

#[then(expr = r"{word} {addsub} {word} = color\({float}, {float}, {float}\)")]
fn assert_color_addsub(
    world: &mut RayTracerWorld,
    lhs_name: String,
    op: AddSub,
    rhs_name: String,
    r: f32,
    g: f32,
    b: f32,
) {
    let lhs = world.get_color_or_panic(&lhs_name);
    let rhs = world.get_color_or_panic(&rhs_name);
    let expected = Color::new(r, g, b);
    let actual = match op {
        AddSub::Add => *lhs + *rhs,
        AddSub::Sub => *lhs - *rhs,
    };

    assert_abs_diff_eq!(expected, &actual);
}

#[then(expr = r"{word} * {word} = color\({float}, {float}, {float}\)")]
fn assert_color_mul(
    world: &mut RayTracerWorld,
    lhs_name: String,
    rhs: String,
    r: f32,
    g: f32,
    b: f32,
) {
    let lhs = world.get_color_or_panic(&lhs_name);
    let rhs_f32 = f32::from_str(rhs.as_str());

    let actual = if let Ok(rhs) = rhs_f32 {
        *lhs * rhs
    } else {
        *lhs * *world.get_color_or_panic(&rhs)
    };

    let expected = Color::new(r, g, b);

    assert_abs_diff_eq!(expected, &actual);
}

fn main() {
    future::block_on(RayTracerWorld::run("tests/features/tuples.feature"));
}
