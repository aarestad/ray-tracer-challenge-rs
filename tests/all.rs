use cucumber::World;
use futures_lite::future;
use testutils::world::RayTracerWorld;

fn main() {
    // future::block_on(RayTracerWorld::run("tests/features/lights.feature"));
    future::block_on(RayTracerWorld::run("tests/features/materials.feature"));
    // future::block_on(RayTracerWorld::run("tests/features/world.feature"));
    // future::block_on(RayTracerWorld::run("tests/features/camera.feature"));
    // future::block_on(RayTracerWorld::run("tests/features/shapes.feature"));
    // future::block_on(RayTracerWorld::run("tests/features/planes.feature"));
    // future::block_on(RayTracerWorld::run("tests/features/patterns.feature"));
}
