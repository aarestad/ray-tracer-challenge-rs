use std::io::Result;
use std::path::Path;

mod canvas;
mod clock_face;
mod color;
mod ppm;
mod transforms;
mod tuple;
mod util;
mod virtual_cannon;

fn main() -> Result<()> {
    virtual_cannon::ch2_playground(Path::new("trajectory.ppm"))?;
    clock_face::ch4_playground(Path::new("clock_face.ppm"))
}
