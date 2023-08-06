use approx::assert_abs_diff_eq;
use cucumber::{given, then, when, World};
use futures_lite::future;
use ray_tracer_challenge_rs::tuple::Tuple;
use testutils::parameters::Axis;
use testutils::parameters::SingleValue;
use testutils::world::RayTracerWorld;
use testutils::RayTracerFloat;

use ray_tracer_challenge_rs::transforms::*;

// TODO generalize with above
#[given(expr = r"{word} ← shearing\({float}, {float}, {float}, {float}, {float}, {float}\)")]
fn given_a_shearing_matrix(
    world: &mut RayTracerWorld,
    matrix_name: String,
    xy: RayTracerFloat,
    xz: RayTracerFloat,
    yx: RayTracerFloat,
    yz: RayTracerFloat,
    zx: RayTracerFloat,
    zy: RayTracerFloat,
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

#[given(expr = r"{word} ← rotation_{axis}\({float}\)")]
fn given_a_rotation(world: &mut RayTracerWorld, matrix_name: String, axis: Axis, r: RayTracerFloat) {
    let matrix = rotation(axis.val(), r);
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
    let p = world.get_point_or_panic(&point_name);
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

#[then(expr = r"{word} * {word} = point\({float}, {float}, {float}\)")]
fn assert_point_transform_specified(
    world: &mut RayTracerWorld,
    matrix_name: String,
    point_name: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
) {
    let lhs = world.get_transform_or_panic(&matrix_name);
    let rhs = world.get_point_or_panic(&point_name);
    let expected = Tuple::point(x, y, z);

    let actual = rhs.transform(lhs);

    assert_abs_diff_eq!(expected, &actual);
}

#[then(expr = r"{word} = point\({float}, {float}, {float}\)")]
fn assert_point_value(world: &mut RayTracerWorld, point_name: String, x: RayTracerFloat, y: RayTracerFloat, z: RayTracerFloat) {
    let expected = Tuple::point(x, y, z);
    let actual = world.get_point_or_panic(&point_name);

    assert_abs_diff_eq!(expected, &actual,);
}

#[then(expr = r"{word} * {word} = {word}")]
fn assert_point_transform_name(
    world: &mut RayTracerWorld,
    matrix_name: String,
    tuple_name: String,
    expected_name: String,
) {
    let lhs = world.get_transform_or_panic(&matrix_name);
    let rhs = world.get_point_or_panic(&tuple_name);
    let expected = world.get_point_or_panic(&expected_name);

    let actual = rhs.transform(lhs);

    assert_abs_diff_eq!(expected, &actual);
}

// TODO generalize with above
#[then(expr = r"{word} * {word} = vector\({float}, {float}, {float}\)")]
fn assert_vector_transform_specified(
    world: &mut RayTracerWorld,
    matrix_name: String,
    vector_name: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
) {
    let lhs = world.get_transform_or_panic(&matrix_name);
    let rhs = world.get_point_or_panic(&vector_name);
    let expected = Tuple::vector(x, y, z);

    let actual = rhs.transform(lhs);

    assert_abs_diff_eq!(expected, &actual);
}

fn main() {
    future::block_on(RayTracerWorld::run(
        "tests/features/transformations.feature",
    ));
}
