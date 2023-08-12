use crate::{color::Color, transforms::Transform, tuple::Point};

use super::Pattern;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TestPattern {
    transform: Transform,
}

impl TestPattern {
    pub fn new(transform: Transform) -> Self {
        Self { transform }
    }
}

impl Pattern for TestPattern {
    fn transform(&self) -> Option<&Transform> {
        Some(&self.transform)
    }

    fn local_color_at(&self, p: &Point) -> Color {
        // to test that the transformation is being applied
        Color::new(p.x(), p.y(), p.z())
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::{
        color::Color,
        material::MaterialBuilder,
        objects::{Object, Sphere},
        transforms::{identity, scaling, translation},
        tuple::Point,
    };

    use super::{Pattern, TestPattern};

    #[test]
    fn pattern_with_object_transform() {
        let p = TestPattern::new(identity());

        let s = Sphere::new(
            scaling(2., 2., 2.),
            Rc::new(MaterialBuilder::default().pattern(Rc::new(p)).build()),
        );

        let c = p.color_at(&s, &Point::point(2., 3., 4.));

        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn pattern_with_pattern_transform() {
        let p = TestPattern::new(scaling(2., 2., 2.));

        let s = Sphere::new(
            identity(),
            Rc::new(MaterialBuilder::default().pattern(Rc::new(p)).build()),
        );

        let c = p.color_at(&s, &Point::point(2., 3., 4.));

        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn pattern_with_object_and_pattern_transform() {
        let p = TestPattern::new(translation(0.5, 1., 1.5));

        let s = Sphere::new(
            scaling(2., 2., 2.),
            Rc::new(MaterialBuilder::default().pattern(Rc::new(p)).build()),
        );

        let c = p.color_at(&s, &Point::point(2.5, 3., 3.5));

        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
