use std::f64::consts::PI;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;
use std::rc::Rc;

use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::color::{Color, WHITE};
use crate::light::PointLight;
use crate::material::MaterialBuilder;
use crate::objects::{Object, Plane, Sphere};
use crate::patterns::{Gradient, Stripe};
use crate::ray::Ray;
use crate::transforms::{identity, scaling, translation, Transform};
use crate::tuple::{Point, Vector};
use crate::util::RayTracerFloat;
use crate::world::World;

pub fn basic_ray_trace(filename: &Path, transform: Transform) -> Result<()> {
    let ray_origin = Point::point(0., 0., -5.);
    let wall_z = 10. as RayTracerFloat;
    let wall_size = 7. as RayTracerFloat;
    let canvas_pixels = 100_usize;
    let pixel_size = wall_size / (canvas_pixels as RayTracerFloat);
    let half = wall_size / 2.;

    let light = PointLight::new(Point::point(-10., 10., -10.), Color::new(1., 1., 1.));

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let sphere = Rc::new(Sphere::new(
        transform,
        Rc::new(
            MaterialBuilder::default()
                .color(Color::new(1., 0.2, 1.))
                .diffuse(0.9)
                .specular(0.9)
                .build(),
        ),
    ));

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as RayTracerFloat);

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as RayTracerFloat);
            let position = Point::point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let ints = sphere.clone().intersections(&ray);

            if let Some(hit) = Rc::new(ints).hit() {
                let p = ray.position(hit.t);
                let n = hit.object.normal_at(p);
                let e = -ray.direction;
                let c = hit
                    .object
                    .material()
                    .lighting(hit.object.as_ref(), light, p, e, n, false);

                canvas.write(x, y, c);
            }
        }
    }

    let mut output = File::create(filename)?;
    write!(output, "{}", canvas.to_ppm().whole_file())
}

pub fn chapter_7_scene(filename: &Path) -> Result<()> {
    let floor = Plane::new(
        identity(),
        Rc::new(
            MaterialBuilder::default()
                .pattern(Rc::new(Gradient::new(
                    Color::new(1., 0., 0.),
                    WHITE,
                    identity(),
                )))
                .specular(0.)
                .reflective(0.5)
                .build(),
        ),
    );

    let middle_sphere = Sphere::new(
        translation(-0.5, 1., 0.5),
        Rc::new(
            MaterialBuilder::default()
                .pattern(Rc::new(Stripe::new(
                    Color::new(1., 0., 0.),
                    WHITE,
                    identity(),
                )))
                .diffuse(0.7)
                .specular(0.3)
                .reflective(0.8)
                .build(),
        ),
    );

    let right_sphere = Sphere::new(
        translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5),
        Rc::new(
            MaterialBuilder::default()
                .pattern(Rc::new(Gradient::new(
                    Color::new(1., 0., 0.),
                    WHITE,
                    identity(),
                )))
                .diffuse(0.7)
                .specular(0.3)
                .reflective(1.)
                .build(),
        ),
    );

    let left_sphere = Sphere::new(
        translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33),
        Rc::new(
            MaterialBuilder::default()
                .color(Color::new(1., 0.8, 0.1))
                .diffuse(0.7)
                .specular(0.3)
                .build(),
        ),
    );

    let light = PointLight::new(Point::point(-10., 10., -10.), Color::new(1., 1., 1.));

    let world = World::new(
        vec![
            Rc::new(floor),
            Rc::new(middle_sphere),
            Rc::new(right_sphere),
            Rc::new(left_sphere),
        ],
        light,
    );

    let view = Point::point(0., 1.5, -5.)
        .view_transform(&Point::point(0., 1., 0.), &Vector::vector(0., 1., 0.));

    let camera = Camera::new(600, 300, PI / 3., view);

    let mut output = File::create(filename)?;
    write!(
        output,
        "{}",
        camera.render(&Rc::new(world)).to_ppm().whole_file()
    )
}
