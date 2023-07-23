use crate::{
    canvas::Canvas,
    color::Color,
    transforms::{rotation, translation, RotationAxis},
    tuple::Tuple,
};
use std::f32::consts::TAU;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

pub fn ch4_playground(filename: &Path) -> Result<()> {
    let mut canvas = Canvas::new(500, 500);
    let color = Color::new(1., 1., 1.);

    let origin = Tuple::point(0., 0., 0.);
    let mut current_loc = origin.transform(&translation(200., 0., 0.));

    let canvas_translation = translation(250., 250., 0.);
    let rotation = rotation(RotationAxis::Z, TAU / 12.);

    for _ in 0..12 {
        let canvas_loc = current_loc.transform(&canvas_translation);
        canvas.write(canvas_loc.x(), canvas_loc.y(), color);
        current_loc = current_loc.transform(&rotation);
    }

    let mut output = File::create(filename)?;
    write!(output, "{}", canvas.to_ppm().whole_file())
}
