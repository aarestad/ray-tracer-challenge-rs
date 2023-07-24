use nalgebra::Matrix4;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::intersection::Intersectable;
use crate::objects::Sphere;
use crate::ray::Ray;
use crate::tuple::Tuple;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

pub fn ch5_playground(filename: &Path, transform: Matrix4<f32>) -> Result<()> {
    let ray_origin = Tuple::point(0., 0., -5.);
    let wall_z = 10f32;
    let wall_size = 7f32;
    let canvas_pixels = 100usize;
    let pixel_size = wall_size / (canvas_pixels as f32);
    let half = wall_size / 2.;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let red = Color::new(1., 0., 0.);
    let sphere = Sphere::new(transform);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f32);

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f32);
            let position = Tuple::point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let ints = sphere.intersections(&ray);

            if ints.hit().is_some() {
                canvas.write(x, y, red);
            }
        }
    }

    let mut output = File::create(filename)?;
    write!(output, "{}", canvas.to_ppm().whole_file())
}
