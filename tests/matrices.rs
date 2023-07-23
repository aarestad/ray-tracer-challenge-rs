use cucumber::{gherkin::Step, given, then, when, Parameter, World};
use futures_lite::future;
use nalgebra::{ArrayStorage, DMatrix, Matrix4, MatrixN};
use ray_tracer_challenge_rs::tuple::Tuple;
use std::{collections::HashMap, str::FromStr, thread::sleep, time::Duration};

#[derive(Debug, Default, World)]
struct MatrixWorld {
    matrices: HashMap<String, DMatrix<f32>>,
    tuples: HashMap<String, Tuple>,
}

impl MatrixWorld {
    fn get_matrix_or_panic(&self, matrix_name: &String) -> &DMatrix<f32> {
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

fn get_matrix_from_step(step: &Step, rows: usize, cols: usize) -> DMatrix<f32> {
    let table = step.table.as_ref().expect("no table?");
    let mut data: Vec<f32> = vec![];

    for row in &table.rows {
        data.extend(row.iter().map(|e| f32::from_str(e).expect("bad number")));
    }

    // data is read in row-major; matrix is stored col-major
    DMatrix::from_vec(rows, cols, data).transpose()
}

#[given(expr = r"the following {int}x{int} matrix {word}:")]
fn given_a_matrix(world: &mut MatrixWorld, step: &Step, rows: usize, cols: usize, name: String) {
    world
        .matrices
        .insert(name, get_matrix_from_step(step, rows, cols));
}

#[given(expr = r"{word} ← tuple\({float}, {float}, {float}, {float}\)")]
fn given_a_tuple(world: &mut MatrixWorld, tuple_name: String, x: f32, y: f32, z: f32, w: f32) {
    world.tuples.insert(tuple_name, Tuple::new(x, y, z, w));
}

#[then(expr = r"{word}[{int},{int}] = {float}")]
fn assert_entry_value(
    world: &mut MatrixWorld,
    name: String,
    row: usize,
    col: usize,
    expected: f32,
) {
    let matrix = world.get_matrix_or_panic(&name);
    let num_cols = matrix.column_iter().count();
    let actual = matrix[row + col * num_cols];

    assert_eq!(actual, expected);
}

#[then(regex = r"^(\w+) (!)?= (\w+)$")]
fn assert_matrix_equality(
    world: &mut MatrixWorld,
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
    world: &mut MatrixWorld,
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
    world: &mut MatrixWorld,
    lhs_name: String,
    rhs_name: String,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let lhs = world.get_matrix_or_panic(&lhs_name);
    let rhs = world.get_tuple_or_panic(&rhs_name);
    let expected = Tuple::new(x, y, z, w);
    let actual = rhs.rhs_mult(&Matrix4::from_vec(lhs.data.as_vec().clone()));

    assert_eq!(expected, actual);
}

#[then(expr = r"{word} * identity_matrix = {word}")]
fn assert_matrix_identity_mult_rhs(
    world: &mut MatrixWorld,
    lhs_name: String,
    expected_name: String,
) {
    assert_eq!(lhs_name, expected_name, "expecting the same name");
    let m = world.get_matrix_or_panic(&lhs_name);
    let matrix_dim = m.column_iter().count();
    let identity = DMatrix::<f32>::identity(matrix_dim, matrix_dim);

    assert_eq!(*m, m * identity);
}

#[then(expr = r"identity_matrix * {word} = {word}")]
fn assert_tuple_identity_mult_lhs(
    world: &mut MatrixWorld,
    rhs_name: String,
    expected_name: String,
) {
    assert_eq!(rhs_name, expected_name, "expecting the same name");
    let t = world.get_tuple_or_panic(&rhs_name);
    let identity = Matrix4::<f32>::identity();

    assert_eq!(*t, t.rhs_mult(&identity),);
}

fn main() {
    future::block_on(MatrixWorld::run("tests/features/matrices.feature"));
}
