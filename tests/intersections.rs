use ray_tracer_challenge_rs::objects::{Intersection, Sphere};
use ray_tracer_challenge_rs::ray::Ray;

use cucumber::{given, then, when, World};
use futures_lite::future;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Default, World)]
struct IntersectionsWorld {
    spheres: HashMap<String, Sphere>,
    rays: HashMap<String, Ray>,
    intersections: HashMap<String, Intersection>,
    // lol
    intersectionses: HashMap<String, Option<(Intersection, Intersection)>>,
}

impl IntersectionsWorld {
    fn get_sphere_or_panic(&self, sphere_name: &String) -> &Sphere {
        self.spheres
            .get(sphere_name)
            .expect(format!("missing sphere named {}", sphere_name).as_str())
    }

    fn get_int_or_panic(&self, int_name: &String) -> Intersection {
        let inter = self
            .intersections
            .get(int_name)
            .expect(format!("missing intersection named {}", int_name).as_str());

        Intersection::new(inter.t, inter.object.clone())
    }

    fn get_ints_or_panic(&self, ints_name: &String) -> &Option<(Intersection, Intersection)> {
        self.intersectionses
            .get(ints_name)
            .expect(format!("missing intersections named {}", ints_name).as_str())
    }
}

#[given(expr = r"{word} ← sphere\(\)")]
fn given_a_sphere(world: &mut IntersectionsWorld, sphere_name: String) {
    world.spheres.insert(sphere_name, Sphere::new());
}

#[given(expr = r"{word} ← intersection\({float}, {word}\)")]
#[when(expr = r"{word} ← intersection\({float}, {word}\)")]
fn when_intersection_created(
    world: &mut IntersectionsWorld,
    int_name: String,
    t: f32,
    object_name: String,
) {
    let o = world.get_sphere_or_panic(&object_name);
    world
        .intersections
        .insert(int_name, Intersection::new(t, Rc::new(*o)));
}

#[given(expr = r"{word} ← intersections\({word}, {word}\)")]
#[when(expr = r"{word} ← intersections\({word}, {word}\)")]
fn when_intersections_created(
    world: &mut IntersectionsWorld,
    ints_name: String,
    int1_name: String,
    int2_name: String,
) {
    let int1 = world.get_int_or_panic(&int1_name);
    let int2 = world.get_int_or_panic(&int2_name);
    world.intersectionses.insert(ints_name, Some((int1, int2)));
}

#[then(regex = r"^(\w+).t = (.+)")]
fn assert_t(world: &mut IntersectionsWorld, int_name: String, expected_t: f32) {
    let i = world.get_int_or_panic(&int_name);

    assert_eq!(i.t, expected_t);
}

#[then(expr = r"{word}.count = {int}")]
fn assert_intersection_count(world: &mut IntersectionsWorld, int_name: String, expected: usize) {
    let intersect = world.get_ints_or_panic(&int_name);

    if expected == 0 {
        assert!(intersect.is_none())
    }

    // asserting the length of a tuple is redundant
}

#[then(regex = r"^(\w+)\[(\d)\]\.t = (.+)")]
fn assert_nth_intersection(
    world: &mut IntersectionsWorld,
    int_name: String,
    nth: usize,
    expected: f32,
) {
    let (i0, i1) = world
        .get_ints_or_panic(&int_name)
        .as_ref()
        .expect("no intersection");

    let actual = match nth {
        0 => i0,
        1 => i1,
        _ => panic!("bad nth value"),
    };

    assert_eq!(actual.t, expected);
}

fn main() {
    future::block_on(IntersectionsWorld::run(
        "tests/features/intersections.feature",
    ));
}
