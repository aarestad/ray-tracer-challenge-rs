use cucumber::{given, then, when, World};
use futures_lite::future;
use ray_tracer_challenge_rs::tuple::Tuple;
use testutils::parameters::SingleValue;
use testutils::parameters::{Axis, MathExpr};
use testutils::world::RayTracerWorld;

use ray_tracer_challenge_rs::transforms::*;

#[given(expr = r"{word} ← translation\({float}, {float}, {float}\)")]
fn given_a_translation_matrix(
    world: &mut RayTracerWorld,
    matrix_name: String,
    x: f32,
    y: f32,
    z: f32,
) {
    world.transforms.insert(matrix_name, translation(x, y, z));
}

// TODO generalize with above
#[given(expr = r"{word} ← scaling\({float}, {float}, {float}\)")]
fn given_a_scaling_matrix(world: &mut RayTracerWorld, matrix_name: String, x: f32, y: f32, z: f32) {
    world.transforms.insert(matrix_name, scaling(x, y, z));
}

// TODO generalize with above
#[given(expr = r"{word} ← shearing\({float}, {float}, {float}, {float}, {float}, {float}\)")]
fn given_a_shearing_matrix(
    world: &mut RayTracerWorld,
    matrix_name: String,
    xy: f32,
    xz: f32,
    yx: f32,
    yz: f32,
    zx: f32,
    zy: f32,
) {
    world
        .transforms
        .insert(matrix_name, shearing(xy, xz, yx, yz, zx, zy));
}

#[given(expr = r"{word} ← inverse\({word}\)")]
fn given_an_inverse(world: &mut RayTracerWorld, inverted_matrix_name: String, matrix_name: String) {
    let m = world.get_transform_or_panic(&matrix_name);
    world.transforms.insert(
        inverted_matrix_name,
        m.clone().try_inverse().expect("not invertible!"),
    );
}

#[given(expr = r"{word} ← point\({float}, {float}, {float}\)")]
fn given_a_point(world: &mut RayTracerWorld, point_name: String, x: f32, y: f32, z: f32) {
    world.tuples.insert(point_name, Tuple::point(x, y, z));
}

#[given(expr = r"{word} ← vector\({float}, {float}, {float}\)")]
fn given_a_vector(world: &mut RayTracerWorld, point_name: String, x: f32, y: f32, z: f32) {
    world.tuples.insert(point_name, Tuple::vector(x, y, z));
}

#[given(expr = r"{word} ← rotation_{axis}\({mathexpr}\)")]
fn given_a_rotation(world: &mut RayTracerWorld, matrix_name: String, axis: Axis, r: MathExpr) {
    let matrix = rotation(axis.val(), r.val());
    world.transforms.insert(matrix_name, matrix);
}

#[when(expr = r"{word} ← {word} * {word}")]
fn when_transform_applied(
    world: &mut RayTracerWorld,
    result_point_name: String,
    matrix_name: String,
    point_name: String,
) {
    let m = world.get_transform_or_panic(&matrix_name);
    let p = world.get_tuple_or_panic(&point_name);
    world.tuples.insert(result_point_name, p.transform(m));
}

#[when(expr = r"{word} ← {word} * {word} * {word}")]
fn when_chaining_transforms(
    world: &mut RayTracerWorld,
    result_name: String,
    arg1_name: String,
    arg2_name: String,
    arg3_name: String,
) {
    let arg1 = world.get_transform_or_panic(&arg1_name);
    let arg2 = world.get_transform_or_panic(&arg2_name);
    let arg3 = world.get_transform_or_panic(&arg3_name);

    world.transforms.insert(result_name, arg1 * arg2 * arg3);
}

#[then(expr = r"{word} * {word} = point\({mathexpr}, {mathexpr}, {mathexpr}\)")]
fn assert_point_transform_specified(
    world: &mut RayTracerWorld,
    matrix_name: String,
    point_name: String,
    x: MathExpr,
    y: MathExpr,
    z: MathExpr,
) {
    let lhs = world.get_transform_or_panic(&matrix_name);
    let rhs = world.get_tuple_or_panic(&point_name);
    let expected = Tuple::point(x.val(), y.val(), z.val());

    let actual = rhs.transform(lhs);

    assert!(
        expected.approx_eq(&actual),
        "expected {} but was {}",
        expected,
        actual
    );
}

#[then(expr = r"{word} = point\({mathexpr}, {mathexpr}, {mathexpr}\)")]
fn assert_point_value(
    world: &mut RayTracerWorld,
    point_name: String,
    x: MathExpr,
    y: MathExpr,
    z: MathExpr,
) {
    let expected = Tuple::point(x.val(), y.val(), z.val());
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
    world: &mut RayTracerWorld,
    matrix_name: String,
    tuple_name: String,
    expected_name: String,
) {
    let lhs = world.get_transform_or_panic(&matrix_name);
    let rhs = world.get_tuple_or_panic(&tuple_name);
    let expected = world.get_tuple_or_panic(&expected_name);

    let actual = rhs.transform(lhs);

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
    world: &mut RayTracerWorld,
    matrix_name: String,
    vector_name: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let lhs = world.get_transform_or_panic(&matrix_name);
    let rhs = world.get_tuple_or_panic(&vector_name);
    let expected = Tuple::vector(x, y, z);

    let actual = rhs.transform(lhs);

    assert!(
        expected.approx_eq(&actual),
        "expected {} but was {}",
        expected,
        actual
    );
}

fn main() {
    future::block_on(RayTracerWorld::run(
        "tests/features/transformations.feature",
    ));
}
