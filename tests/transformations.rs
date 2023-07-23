use core::convert::Infallible;
use cucumber::{gherkin::Step, given, then, when, Parameter, World};
use futures_lite::future;
use nalgebra::Matrix4;
use ray_tracer_challenge_rs::tuple::Tuple;
use std::f32::consts::PI;
use std::{collections::HashMap, str::FromStr};

use ray_tracer_challenge_rs::transforms::*;

#[derive(Debug, Parameter)]
#[param(regex = r"(-?(π|√\d+)\s*/\s*\d+)|(-?\d+)")]
struct MathExpr(f32);

impl FromStr for MathExpr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = f32::from_str(s) {
            return Ok(MathExpr(val));
        }

        let mut chars = s.chars();
        let negate = chars.next().expect("empty string?") == '-';

        let rest = if negate {
            chars.collect::<String>()
        } else {
            s.to_string()
        };

        let parts = rest.split("/").map(|s| s.trim()).collect::<Vec<&str>>();

        let [dividend_str, divisor_str] = parts[0..2] else {
          return Err("bad format".to_string());
        };

        let dividend = match dividend_str {
            "π" => PI,
            d if dividend_str.starts_with("√") => {
                let operand = d.chars().skip(1).collect::<String>();
                (f32::from_str(&operand).expect(&format!("bad dividend: {}", dividend_str))).sqrt()
            }
            _ => return Err(format!("bad dividend: {}", dividend_str)),
        };

        let divisor = f32::from_str(&divisor_str).expect(&format!("bad divisor: {}", divisor_str));

        let result = if negate {
            -dividend / divisor
        } else {
            dividend / divisor
        };

        Ok(MathExpr(result))
    }
}

#[derive(Debug, Parameter)]
#[param(regex = r"x|y|z")]
struct Axis(RotationAxis);

impl FromStr for Axis {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Axis(match s {
            "x" => RotationAxis::X,
            "y" => RotationAxis::Y,
            "z" => RotationAxis::Z,
            _ => unreachable!(),
        }))
    }
}

#[derive(Debug, Default, World)]
struct TransformationsWorld {
    matrices: HashMap<String, Matrix4<f32>>,
    tuples: HashMap<String, Tuple>,
}

impl TransformationsWorld {
    fn get_matrix_or_panic(&self, matrix_name: &String) -> &Matrix4<f32> {
        self.matrices
            .get(matrix_name)
            .expect(format!("missing array {}", matrix_name).as_str())
    }

    fn get_tuple_or_panic(&self, tuple_name: &String) -> &Tuple {
        self.tuples
            .get(tuple_name)
            .expect(format!("missing tuple {}", tuple_name).as_str())
    }
}

#[given(expr = r"{word} ← translation\({float}, {float}, {float}\)")]
fn given_a_translation_matrix(
    world: &mut TransformationsWorld,
    matrix_name: String,
    x: f32,
    y: f32,
    z: f32,
) {
    world.matrices.insert(matrix_name, translation(x, y, z));
}

// TODO generalize with above
#[given(expr = r"{word} ← scaling\({float}, {float}, {float}\)")]
fn given_a_scaling_matrix(
    world: &mut TransformationsWorld,
    matrix_name: String,
    x: f32,
    y: f32,
    z: f32,
) {
    world.matrices.insert(matrix_name, scaling(x, y, z));
}

// TODO generalize with above
#[given(expr = r"{word} ← shearing\({float}, {float}, {float}, {float}, {float}, {float}\)")]
fn given_a_shearing_matrix(
    world: &mut TransformationsWorld,
    matrix_name: String,
    xy: f32,
    xz: f32,
    yx: f32,
    yz: f32,
    zx: f32,
    zy: f32,
) {
    world
        .matrices
        .insert(matrix_name, shearing(xy, xz, yx, yz, zx, zy));
}

#[given(expr = r"{word} ← inverse\({word}\)")]
fn given_an_inverse(
    world: &mut TransformationsWorld,
    inverted_matrix_name: String,
    matrix_name: String,
) {
    let m = world.get_matrix_or_panic(&matrix_name);
    world.matrices.insert(
        inverted_matrix_name,
        m.clone().try_inverse().expect("not invertible!"),
    );
}

#[given(expr = r"{word} ← point\({float}, {float}, {float}\)")]
fn given_a_point(world: &mut TransformationsWorld, point_name: String, x: f32, y: f32, z: f32) {
    world.tuples.insert(point_name, Tuple::point(x, y, z));
}

#[given(expr = r"{word} ← vector\({float}, {float}, {float}\)")]
fn given_a_vector(world: &mut TransformationsWorld, point_name: String, x: f32, y: f32, z: f32) {
    world.tuples.insert(point_name, Tuple::vector(x, y, z));
}

#[given(expr = r"{word} ← rotation_{axis}\({mathexpr}\)")]
fn given_a_rotation(
    world: &mut TransformationsWorld,
    matrix_name: String,
    axis: Axis,
    r: MathExpr,
) {
    let matrix = rotation(axis.0, r.0);
    world.matrices.insert(matrix_name, matrix);
}

#[when(expr = r"{word} ← {word} * {word}")]
fn when_transform_applied(
    world: &mut TransformationsWorld,
    result_point_name: String,
    matrix_name: String,
    point_name: String,
) {
    let m = world.get_matrix_or_panic(&matrix_name);
    let p = world.get_tuple_or_panic(&point_name);
    world.tuples.insert(result_point_name, p.rhs_mult(m));
}

#[when(expr = r"{word} ← {word} * {word} * {word}")]
fn when_chaining_transforms(
    world: &mut TransformationsWorld,
    result_name: String,
    arg1_name: String,
    arg2_name: String,
    arg3_name: String,
) {
    let arg1 = world.get_matrix_or_panic(&arg1_name);
    let arg2 = world.get_matrix_or_panic(&arg2_name);
    let arg3 = world.get_matrix_or_panic(&arg3_name);

    world.matrices.insert(result_name, arg1 * arg2 * arg3);
}

#[then(expr = r"{word} * {word} = point\({mathexpr}, {mathexpr}, {mathexpr}\)")]
fn assert_point_transform_specified(
    world: &mut TransformationsWorld,
    matrix_name: String,
    point_name: String,
    x: MathExpr,
    y: MathExpr,
    z: MathExpr,
) {
    let lhs = world.get_matrix_or_panic(&matrix_name);
    let rhs = world.get_tuple_or_panic(&point_name);
    let expected = Tuple::point(x.0, y.0, z.0);

    let actual = rhs.rhs_mult(lhs);

    assert!(
        expected.approx_eq(&actual),
        "expected {} but was {}",
        expected,
        actual
    );
}

#[then(expr = r"{word} = point\({mathexpr}, {mathexpr}, {mathexpr}\)")]
fn assert_point_value(
    world: &mut TransformationsWorld,
    point_name: String,
    x: MathExpr,
    y: MathExpr,
    z: MathExpr,
) {
    let expected = Tuple::point(x.0, y.0, z.0);
    let actual = world.get_tuple_or_panic(&point_name);

    assert!(
        expected.approx_eq(&actual),
        "expected {} but was {}",
        expected,
        actual
    );
}

#[then(expr = r"{word} * {word} = {word}")]
fn assert_point_transform_name(
    world: &mut TransformationsWorld,
    matrix_name: String,
    tuple_name: String,
    expected_name: String,
) {
    let lhs = world.get_matrix_or_panic(&matrix_name);
    let rhs = world.get_tuple_or_panic(&tuple_name);
    let expected = world.get_tuple_or_panic(&expected_name);

    let actual = rhs.rhs_mult(lhs);

    assert!(
        expected.approx_eq(&actual),
        "expected {} but was {}",
        expected,
        actual
    );
}

// TODO generalize with above
#[then(expr = r"{word} * {word} = vector\({float}, {float}, {float}\)")]
fn assert_vector_transform_specified(
    world: &mut TransformationsWorld,
    matrix_name: String,
    vector_name: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let lhs = world.get_matrix_or_panic(&matrix_name);
    let rhs = world.get_tuple_or_panic(&vector_name);
    let expected = Tuple::vector(x, y, z);

    let actual = rhs.rhs_mult(lhs);

    assert!(
        expected.approx_eq(&actual),
        "expected {} but was {}",
        expected,
        actual
    );
}

fn main() {
    future::block_on(TransformationsWorld::run(
        "tests/features/transformations.feature",
    ));
}
