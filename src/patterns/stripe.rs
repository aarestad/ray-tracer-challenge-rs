use crate::{color::Color, transforms::Transform, tuple::Point};

use super::Pattern;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Stripe {
    pub even: Color,
    pub odd: Color,
    transform: Transform,
}

impl Stripe {
    pub fn new(even: Color, odd: Color, transform: Transform) -> Self {
        Self {
            even,
            odd,
            transform,
        }
    }
}

impl Pattern for Stripe {
    fn as_stripe(&self) -> &Stripe {
        self
    }

    fn transform(&self) -> Option<&Transform> {
        Some(&self.transform)
    }

    fn local_color_at(&self, p: &Point) -> Color {
        if p.x().floor() % 2. == 0. {
            self.even
        } else {
            self.odd
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::{
        color::{BLACK, WHITE},
        material::Material,
        objects::Object,
        transforms::{identity, scaling, translation},
        tuple::Point,
    };

    use super::{Pattern, Stripe};

    #[test]
    fn stripe_with_object_transform() {
        let p = Stripe::new(WHITE, BLACK, identity());
        let s = Object::Sphere(scaling(2., 2., 2.), Rc::new(Material::default()));
        let point = Point::point(1.5, 0., 0.);
        let c = p.color_at(&s, &point);
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripe_with_pattern_transform() {
        let p = Stripe::new(WHITE, BLACK, scaling(2., 2., 2.));
        let s = Object::Sphere(identity(), Rc::new(Material::default()));
        let point = Point::point(1.5, 0., 0.);
        let c = p.color_at(&s, &point);
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripe_with_pattern_and_obj_transform() {
        let p = Stripe::new(WHITE, BLACK, translation(0.5, 0., 0.));
        let s = Object::Sphere(scaling(2., 2., 2.), Rc::new(Material::default()));
        let point = Point::point(2.5, 0., 0.);
        let c = p.color_at(&s, &point);
        assert_eq!(c, WHITE);
    }
}
