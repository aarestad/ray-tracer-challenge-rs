use cucumber::{gherkin::Step, then, when, World};
use futures_lite::future;
use ray_tracer_challenge_rs::color::Color;
use testutils::world::RayTracerWorld;
use testutils::RayTracerFloat;

#[when(expr = r"write_pixel\({word}, {int}, {int}, {word}\)")]
fn when_write_pixel(
    world: &mut RayTracerWorld,
    canvas_name: String,
    x: usize,
    y: usize,
    color_name: String,
) {
    let color = world.get_color_or_panic(&color_name).clone();
    let canvas = world.get_mut_canvas_or_panic(&canvas_name);
    canvas.write(x, y, color);
}

#[when(expr = r"{word} ← canvas_to_ppm\({word}\)")]
fn when_canvas_to_ppm(world: &mut RayTracerWorld, ppm_name: String, canvas_name: String) {
    let canvas = world.get_canvas_or_panic(&canvas_name);
    world.ppms.insert(ppm_name, canvas.to_ppm());
}

#[when(expr = r"every pixel of {word} is set to color\({float}, {float}, {float}\)")]
fn when_every_pixel_set(
    world: &mut RayTracerWorld,
    canvas_name: String,
    r: RayTracerFloat,
    g: RayTracerFloat,
    b: RayTracerFloat,
) {
    let canvas = world.get_mut_canvas_or_panic(&canvas_name);

    for y in 0..canvas.height() {
        for x in 0..canvas.width() {
            canvas.write(x, y, Color::new(r, g, b));
        }
    }
}

#[then(expr = r"{word}.{word} = {int}")]
fn assert_property(
    world: &mut RayTracerWorld,
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
fn assert_every_pixel(world: &mut RayTracerWorld, canvas_name: String, expected: Color) {
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
    world: &mut RayTracerWorld,
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
    world: &mut RayTracerWorld,
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
fn assert_ppm_ends_with_newline(world: &mut RayTracerWorld, ppm_name: String) {
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
    future::block_on(RayTracerWorld::run("tests/features/canvas.feature"));
}
