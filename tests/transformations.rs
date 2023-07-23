use cucumber::{gherkin::Step, given, then, World};
use futures_lite::future;
use nalgebra::Matrix4;
use ray_tracer_challenge_rs::tuple::Tuple;
use std::collections::HashMap;

use ray_tracer_challenge_rs::transforms::translation;

#[derive(Debug, Default, World)]
struct TransformationsWorld {
    matrices: HashMap<String, Matrix4<f32>>,
    tuples: HashMap<String, Tuple>,
}

impl TransformationsWorld {
    fn get_matrix_or_panic(&self, matrix_name: &String) -> &Matrix4<f32> {
        self.matrices
            .get(matrix_name)
            .expect(format!("missing array {}", matrix_name).as_str())
    }

    fn get_tuple_or_panic(&self, tuple_name: &String) -> &Tuple {
        self.tuples
            .get(tuple_name)
            .expect(format!("missing tuple {}", tuple_name).as_str())
    }
}

#[given(expr = r"{word} ← translation\({float}, {float}, {float}\)")]
fn given_a_translation_matrix(
    world: &mut TransformationsWorld,
    matrix_name: String,
    x: f32,
    y: f32,
    z: f32,
) {
    world.matrices.insert(matrix_name, translation(x, y, z));
}

#[given(expr = r"{word} ← point\({float}, {float}, {float}\)")]
fn given_a_point(world: &mut TransformationsWorld, point_name: String, x: f32, y: f32, z: f32) {
    world.tuples.insert(point_name, Tuple::point(x, y, z));
}

#[then(expr = r"{word} * {word} = point\({float}, {float}, {float}\)")]
fn assert_point_transform(
    world: &mut TransformationsWorld,
    matrix_name: String,
    point_name: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let lhs = world.get_matrix_or_panic(&matrix_name);
    let rhs = world.get_tuple_or_panic(&point_name);
    let expected = Tuple::point(x, y, z);
    let actual = rhs.rhs_mult(lhs);

    assert!(expected.approx_eq(&actual));
}

fn main() {
    future::block_on(TransformationsWorld::run("tests/features/tuples.feature"));
}
