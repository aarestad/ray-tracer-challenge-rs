use cucumber::gherkin::Step;
use nalgebra::{DMatrix, Matrix4};
use std::str::FromStr;
use crate::RayTracerFloat;

pub fn get_matrix_from_step(step: &Step, rows: usize, cols: usize) -> DMatrix<RayTracerFloat> {
    let table = step.table.as_ref().expect("no table?");
    let mut data: Vec<RayTracerFloat> = vec![];

    for row in &table.rows {
        data.extend(row.iter().map(|e| RayTracerFloat::from_str(e).expect("bad number")));
    }

    // data is read in row-major; matrix is stored col-major
    DMatrix::from_vec(rows, cols, data).transpose()
}

// TODO: can this use the function above?
pub fn get_4x4_matrix_from_step(step: &Step) -> Matrix4<RayTracerFloat> {
    let table = step.table.as_ref().expect("no table?");
    let mut data: Vec<RayTracerFloat> = vec![];

    for row in &table.rows {
        data.extend(row.iter().map(|e| RayTracerFloat::from_str(e).expect("bad number")));
    }

    // data is read in row-major; matrix is stored col-major
    Matrix4::from_vec(data).transpose()
}
