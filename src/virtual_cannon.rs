use crate::tuple::Tuple;

pub struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

pub struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Projectile {
        Projectile { position, velocity }
    }
}

impl Environment {
    pub fn new(gravity: Tuple, wind: Tuple) -> Environment {
        Environment { gravity, wind }
    }

    pub fn tick(&self, proj: Projectile) -> Projectile {
        Projectile {
            position: proj.position + proj.velocity,
            velocity: proj.velocity + self.gravity + self.wind,
        }
    }
}

pub fn ch1_playground() {
    let mut proj = Projectile::new(
        Tuple::point(0., 1., 0.),
        Tuple::vector(1., 1., 0.).normalize(),
    );
    let env = Environment::new(Tuple::vector(0., -0.1, 0.), Tuple::vector(-0.01, 0., 0.));

    while proj.position.y > 0. {
        proj = env.tick(proj);
        println!("proj now at {:?}", proj.position);
    }
}
