use crate::{color::Color, transforms::Transform, tuple::Point};

use super::Pattern;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Gradient {
    pub start: Color,
    pub end: Color,
    transform: Transform,
}

impl Gradient {
    pub fn new(start: Color, end: Color, transform: Transform) -> Self {
        Self {
            start,
            end,
            transform,
        }
    }
}

impl Pattern for Gradient {
    fn transform(&self) -> Option<&Transform> {
        Some(&self.transform)
    }

    fn local_color_at(&self, p: &Point) -> Color {
        self.start + (self.end - self.start) * (p.x() - p.x().floor())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        color::{Color, BLACK, WHITE},
        patterns::Pattern,
        transforms::identity,
        tuple::Point,
    };

    use super::Gradient;

    #[test]
    fn gradient_lerps_between_colors() {
        let pattern = Gradient::new(WHITE, BLACK, identity());

        assert_eq!(pattern.local_color_at(&Point::point(0., 0., 0.)), WHITE);
        assert_eq!(
            pattern.local_color_at(&Point::point(0.25, 0., 0.)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.local_color_at(&Point::point(0.5, 0., 0.)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.local_color_at(&Point::point(0.75, 0., 0.)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
