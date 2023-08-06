use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::tuple::{Point, Vector};

pub struct Projectile {
    position: Point,
    velocity: Vector,
}

pub struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Projectile {
        Projectile { position, velocity }
    }
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Environment {
        Environment { gravity, wind }
    }

    pub fn tick(&self, proj: Projectile) -> Projectile {
        Projectile {
            position: proj.position + proj.velocity,
            velocity: proj.velocity + self.gravity + self.wind,
        }
    }
}

#[allow(dead_code)]
pub fn ch1_playground() {
    let mut proj = Projectile::new(
        Point::point(0., 1., 0.),
        Vector::vector(1., 1., 0.).normalize(),
    );
    let env = Environment::new(Vector::vector(0., -0.1, 0.), Vector::vector(-0.01, 0., 0.));

    while proj.position.y() > 0. {
        proj = env.tick(proj);
        println!("proj now at {:?}", proj.position);
    }
}

#[allow(dead_code)]
pub fn ch2_playground(filename: &Path) -> Result<()> {
    let mut proj = Projectile::new(
        Point::point(0., 1., 0.),
        Vector::vector(1., 1.8, 0.).normalize() * 11.25,
    );

    let env = Environment::new(Vector::vector(0., -0.1, 0.), Vector::vector(-0.01, 0., 0.));

    let mut canvas = Canvas::new(900, 550);
    let color = Color::new(0.5, 0.7, 0.5);

    while proj.position.y() >= 0. {
        canvas.write(
            proj.position.x().floor() as usize,
            550 - proj.position.y() as usize,
            color,
        );

        proj = env.tick(proj);
    }

    let mut output = File::create(filename)?;
    write!(output, "{}", canvas.to_ppm().whole_file())
}
