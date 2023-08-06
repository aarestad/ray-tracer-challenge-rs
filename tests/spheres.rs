use cucumber::{then, World};
use futures_lite::future;

use ray_tracer_challenge_rs::transforms;
use testutils::world::RayTracerWorld;
use testutils::RayTracerFloat;

#[then(expr = r"{word}[{int}] = {float}")]
fn assert_nth_intersection(
    world: &mut RayTracerWorld,
    int_name: String,
    nth: usize,
    expected: RayTracerFloat,
) {
    let ints = world.get_ints_or_panic(&int_name);

    let actual = &ints.ints()[nth];

    assert_eq!(actual.t, expected);
}

fn main() {
    future::block_on(RayTracerWorld::run("tests/features/spheres.feature"));
}
