use cucumber::{then, World};
use futures_lite::future;
use nalgebra::Matrix4;

use testutils::world::RayTracerWorld;

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

fn main() {
    future::block_on(RayTracerWorld::run("tests/features/spheres.feature"));
}
