use ray_tracer_challenge_rs::objects::{Intersectable, Intersection, Sphere};
use ray_tracer_challenge_rs::ray::Ray;
use ray_tracer_challenge_rs::tuple::Tuple;

use cucumber::{given, then, when, World};
use futures_lite::future;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Default, World)]
struct IntersectionsWorld {
    spheres: HashMap<String, Sphere>,
    rays: HashMap<String, Ray>,
    intersections: HashMap<String, Option<(Intersection, Intersection)>>,
}

fn main() {
    future::block_on(IntersectionsWorld::run(
        "tests/features/intersections.feature",
    ));
}
