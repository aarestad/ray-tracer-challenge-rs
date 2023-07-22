use std::{
    num::ParseFloatError,
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

use regex::Regex;

use crate::util::approx;

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(PartialEq)]
enum TupleType {
    Point,
    Vector,
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f32> for Tuple {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[derive(Debug)]
pub struct ParseTupleError;

impl FromStr for Tuple {
    type Err = ParseTupleError;

    /// Converts from "(point|vector)(x, y, z)" or "tuple(x, y, z, w)" to a Tuple
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parens_contents_re = Regex::new(r"\((.+)\)").expect("bad regex");

        let Some(args_group) = parens_contents_re.captures(s) else {
            return Err(ParseTupleError)
        };

        let args_str = &args_group[1];

        let args: Vec<Result<f32, ParseFloatError>> = args_str
            .replace(' ', "")
            .split(',')
            .map(f32::from_str)
            .collect();

        if args.iter().any(|a| a.is_err()) {
            return Err(ParseTupleError);
        }

        match s {
            s if s.starts_with("tuple") => Ok(Tuple {
                x: *args[0].as_ref().unwrap(),
                y: *args[1].as_ref().unwrap(),
                z: *args[2].as_ref().unwrap(),
                w: *args[3].as_ref().unwrap(),
            }),
            s if s.starts_with("point") => Ok(Tuple::point(
                *args[0].as_ref().unwrap(),
                *args[1].as_ref().unwrap(),
                *args[2].as_ref().unwrap(),
            )),
            s if s.starts_with("vector") => Ok(Tuple::vector(
                *args[0].as_ref().unwrap(),
                *args[1].as_ref().unwrap(),
                *args[2].as_ref().unwrap(),
            )),
            _ => Err(ParseTupleError),
        }
    }
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::new(x, y, z, 0.0)
    }

    fn tuple_type(&self) -> TupleType {
        match self.w {
            w if w == 0.0 => TupleType::Vector,
            w if w == 1.0 => TupleType::Point,
            _ => panic!("bad value for w: {}", self.w),
        }
    }

    pub fn approx_eq(&self, rhs: Tuple) -> bool {
        approx(self.x, rhs.x)
            && approx(self.y, rhs.y)
            && approx(self.z, rhs.z)
            && approx(self.w, rhs.w)
    }

    pub fn is_point(&self) -> bool {
        self.tuple_type() == TupleType::Point
    }

    pub fn is_vector(&self) -> bool {
        self.tuple_type() == TupleType::Vector
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Tuple {
        let mag = self.magnitude();

        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot(&self, rhs: Tuple) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn cross(self, rhs: Tuple) -> Tuple {
        assert!(self.is_vector(), "must use vectors in cross");
        assert!(rhs.is_vector(), "must use vectors in cross");

        Tuple::vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}
