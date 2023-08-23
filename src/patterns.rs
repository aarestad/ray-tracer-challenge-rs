use std::fmt::Debug;

use crate::{
    color::Color,
    objects::Object,
    transforms::{identity, Transform},
    tuple::Point,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Pattern {
    // TODO cfg[test]
    Test(Transform),
    Stripe {
        transform: Transform,
        even: Color,
        odd: Color,
    },
    Ring {
        transform: Transform,
        even: Color,
        odd: Color,
    },
    Solid(Color),
    Gradient {
        transform: Transform,
        start: Color,
        end: Color,
    },
    Checker {
        transform: Transform,
        even: Color,
        odd: Color,
    },
}

impl Pattern {
    fn transform(&self) -> Option<&Transform> {
        match self {
            Pattern::Test(t) => Some(t),
            Pattern::Stripe { transform, .. }
            | Pattern::Ring { transform, .. }
            | Pattern::Gradient { transform, .. }
            | Pattern::Checker { transform, .. } => Some(transform),
            _ => None,
        }
    }

    pub fn color_at(&self, object: &Object, world_point: &Point) -> Color {
        let object_point = object.world_point_to_local(*world_point);

        let p = if let Some(t) = self.transform() {
            object_point.transform(&t.try_inverse().unwrap())
        } else {
            object_point
        };

        match self {
            Pattern::Solid(c) => *c,
            Pattern::Test(_) =>
            // to test that the transformation is being applied
            {
                Color::new(p.x(), p.y(), p.z())
            }
            Pattern::Stripe {
                transform: _,
                even,
                odd,
            } => {
                if p.x().floor() % 2. == 0. {
                    *even
                } else {
                    *odd
                }
            }
            Pattern::Ring {
                transform: _,
                even,
                odd,
            } => {
                let dist_from_origin = (p.x().powi(2) + p.z().powi(2)).sqrt();

                if dist_from_origin % 2. == 0. {
                    *even
                } else {
                    *odd
                }
            }
            Pattern::Gradient {
                transform: _,
                start,
                end,
            } => *start + (*end - *start) * (p.x() - p.x().floor()),
            Pattern::Checker {
                transform: _,
                even,
                odd,
            } => {
                let floor_sum = p.x().floor() + p.y().floor() + p.z().floor();

                if floor_sum % 2. == 0. {
                    *even
                } else {
                    *odd
                }
            }
        }
    }
}

#[cfg(test)]
pub fn default_test_pattern() -> Pattern {
    Pattern::Test(identity())
}

#[cfg(test)]
mod test {
    use crate::{
        color::{Color, BLACK, WHITE},
        material::{Material, MaterialBuilder},
        objects::{default_sphere, Object},
        transforms::{identity, scaling, translation},
        tuple::Point,
    };

    use super::Pattern;

    #[test]
    fn pattern_with_object_transform() {
        let p = Pattern::Test(identity());

        let s = Object::sphere(
            scaling(2., 2., 2.),
            MaterialBuilder::default().pattern(p).build(),
        );

        let c = p.color_at(&s, &Point::point(2., 3., 4.));

        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn pattern_with_pattern_transform() {
        let p = Pattern::Test(scaling(2., 2., 2.));

        let s = Object::sphere(identity(), MaterialBuilder::default().pattern(p).build());

        let c = p.color_at(&s, &Point::point(2., 3., 4.));

        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn pattern_with_object_and_pattern_transform() {
        let p = Pattern::Test(translation(0.5, 1., 1.5));

        let s = Object::sphere(
            scaling(2., 2., 2.),
            MaterialBuilder::default().pattern(p).build(),
        );

        let c = p.color_at(&s, &Point::point(2.5, 3., 3.5));

        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }

    #[test]
    fn stripe_with_object_transform() {
        let p = Pattern::Stripe {
            even: WHITE,
            odd: BLACK,
            transform: identity(),
        };
        let s = Object::sphere(scaling(2., 2., 2.), Material::default());
        let point = Point::point(1.5, 0., 0.);
        let c = p.color_at(&s, &point);
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripe_with_pattern_transform() {
        let p = Pattern::Stripe {
            even: WHITE,
            odd: BLACK,
            transform: scaling(2., 2., 2.),
        };
        let s = Object::sphere(identity(), Material::default());
        let point = Point::point(1.5, 0., 0.);
        let c = p.color_at(&s, &point);
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripe_with_pattern_and_obj_transform() {
        let p = Pattern::Stripe {
            even: WHITE,
            odd: BLACK,
            transform: translation(0.5, 0., 0.),
        };
        let s = Object::sphere(scaling(2., 2., 2.), Material::default());
        let point = Point::point(2.5, 0., 0.);
        let c = p.color_at(&s, &point);
        assert_eq!(c, WHITE);
    }

    #[test]
    fn ring_should_extend_in_both_x_and_z() {
        let p = Pattern::Ring {
            even: WHITE,
            odd: BLACK,
            transform: identity(),
        };

        let s = default_sphere();

        assert_eq!(p.color_at(&s, &Point::point(0., 0., 0.)), WHITE);
        assert_eq!(p.color_at(&s, &Point::point(1., 0., 0.)), BLACK);
        assert_eq!(p.color_at(&s, &Point::point(0., 0., 1.)), BLACK);
        assert_eq!(p.color_at(&s, &Point::point(0.708, 0., 0.708)), BLACK);
    }

    #[test]
    fn gradient_lerps_between_colors() {
        let pattern = Pattern::Gradient {
            start: WHITE,
            end: BLACK,
            transform: identity(),
        };

        let s = default_sphere();

        assert_eq!(pattern.color_at(&s, &Point::point(0., 0., 0.)), WHITE);
        assert_eq!(
            pattern.color_at(&s, &Point::point(0.25, 0., 0.)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at(&s, &Point::point(0.5, 0., 0.)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at(&s, &Point::point(0.75, 0., 0.)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn checkers_should_repeat_in_each_dim() {
        let p = Pattern::Checker {
            even: WHITE,
            odd: BLACK,
            transform: identity(),
        };

        let s = default_sphere();

        assert_eq!(p.color_at(&s, &Point::point(0., 0., 0.)), WHITE);
        assert_eq!(p.color_at(&s, &Point::point(0.99, 0., 0.)), WHITE);
        assert_eq!(p.color_at(&s, &Point::point(1.01, 0., 0.)), BLACK);

        assert_eq!(p.color_at(&s, &Point::point(0., 0.99, 0.)), WHITE);
        assert_eq!(p.color_at(&s, &Point::point(0., 1.01, 0.)), BLACK);

        assert_eq!(p.color_at(&s, &Point::point(0., 0., 0.99)), WHITE);
        assert_eq!(p.color_at(&s, &Point::point(0., 0., 1.01)), BLACK);
    }
}
