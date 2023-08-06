use cucumber::{gherkin::Step, given, then, World};
use futures_lite::future;
use nalgebra::{DMatrix, Matrix4};
use ray_tracer_challenge_rs::{tuple::Point, util::EPSILON};
use testutils::step::get_matrix_from_step;
use testutils::world::RayTracerWorld;
use testutils::RayTracerFloat;

use approx::assert_abs_diff_eq;

#[given(expr = r"the following {int}x{int} matrix {word}:")]
fn given_a_matrix(world: &mut RayTracerWorld, step: &Step, rows: usize, cols: usize, name: String) {
    world
        .matrices
        .insert(name, get_matrix_from_step(step, rows, cols));
}

#[given(expr = r"{word} ← inverse\({word}\)")]
fn given_an_inverse(world: &mut RayTracerWorld, inv_name: String, matrix_name: String) {
    let m = world.get_matrix_or_panic(&matrix_name);
    world
        .matrices
        .insert(inv_name, m.clone().try_inverse().expect("not invertible!"));
}

#[given(expr = r"{word} ← {word} * {word}")]
fn given_matrix_mul(
    world: &mut RayTracerWorld,
    product_name: String,
    lhs_name: String,
    rhs_name: String,
) {
    let lhs = world.get_matrix_or_panic(&lhs_name);
    let rhs = world.get_matrix_or_panic(&rhs_name);
    world.matrices.insert(product_name, lhs * rhs);
}

#[then(expr = r"{word}[{int},{int}] = {float}")]
fn assert_entry_value(
    world: &mut RayTracerWorld,
    name: String,
    row: usize,
    col: usize,
    expected: RayTracerFloat,
) {
    let matrix = world.get_matrix_or_panic(&name);
    let num_cols = matrix.column_iter().count();
    let actual = matrix[row + col * num_cols];

    assert_eq!(actual, expected);
}

#[then(expr = r"{word}[{int},{int}] = {float}\/{float}")]
fn assert_entry_value_fraction(
    world: &mut RayTracerWorld,
    name: String,
    row: usize,
    col: usize,
    expected_num: RayTracerFloat,
    expected_denom: RayTracerFloat,
) {
    let matrix = world.get_matrix_or_panic(&name);
    let num_cols = matrix.column_iter().count();
    let actual = matrix[row + col * num_cols];
    let expected = expected_num / expected_denom;

    assert_abs_diff_eq!(actual, expected, epsilon = EPSILON);
}

#[then(regex = r"^(\w+) (!)?= (\w+)$")]
fn assert_matrix_equality(
    world: &mut RayTracerWorld,
    lhs_name: String,
    negation: String,
    rhs_name: String,
) {
    let lhs = world.get_matrix_or_panic(&lhs_name);
    let rhs = world.get_matrix_or_panic(&rhs_name);
    let negate = negation == "!";

    assert!(
        if negate { lhs != rhs } else { lhs == rhs },
        "expected {} {}= {} but were not",
        lhs_name,
        if negate { "!" } else { "" },
        rhs_name
    );
}

#[then(expr = r"{word} * {word} is the following {int}x{int} matrix:")]
fn assert_matrix_mult(
    world: &mut RayTracerWorld,
    step: &Step,
    lhs_name: String,
    rhs_name: String,
    rows: usize,
    cols: usize,
) {
    let lhs = world.get_matrix_or_panic(&lhs_name);
    let rhs = world.get_matrix_or_panic(&rhs_name);
    let expected = get_matrix_from_step(step, rows, cols);
    let actual = lhs * rhs;

    assert_eq!(expected, actual);
}

#[then(regex = r"(\w+) \* (\w+) = tuple\((\d+), (\d+), (\d+), (\d+)\)")]
fn assert_matrix_tuple_mult(
    world: &mut RayTracerWorld,
    lhs_name: String,
    rhs_name: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
    w: RayTracerFloat,
) {
    let lhs = world.get_matrix_or_panic(&lhs_name);
    let rhs = world.get_point_or_panic(&rhs_name);
    let expected = Point::point(x, y, z);
    let actual = rhs.transform(&Matrix4::from_vec(lhs.data.as_vec().clone()));

    assert_eq!(expected, actual);
    assert_eq!(w, actual.w());
}

#[then(expr = r"{word} * identity_matrix = {word}")]
fn assert_matrix_identity_mult_rhs(
    world: &mut RayTracerWorld,
    lhs_name: String,
    expected_name: String,
) {
    assert_eq!(lhs_name, expected_name, "expecting the same name");
    let m = world.get_matrix_or_panic(&lhs_name);
    let matrix_dim = m.column_iter().count();
    let identity = DMatrix::<RayTracerFloat>::identity(matrix_dim, matrix_dim);

    assert_eq!(*m, m * identity);
}

#[then(expr = r"identity_matrix * {word} = {word}")]
fn assert_tuple_identity_mult_lhs(
    world: &mut RayTracerWorld,
    rhs_name: String,
    expected_name: String,
) {
    assert_eq!(rhs_name, expected_name, "expecting the same name");
    let t = world.get_point_or_panic(&rhs_name);
    let identity = Matrix4::<RayTracerFloat>::identity();

    assert_eq!(*t, t.transform(&identity));
}

#[then(expr = r"transpose\({word}\) is the following {int}x{int} matrix:")]
fn assert_transpose(
    world: &mut RayTracerWorld,
    step: &Step,
    matrix_name: String,
    rows: usize,
    cols: usize,
) {
    let original_matrix = world.get_matrix_or_panic(&matrix_name);
    let expected = get_matrix_from_step(step, rows, cols);
    assert_eq!(original_matrix.transpose(), expected);
}

#[then(expr = r"determinant\({word}\) = {float}")]
fn assert_determinant(world: &mut RayTracerWorld, matrix_name: String, expected: RayTracerFloat) {
    let m = world.get_matrix_or_panic(&matrix_name);

    // bit looser of an epsilon here
    assert_abs_diff_eq!(m.determinant(), expected, epsilon = 0.001);
}

#[then(regex = r"(\w+) is (not )?invertible")]
fn assert_invertible(world: &mut RayTracerWorld, matrix_name: String, invert: String) {
    let m = world.get_matrix_or_panic(&matrix_name);
    let inv = invert.starts_with("not");
    let actual = if inv {
        !m.is_invertible()
    } else {
        m.is_invertible()
    };
    assert!(
        actual,
        "expected {}{}.invertible() to be true but was false",
        if inv { "!" } else { "" },
        matrix_name
    );
}

#[then(expr = r"inverse\({word}\) is the following {int}x{int} matrix:")]
fn assert_inverse(
    world: &mut RayTracerWorld,
    step: &Step,
    matrix_name: String,
    rows: usize,
    cols: usize,
) {
    let m = world.get_matrix_or_panic(&matrix_name);
    let expected = get_matrix_from_step(step, rows, cols);

    assert_abs_diff_eq!(
        &m.clone().try_inverse().expect("not invertible!"),
        &expected,
        epsilon = EPSILON,
    );
}

#[then(regex = r"^matrix (\w+) is the following (\d)x(\d) matrix:")]
fn assert_given_matrix_equality(
    world: &mut RayTracerWorld,
    step: &Step,
    matrix_name: String,
    rows: usize,
    cols: usize,
) {
    let actual = world.get_matrix_or_panic(&matrix_name);
    let expected = get_matrix_from_step(step, rows, cols);

    assert_abs_diff_eq!(actual, &expected, epsilon = EPSILON);
}

#[then(expr = r"{word} * inverse\({word}\) = {word}")]
fn assert_inverse_mult(
    world: &mut RayTracerWorld,
    lhs_name: String,
    rhs_name: String,
    expected_name: String,
) {
    let lhs = world.get_matrix_or_panic(&lhs_name);
    let rhs = world.get_matrix_or_panic(&rhs_name);
    let expected = world.get_matrix_or_panic(&expected_name);

    assert_abs_diff_eq!(
        lhs * &rhs.clone().try_inverse().expect("not invertible!"),
        &expected,
        epsilon = EPSILON
    );
}

fn main() {
    future::block_on(RayTracerWorld::run("tests/features/matrices.feature"));
}
