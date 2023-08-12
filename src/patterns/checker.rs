use crate::{color::Color, transforms::Transform, tuple::Point};

use super::Pattern;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Checker {
    pub even: Color,
    pub odd: Color,
    transform: Transform,
}

impl Checker {
    pub fn new(even: Color, odd: Color, transform: Transform) -> Self {
        Self {
            even,
            odd,
            transform,
        }
    }
}

impl Pattern for Checker {
    fn transform(&self) -> Option<&Transform> {
        Some(&self.transform)
    }

    fn local_color_at(&self, p: &Point) -> Color {
        let floor_sum = p.x().floor() + p.y().floor() + p.z().floor();

        if floor_sum % 2. == 0. {
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
        transforms::identity,
        tuple::Point,
    };

    use super::{Checker, Pattern};

    #[test]
    fn checkers_should_repeat_in_each_dim() {
        let p = Checker::new(WHITE, BLACK, identity());

        assert_eq!(p.local_color_at(&Point::point(0., 0., 0.)), WHITE);
        assert_eq!(p.local_color_at(&Point::point(0.99, 0., 0.)), WHITE);
        assert_eq!(p.local_color_at(&Point::point(1.01, 0., 0.)), BLACK);

        assert_eq!(p.local_color_at(&Point::point(0., 0.99, 0.)), WHITE);
        assert_eq!(p.local_color_at(&Point::point(0., 1.01, 0.)), BLACK);

        assert_eq!(p.local_color_at(&Point::point(0., 0., 0.99)), WHITE);
        assert_eq!(p.local_color_at(&Point::point(0., 0., 1.01)), BLACK);
    }
}
