use crate::color::Color;

#[derive(Default, Debug)]
pub struct Canvas {
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut rows: Vec<Vec<Color>> = vec![];

        for _ in 0..height {
            let mut row: Vec<Color> = vec![];

            for _ in 0..width {
                row.push(Color::default());
            }

            rows.push(row);
        }

        Canvas { pixels: rows }
    }

    pub fn width(&self) -> usize {
        self.pixels[0].len()
    }

    pub fn height(&self) -> usize {
        self.pixels.len()
    }
}
