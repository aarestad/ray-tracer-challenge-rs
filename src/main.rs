use std::f32::consts::PI;
use std::io::Result;
use std::path::Path;

use nalgebra::Matrix4;

use basic_ray_trace::basic_ray_trace;

mod basic_ray_trace;
mod canvas;
mod clock_face;
mod color;
mod intersection;
mod light;
mod material;
mod objects;
mod ppm;
mod ray;
mod transforms;
mod tuple;
mod util;
mod virtual_cannon;
mod world;

fn main() -> Result<()> {
    basic_ray_trace::basic_ray_trace(Path::new("original_sphere.ppm"), Matrix4::identity())?;
    Ok(())
}
