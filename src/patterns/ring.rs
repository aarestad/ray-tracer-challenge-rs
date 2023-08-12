use crate::{color::Color, transforms::Transform, tuple::Point};

use super::Pattern;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ring {
    pub even: Color,
    pub odd: Color,
    transform: Transform,
}

impl Ring {
    pub fn new(even: Color, odd: Color, transform: Transform) -> Self {
        Self {
            even,
            odd,
            transform,
        }
    }
}

impl Pattern for Ring {
    fn transform(&self) -> Option<&Transform> {
        Some(&self.transform)
    }

    fn local_color_at(&self, p: &Point) -> Color {
        let dist_from_origin = (p.x().powi(2) + p.z().powi(2)).sqrt();

        if dist_from_origin % 2. == 0. {
            self.even
        } else {
            self.odd
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        color::{BLACK, WHITE},
        patterns::Pattern,
        transforms::identity,
        tuple::Point,
    };

    use super::Ring;

    #[test]
    fn ring_should_extend_in_both_x_and_z() {
        let p = Ring::new(WHITE, BLACK, identity());

        assert_eq!(p.local_color_at(&Point::point(0., 0., 0.)), WHITE);
        assert_eq!(p.local_color_at(&Point::point(1., 0., 0.)), BLACK);
        assert_eq!(p.local_color_at(&Point::point(0., 0., 1.)), BLACK);
        assert_eq!(p.local_color_at(&Point::point(0.708, 0., 0.708)), BLACK);
    }
}
