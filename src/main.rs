use std::io::Result;
use std::path::Path;

mod basic_ray_trace;
mod camera;
mod canvas;
mod color;
mod intersection;
mod light;
mod material;
mod objects;
mod ray;
mod transforms;
mod tuple;
mod util;
mod world;

fn main() -> Result<()> {
    basic_ray_trace::basic_ray_trace(Path::new("original_sphere.ppm"), transforms::identity())?;
    basic_ray_trace::chapter_7_scene(Path::new("ch7_world.ppm"))?;
    Ok(())
}
