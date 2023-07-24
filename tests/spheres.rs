use nalgebra::Matrix4;
use ray_tracer_challenge_rs::intersection::Intersectable;
use ray_tracer_challenge_rs::objects::Sphere;
use ray_tracer_challenge_rs::ray::Ray;
use ray_tracer_challenge_rs::transforms::{scaling, translation};
use ray_tracer_challenge_rs::tuple::Tuple;

use testutils::world::RayTracerWorld;

use cucumber::{given, then, when, World};
use futures_lite::future;

#[given(
    expr = r"{word} ← ray\(point\({float}, {float}, {float}\), vector\({float}, {float}, {float}\)\)"
)]
fn given_a_ray(
    world: &mut RayTracerWorld,
    ray_name: String,
    ox: f32,
    oy: f32,
    oz: f32,
    dx: f32,
    dy: f32,
    dz: f32,
) {
    world.rays.insert(
        ray_name,
        Ray::new(Tuple::point(ox, oy, oz), Tuple::vector(dx, dy, dz)),
    );
}

#[given(expr = r"{word} ← sphere\(\)")]
fn given_a_sphere(world: &mut RayTracerWorld, sphere_name: String) {
    world.spheres.insert(sphere_name, Sphere::default());
}

#[given(expr = r"{word} ← sphere\({word}\)")]
fn given_a_sphere_with_trans(world: &mut RayTracerWorld, sphere_name: String, trans_name: String) {
    let trans = world.get_transform_or_panic(&trans_name);
    world.spheres.insert(sphere_name, Sphere::new(*trans));
}

#[given(expr = r"{word} ← translation\({float}, {float}, {float}\)")]
fn given_a_translation(world: &mut RayTracerWorld, trans_name: String, x: f32, y: f32, z: f32) {
    world.transforms.insert(trans_name, translation(x, y, z));
}

#[given(expr = r"{word} ← scaling\({float}, {float}, {float}\)")]
fn given_a_scaling(world: &mut RayTracerWorld, trans_name: String, x: f32, y: f32, z: f32) {
    world.transforms.insert(trans_name, scaling(x, y, z));
}

#[given(expr = r"{word} ← intersect\({word}, {word}\)")]
#[when(expr = r"{word} ← intersect\({word}, {word}\)")]
fn when_ray_intersects_sphere(
    world: &mut RayTracerWorld,
    int_name: String,
    sphere_name: String,
    ray_name: String,
) {
    let sphere = world.get_sphere_or_panic(&sphere_name);
    let ray = world.get_ray_or_panic(&ray_name);
    world
        .intersectionses
        .insert(int_name, sphere.intersections(ray));
}

#[then(expr = r"{word}.count = {int}")]
fn assert_intersection_count(world: &mut RayTracerWorld, int_name: String, expected: usize) {
    let intersects = world.get_ints_or_panic(&int_name);

    assert!(intersects.ints().len() == expected);
}

#[then(expr = r"{word}[{int}] = {float}")]
fn assert_nth_intersection(
    world: &mut RayTracerWorld,
    int_name: String,
    nth: usize,
    expected: f32,
) {
    let ints = world.get_ints_or_panic(&int_name);

    let actual = &ints.ints()[nth];

    assert_eq!(actual.t, expected);
}

#[then(expr = r"{word}.transform = {word}")]
fn assert_transform(world: &mut RayTracerWorld, sphere_name: String, trans_name: String) {
    let s = world.get_sphere_or_panic(&sphere_name);

    let t = if trans_name == "identity_matrix" {
        Matrix4::identity()
    } else {
        *world.get_transform_or_panic(&trans_name)
    };

    assert_eq!(s.transform(), &t)
}

#[then(expr = r"{word}[{int}].t = {float}")]
fn assert_nth_intersection_t(
    world: &mut RayTracerWorld,
    int_name: String,
    nth: usize,
    expected: f32,
) {
    let intersects = world.get_ints_or_panic(&int_name);

    assert_eq!(intersects.ints()[nth].t, expected);
}

fn main() {
    future::block_on(RayTracerWorld::run("tests/features/spheres.feature"));
}
