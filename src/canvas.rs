use crate::color::Color;
use crate::ppm::PPM;

#[derive(Default, Debug)]
pub struct Canvas {
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    /// Creates a new width * height Canvas with every pixel black
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

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }

    pub fn write(&mut self, x: usize, y: usize, c: Color) {
        println!("setting {}, {} to {}", x, y, c);
        self.pixels[y][x] = c;
    }

    pub fn to_ppm(&self) -> PPM {
        let mut ppm = PPM::new(self.width(), self.height());

        for row in self.pixels.iter() {
            let mut row_line = row
                .iter()
                .map(|c| c.as_ppm_string())
                .collect::<Vec<_>>()
                .join(" ");

            while row_line.len() > 70 {
                let (l, rest) = row_line.split_at(70);
                ppm.add_line(format!("{}\n", l.to_string()));
                row_line = rest.to_string();
            }

            ppm.add_line(format!("{}\n", row_line));
        }

        ppm
    }
}
