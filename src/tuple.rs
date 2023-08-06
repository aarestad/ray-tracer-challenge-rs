use std::{
    fmt::Display,
    num::ParseFloatError,
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

use crate::transforms::translation;

use approx::{abs_diff_eq, AbsDiffEq};
use nalgebra::{Matrix4, Vector3, Vector4};
use regex::Regex;

use crate::util::EPSILON;

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Tuple(Vector4<f32>);

pub type Point = Tuple;
pub type Vector = Tuple;

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_type = match self {
            _ if self.is_point() => "point",
            _ if self.is_vector() => "vector",
            _ => "tuple",
        };

        if self_type == "tuple" {
            f.write_fmt(format_args!("tuple({})", self.0))
        } else {
            f.write_fmt(format_args!(
                "{}({}, {}, {})",
                self_type, self.0[0], self.0[1], self.0[2],
            ))
        }
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple(self.0 + rhs.0)
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple(-self.0)
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple(self.0 - rhs.0)
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Tuple(self.0 * rhs)
    }
}

impl Div<f32> for Tuple {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Tuple(self.0 / rhs)
    }
}

impl FromStr for Tuple {
    type Err = String;

    /// Converts from "(point|vector)(x, y, z)" or "tuple(x, y, z, w)" to a Tuple
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parens_contents_re = Regex::new(r"\((.+)\)").expect("bad regex");

        let Some(args_group) = parens_contents_re.captures(s) else {
            return Err(s.to_string());
        };

        let args_str = &args_group[1];

        let args: Vec<Result<f32, ParseFloatError>> = args_str
            .replace(' ', "")
            .split(',')
            .map(f32::from_str)
            .collect();

        if args.iter().any(|a| a.is_err()) {
            return Err(s.to_string());
        }

        let mut t = args.iter().map(|a| *a.as_ref().unwrap());

        match s {
            s if s.starts_with("tuple") => Ok(Tuple(Vector4::from_iterator(t))),
            s if s.starts_with("point") => Ok(Tuple::point(
                t.next().unwrap(),
                t.next().unwrap(),
                t.next().unwrap(),
            )),
            s if s.starts_with("vector") => Ok(Tuple::vector(
                t.next().unwrap(),
                t.next().unwrap(),
                t.next().unwrap(),
            )),
            _ => Err(s.to_string()),
        }
    }
}

impl AbsDiffEq for Tuple {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.0, other.0, epsilon = epsilon)
    }
}

impl Point {
    pub const fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn view_transform(&self, to: &Tuple, up: &Tuple) -> Matrix4<f32> {
        let forward = (*to - *self).normalize();
        let upn = up.normalize();
        let left = forward.cross(&upn);
        let true_up = left.cross(&forward);

        nofmt::pls! {
            let orientation = Matrix4::new(
                left.x(),     left.y(),     left.z(),    0.,
                true_up.x(),  true_up.y(),  true_up.z(), 0.,
               -forward.x(), -forward.y(), -forward.z(), 0.,
                0.,           0.,           0.,     1.
            );
        }

        orientation * translation(-self.x(), -self.y(), -self.z())
    }
}

impl Vector {
    pub const fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn cross(&self, rhs_t: &Tuple) -> Tuple {
        // cross product only works with 3D vectors
        let lhs = Vector3::new(self.0.x, self.0.y, self.0.z);
        let rhs = Vector3::new(rhs_t.0.x, rhs_t.0.y, rhs_t.0.z);
        let prod = lhs.cross(&rhs);

        Tuple::vector(prod[0], prod[1], prod[2])
    }
}

impl Tuple {
    const fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple(Vector4::new(x, y, z, w))
    }

    pub fn x(&self) -> f32 {
        self.0.x
    }

    pub fn y(&self) -> f32 {
        self.0.y
    }

    pub fn z(&self) -> f32 {
        self.0.z
    }

    pub fn w(&self) -> f32 {
        self.0.w
    }

    pub fn is_point(&self) -> bool {
        self.0.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.0.w == 0.0
    }

    pub fn magnitude(&self) -> f32 {
        self.0.magnitude()
    }

    pub fn normalize(self) -> Tuple {
        Tuple(self.0.normalize())
    }

    pub fn dot(&self, rhs: &Tuple) -> f32 {
        self.0.dot(&rhs.0)
    }

    pub fn transform(&self, transform_matrix: &Matrix4<f32>) -> Tuple {
        let original_w = self.0.w;
        let mut t = Tuple(transform_matrix * self.0);
        t.0.w = original_w; // preserve the point/vectorness
        t
    }

    pub fn reflect(&self, normal: &Tuple) -> Tuple {
        *self - *normal * 2. * self.dot(normal)
    }
}
