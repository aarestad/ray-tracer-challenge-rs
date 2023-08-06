use nalgebra::Matrix4;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::light::PointLight;
use crate::material::MaterialBuilder;
use crate::objects::{Object, Sphere};
use crate::ray::Ray;
use crate::tuple::Point;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

pub fn basic_ray_trace(filename: &Path, transform: Matrix4<f32>) -> Result<()> {
    let ray_origin = Point::point(0., 0., -5.);
    let wall_z = 10f32;
    let wall_size = 7f32;
    let canvas_pixels = 100usize;
    let pixel_size = wall_size / (canvas_pixels as f32);
    let half = wall_size / 2.;

    let light = PointLight::new(Point::point(-10., 10., -10.), Color::new(1., 1., 1.));

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let sphere = Sphere::new(
        transform,
        MaterialBuilder::default()
            .color(Color::new(1., 0.2, 1.))
            .diffuse(0.9)
            .specular(0.9)
            .build(),
    );

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f32);

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f32);
            let position = Point::point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let ints = sphere.intersections(&ray);

            if let Some(hit) = ints.hit() {
                let p = ray.position(hit.t);
                let n = hit.object.normal_at(p);
                let e = -ray.direction;
                let c = hit.object.material().lighting(light, p, e, n);

                canvas.write(x, y, c);
            }
        }
    }

    let mut output = File::create(filename)?;
    write!(output, "{}", canvas.to_ppm().whole_file())
}
