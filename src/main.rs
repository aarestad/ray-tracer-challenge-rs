#![feature(get_mut_unchecked)]

use std::f64::consts::FRAC_PI_3;
use std::io::Result;
use std::path::Path;

use basic_ray_trace::basic_scene;
use camera::Camera;
use color::Color;
use examples::hexagon_scene::hexagon;
use light::PointLight;
use tuple::{Point, Vector};
use world::World;

mod basic_ray_trace;
mod camera;
mod canvas;
mod color;
mod examples;
mod intersection;
mod light;
mod material;
mod objects;
mod patterns;
mod precompute;
mod ray;
mod transforms;
mod tuple;
mod util;
mod world;

fn main() -> Result<()> {
    basic_ray_trace::basic_ray_trace(Path::new("original_sphere.ppm"), transforms::identity())?;

    basic_ray_trace::render_scene_to_file(
        basic_scene(),
        Camera::new(
            600,
            300,
            FRAC_PI_3,
            Point::point(0., 1.5, -5.)
                .view_transform(&Point::point(0., 1., 0.), &Vector::vector(0., 1., 0.)),
        ),
        Path::new("ch7_world.ppm"),
    )?;

    basic_ray_trace::render_scene_to_file(
        World::new(vec![hexagon()], PointLight::new(Point::point(-10., 10., -10.), Color::new(1., 1., 1.))),
        Camera::new(
            600,
            300,
            FRAC_PI_3,
            Point::point(0., 1.5, -5.)
                .view_transform(&Point::point(0., 1., 0.), &Vector::vector(0., 1., 0.)),
        ),
        Path::new("hexagon.ppm"),
    )?;

    Ok(())
}
