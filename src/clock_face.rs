use crate::{
    canvas::Canvas,
    color::Color,
    transforms::{rotation, translation, RotationAxis},
    tuple::Point,
};
use std::f32::consts::TAU;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

#[allow(dead_code)]
pub fn ch4_playground(filename: &Path) -> Result<()> {
    let mut canvas = Canvas::new(500, 500);
    let color = Color::new(1., 1., 1.);
    let canvas_translation = translation(250., 250., 0.);
    let rotation = rotation(RotationAxis::Z, TAU / 12.);

    let mut current_loc = Point::point(200., 0., 0.);

    for _ in 0..12 {
        let canvas_loc = current_loc.transform(&canvas_translation);
        canvas.write(
            canvas_loc.x().floor() as usize,
            canvas_loc.y().floor() as usize,
            color,
        );
        current_loc = current_loc.transform(&rotation);
    }

    let mut output = File::create(filename)?;
    write!(output, "{}", canvas.to_ppm().whole_file())
}
