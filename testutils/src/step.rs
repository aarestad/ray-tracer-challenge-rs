use cucumber::gherkin::Step;
use nalgebra::DMatrix;
use std::str::FromStr;

pub fn get_matrix_from_step(step: &Step, rows: usize, cols: usize) -> DMatrix<f32> {
    let table = step.table.as_ref().expect("no table?");
    let mut data: Vec<f32> = vec![];

    for row in &table.rows {
        data.extend(row.iter().map(|e| f32::from_str(e).expect("bad number")));
    }

    // data is read in row-major; matrix is stored col-major
    DMatrix::from_vec(rows, cols, data).transpose()
}
