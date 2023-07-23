use std::{
    fmt::Display,
    num::ParseFloatError,
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

use nalgebra::{Vector3, Vector4};
use regex::Regex;

use crate::util::approx;

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Tuple(Vector4<f32>);

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

#[derive(PartialEq)]
enum TupleType {
    Point,
    Vector,
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

        let t = args.iter().map(|a| *a.as_ref().unwrap());

        match s {
            s if s.starts_with("tuple") => Ok(Tuple(Vector4::from_iterator(t))),
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
            _ => Err(s.to_string()),
        }
    }
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple(Vector4::from_vec(vec![x, y, z, w]))
    }

    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn w(&self) -> f32 {
        self.0[3]
    }

    fn tuple_type(&self) -> TupleType {
        match self.w() {
            w if w == 0.0 => TupleType::Vector,
            w if w == 1.0 => TupleType::Point,
            _ => panic!("bad value for w: {}", self.w()),
        }
    }

    pub fn approx_eq(&self, rhs: &Tuple) -> bool {
        approx(self.x(), rhs.x())
            && approx(self.y(), rhs.y())
            && approx(self.z(), rhs.z())
            && approx(self.w(), rhs.w())
    }

    pub fn is_point(&self) -> bool {
        self.tuple_type() == TupleType::Point
    }

    pub fn is_vector(&self) -> bool {
        self.tuple_type() == TupleType::Vector
    }

    pub fn magnitude(&self) -> f32 {
        self.0.magnitude()
    }

    pub fn normalize(self) -> Tuple {
        Tuple(self.0.normalize())
    }

    #[allow(dead_code)]
    pub fn dot(&self, rhs: Tuple) -> f32 {
        self.0.dot(&rhs.0)
    }

    #[allow(dead_code)]
    pub fn cross(self, rhs_t: Tuple) -> Tuple {
        assert!(self.is_vector(), "must use vectors in cross");
        assert!(rhs_t.is_vector(), "must use vectors in cross");

        let lhs = Vector3::new(self.0[0], self.0[1], self.0[2]);
        let rhs = Vector3::new(rhs_t.0[0], rhs_t.0[1], rhs_t.0[2]);

        let prod = lhs.cross(&rhs);

        Tuple::vector(prod[0], prod[1], prod[2])
    }
}
