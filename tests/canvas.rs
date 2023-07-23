use std::collections::HashMap;

use cucumber::{gherkin::Step, given, then, when, World};
use futures_lite::future;
use ray_tracer_challenge_rs::canvas::Canvas;
use ray_tracer_challenge_rs::color::Color;
use ray_tracer_challenge_rs::ppm::Ppm;

#[derive(Debug, Default, World)]
struct CanvasWorld {
    canvases: HashMap<String, Canvas>,
    colors: HashMap<String, Color>,
    ppms: HashMap<String, Ppm>,
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

    fn get_ppm_or_panic(&self, ppm_name: &String) -> &Ppm {
        self.ppms
            .get(ppm_name)
            .expect(format!("missing PPM named {}", ppm_name).as_str())
    }
}

#[given(expr = r"{word} ← canvas\({int}, {int}\)")]
fn given_a_canvas(world: &mut CanvasWorld, name: String, width: usize, height: usize) {
    world.canvases.insert(name, Canvas::new(width, height));
}

#[given(expr = r"{word} ← color\({float}, {float}, {float}\)")]
fn given_a_color(world: &mut CanvasWorld, name: String, r: f32, g: f32, b: f32) {
    world.colors.insert(name, Color::new(r, g, b));
}

#[when(expr = r"write_pixel\({word}, {int}, {int}, {word}\)")]
fn when_write_pixel(
    world: &mut CanvasWorld,
    canvas_name: String,
    x: f32,
    y: f32,
    color_name: String,
) {
    let color = world.get_color_or_panic(&color_name).clone();
    let canvas = world.get_mut_canvas_or_panic(&canvas_name);
    canvas.write(x, y, color);
}

#[when(expr = r"{word} ← canvas_to_ppm\({word}\)")]
fn when_canvas_to_ppm(world: &mut CanvasWorld, ppm_name: String, canvas_name: String) {
    let canvas = world.get_canvas_or_panic(&canvas_name);
    world.ppms.insert(ppm_name, canvas.to_ppm());
}

#[when(expr = r"every pixel of {word} is set to color\({float}, {float}, {float}\)")]
fn when_every_pixel_set(world: &mut CanvasWorld, canvas_name: String, r: f32, g: f32, b: f32) {
    let canvas = world.get_mut_canvas_or_panic(&canvas_name);

    for y in 0..canvas.height() {
        for x in 0..canvas.width() {
            canvas.write(x as f32, y as f32, Color::new(r, g, b));
        }
    }
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

#[then(expr = r"lines {int}-{int} of {word} are")]
fn assert_ppm_lines(
    world: &mut CanvasWorld,
    step: &Step,
    start_1_offset: usize,
    end_inclusive_1_offset: usize,
    ppm_name: String,
) {
    let ppm = world.get_ppm_or_panic(&ppm_name);
    let actual = ppm.lines_range(start_1_offset - 1, end_inclusive_1_offset);
    let expected = step.docstring.as_ref().expect("no docstring").trim_start();

    assert!(
        expected == actual,
        "lines {}-{} expected to be <{}> but were actually <{}>",
        start_1_offset,
        end_inclusive_1_offset,
        expected,
        actual,
    );
}

#[then(expr = r"{word} ends with a newline character")]
fn assert_ppm_ends_with_newline(world: &mut CanvasWorld, ppm_name: String) {
    let ppm = world.get_ppm_or_panic(&ppm_name);

    assert!(
        ppm.whole_file()
            .chars()
            .into_iter()
            .rev()
            .next()
            .expect("zero size file")
            == '\n',
        "expected ppm {} to end with newline but it did not",
        ppm_name
    );
}

fn main() {
    future::block_on(CanvasWorld::run("tests/features/canvas.feature"));
}
