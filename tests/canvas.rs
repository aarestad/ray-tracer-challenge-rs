use std::collections::HashMap;

use cucumber::{given, then, when, Parameter, World};
use futures_lite::future;
use ray_tracer_challenge_rs::canvas::Canvas;
use ray_tracer_challenge_rs::color::Color;

#[derive(Debug, Default, World)]
struct CanvasWorld {
    canvases: HashMap<String, Canvas>,
}

impl CanvasWorld {
    fn get_canvas_or_panic(&self, canvas_name: &String) -> &Canvas {
        self.canvases
            .get(canvas_name)
            .expect(format!("missing canvas named {}", canvas_name).as_str())
    }
}

#[given(expr = r"{word} ← canvas\({int}, {int}\)")]
fn given_a_canvas(world: &mut CanvasWorld, name: String, width: usize, height: usize) {
    world.canvases.insert(name, Canvas::new(width, height));
}

#[then(expr = r"{word}.{word} = {int}")]
fn assert_property(
    world: &mut CanvasWorld,
    canvas_name: String,
    prop_name: String,
    expected: usize,
) {
    let canvas = world.get_canvas_or_panic(&canvas_name);

    let actual = match prop_name.as_str() {
        "width" => canvas.width(),
        "height" => canvas.height(),
        _ => panic!("unknown property {}", prop_name),
    };

    assert!(
        expected == actual,
        "expected {}.{} to be {} but was {}",
        canvas_name,
        prop_name,
        expected,
        actual
    );
}

#[then(expr = r"every pixel of {word} is {}")]
fn assert_every_pixel(world: &mut CanvasWorld, canvas_name: String, color: Color) {}

fn main() {
    future::block_on(CanvasWorld::run("tests/features/canvas.feature"));
}
