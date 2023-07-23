use cucumber::{gherkin::Step, given, then, when, Parameter, World};
use futures_lite::future;
use nalgebra::DMatrix;
use std::{collections::HashMap, str::FromStr, thread::sleep, time::Duration};

#[derive(Debug, Default, World)]
struct MatrixWorld {
    matrices: HashMap<String, DMatrix<f32>>,
}

impl MatrixWorld {
    fn get_matrix_or_panic(&self, matrix_name: &String) -> &DMatrix<f32> {
        self.matrices
            .get(matrix_name)
            .expect(format!("missing array {}", matrix_name).as_str())
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

    assert! {
        actual == expected,
        "expected {}[{},{}] to be {} but was {}", name, row, col, expected, actual
    };
}

#[then(regex = r"(\w+) (!)?= (\w+)")]
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
async fn assert_matrix_mult(
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

    assert! {
        expected == actual,
        "expected {} * {} to be {} but was {}",
        lhs_name, rhs_name, expected, actual,
    };
}

fn main() {
    future::block_on(MatrixWorld::run("tests/features/matrices.feature"));
}
