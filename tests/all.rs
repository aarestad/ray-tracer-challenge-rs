use cucumber::World;
use futures_lite::future;
use std::env::args;
use testutils::world::RayTracerWorld;

const ALL_TESTS: [&str; 7] = [
    "lights",
    "materials",
    "world",
    "camera",
    "shapes",
    "planes",
    "patterns",
];

fn main() {
    let mut tests: Vec<String> = args().collect();

    if tests.is_empty() {
        tests.extend(ALL_TESTS.iter().map(|s| s.to_string()));
    }

    // future::block_on(RayTracerWorld::run("tests/features/lights.feature"));
    // future::block_on(RayTracerWorld::run("tests/features/materials.feature"));
    // future::block_on(RayTracerWorld::run("tests/features/world.feature"));
    // future::block_on(RayTracerWorld::run("tests/features/camera.feature"));
    // future::block_on(RayTracerWorld::run("tests/features/shapes.feature"));
    // future::block_on(RayTracerWorld::run("tests/features/planes.feature"));
    future::block_on(RayTracerWorld::run("tests/features/patterns.feature"));
}
