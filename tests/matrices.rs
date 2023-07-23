use cucumber::{gherkin::Step, given, then, when, Parameter, World};
use futures_lite::future;
use ndarray::Array2;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Default, World)]
struct MatrixWorld {
    matrices: HashMap<String, Array2<f32>>,
}

impl MatrixWorld {
    fn get_matrix_or_panic(&self, matrix_name: &String) -> &Array2<f32> {
        self.matrices
            .get(matrix_name)
            .expect(format!("missing array {}", matrix_name).as_str())
    }
}

#[given(expr = r"the following {int}x{int} matrix {word}:")]
fn given_a_matrix(world: &mut MatrixWorld, step: &Step, rows: usize, cols: usize, name: String) {
    if let Some(table) = step.table.as_ref() {
        let mut data: Vec<f32> = vec![];

        for row in &table.rows {
            data.extend(row.iter().map(|e| f32::from_str(e).expect("bad number")))
        }

        let arr = Array2::from_shape_vec((rows, cols), data).expect("bad array shape");

        world.matrices.insert(name, arr);
    }
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
    let actual = matrix[[row, col]];

    assert! {
        actual == expected,
        "expected {}[{},{}] to be {} but was {}", name, row, col, expected, actual
    };
}

#[then(regex = r"(\w+) = (\w+)")]
fn assert_matrix_equality(world: &mut MatrixWorld, lhs_name: String, rhs_name: String) {
    let lhs = world.get_matrix_or_panic(&lhs_name);
    let rhs = world.get_matrix_or_panic(&rhs_name);

    assert!(
        lhs == rhs,
        "expected {} = {} but were not",
        lhs_name,
        rhs_name
    );
}

fn main() {
    future::block_on(MatrixWorld::run("tests/features/matrices.feature"));
}
