use std::{
    num::ParseFloatError,
    ops::{Add, Neg, Sub},
    str::FromStr,
};

use regex::Regex;

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

#[derive(Debug)]
pub struct ParseTupleError;

impl FromStr for Tuple {
    type Err = ParseTupleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parens_contents_re = Regex::new(r"\((.+)\)").expect("bad regex");

        let Some(args_group) = parens_contents_re.captures(s) else {
            return Err(ParseTupleError)
        };

        let args_str = &args_group[1];

        let args: Vec<Result<f32, ParseFloatError>> = args_str
            .replace(" ", "")
            .split(",")
            .map(|arg| f32::from_str(arg))
            .collect();

        if args.clone().iter().any(|a| a.is_err()) {
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
    fn tuple_type(&self) -> TupleType {
        match self.w {
            w if w == 0.0 => TupleType::Vector,
            w if w == 1.0 => TupleType::Point,
            _ => panic!("bad value for w: {}", self.w),
        }
    }

    pub fn is_point(&self) -> bool {
        return self.tuple_type() == TupleType::Point;
    }

    pub fn is_vector(&self) -> bool {
        return self.tuple_type() == TupleType::Vector;
    }

    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }
}
