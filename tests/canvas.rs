use std::collections::HashMap;

use cucumber::{given, then, when, Parameter, World};
use futures_lite::future;
use ray_tracer_challenge_rs::canvas::Canvas;
use ray_tracer_challenge_rs::color::Color;

#[derive(Debug, Default, World)]
struct CanvasWorld {
    canvases: HashMap<String, Canvas>,
    colors: HashMap<String, Color>,
}

impl CanvasWorld {
    fn get_canvas_or_panic(&self, canvas_name: &String) -> &Canvas {
        self.canvases
            .get(canvas_name)
            .expect(format!("missing canvas named {}", canvas_name).as_str())
    }

    fn get_mut_canvas_or_panic(&mut self, canvas_name: &String) -> &mut Canvas {
        self.canvases
            .get_mut(canvas_name)
            .expect(format!("missing canvas named {}", canvas_name).as_str())
    }

    fn get_color_or_panic(&self, color_name: &String) -> &Color {
        self.colors
            .get(color_name)
            .expect(format!("missing color named {}", color_name).as_str())
    }
}

#[given(expr = r"{word} ← canvas\({int}, {int}\)")]
fn given_a_canvas(world: &mut CanvasWorld, name: String, width: usize, height: usize) {
    world.canvases.insert(name, Canvas::new(width, height));
}

#[given(expr = r"{word} ← color\({int}, {int}, {int}\)")]
fn given_a_color(world: &mut CanvasWorld, name: String, r: f32, g: f32, b: f32) {
    world.colors.insert(name, Color::new(r, g, b));
}

#[when(expr = r"write_pixel\({word}, {int}, {int}, {word}\)")]
fn when_write_pixel(
    world: &mut CanvasWorld,
    canvas_name: String,
    x: usize,
    y: usize,
    color_name: String,
) {
    let color = world.get_color_or_panic(&color_name).clone();
    let canvas = world.get_mut_canvas_or_panic(&canvas_name);
    canvas.write(x, y, color);
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
fn assert_every_pixel(world: &mut CanvasWorld, canvas_name: String, expected: Color) {
    let canvas = world.get_canvas_or_panic(&canvas_name);

    for x in 0..canvas.width() {
        for y in 0..canvas.height() {
            let actual = canvas.pixel_at(x, y);
            assert!(
                expected == actual,
                "pixel at {}, {} expected to be {} but was {}",
                x,
                y,
                expected,
                actual,
            );
        }
    }
}

#[then(expr = r"pixel_at\({word}, {int}, {int}\) = {word}")]
fn assert_pixel_at(
    world: &mut CanvasWorld,
    canvas_name: String,
    x: usize,
    y: usize,
    color_name: String,
) {
    let canvas = world.get_canvas_or_panic(&canvas_name);
    let expected = world.get_color_or_panic(&color_name);
    let actual = canvas.pixel_at(x, y);

    assert!(
        *expected == actual,
        "pixel at {}, {} expected to be {} but was {}",
        x,
        y,
        expected,
        actual,
    );
}

fn main() {
    future::block_on(CanvasWorld::run("tests/features/canvas.feature"));
}
