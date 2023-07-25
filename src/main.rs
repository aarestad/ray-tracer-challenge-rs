use std::f32::consts::PI;
use std::io::Result;
use std::path::Path;

use nalgebra::Matrix4;

use crate::transforms::*;

mod basic_ray_trace;
mod canvas;
mod clock_face;
mod color;
mod intersection;
mod material;
mod objects;
mod ppm;
mod ray;
mod transforms;
mod tuple;
mod util;
mod virtual_cannon;

fn main() -> Result<()> {
    virtual_cannon::ch2_playground(Path::new("trajectory.ppm"))?;
    clock_face::ch4_playground(Path::new("clock_face.ppm"))?;
    basic_ray_trace::ch5_playground(Path::new("original_sphere.ppm"), Matrix4::identity())?;
    basic_ray_trace::ch5_playground(Path::new("shrink_y.ppm"), scaling(1., 0.5, 1.))?;
    basic_ray_trace::ch5_playground(Path::new("shrink_x.ppm"), scaling(0.5, 1., 1.))?;
    basic_ray_trace::ch5_playground(
        Path::new("shrink_rotate.ppm"),
        rotation(RotationAxis::Z, PI / 4.) * scaling(0.5, 1., 1.),
    )?;
    basic_ray_trace::ch5_playground(
        Path::new("shrink_skew.ppm"),
        shearing(1., 0., 0., 0., 0., 0.) * scaling(0.5, 1., 1.),
    )?;

    Ok(())
}
