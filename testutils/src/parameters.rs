use core::convert::Infallible;
use cucumber::Parameter;
use std::f64::consts::PI;
use std::fmt::{Debug, Display};
use std::str::FromStr;
use crate::RayTracerFloat;

use ray_tracer_challenge_rs::transforms::RotationAxis;

pub trait SingleValue<T> {
    fn val(&self) -> T;
}

macro_rules! impl_single_value {
    ($t:ty,$rt:ty) => {
        impl SingleValue<$rt> for $t {
            fn val(&self) -> $rt {
                self.0
            }
        }
    };
}

// TODO make this work some day
#[derive(Debug, Parameter)]
#[param(regex = r"(-?((π|√\d+)/\d+)|(\d+(\.\d*)?))")]
pub struct MathExpr(RayTracerFloat);
impl_single_value!(MathExpr, RayTracerFloat);

impl FromStr for MathExpr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            println!("empty string...");
            return Ok(MathExpr(0.));
        }

        if let Ok(val) = RayTracerFloat::from_str(s) {
            return Ok(MathExpr(val));
        }

        let mut chars = s.chars();
        let negate = chars.next().expect("empty string?") == '-';

        let rest = if negate {
            chars.collect::<String>()
        } else {
            s.to_string()
        };

        let parts = rest.split('.').map(|s| s.trim()).collect::<Vec<&str>>();

        let [dividend_str, divisor_str] = parts[0..2] else {
          return Err("bad format".to_string());
        };

        let dividend = match dividend_str {
            "π" => PI,
            d if dividend_str.starts_with('√') => {
                let operand = d.chars().skip(1).collect::<String>();
                RayTracerFloat::from_str(&operand).unwrap().sqrt()
            }
            _ => return Err(format!("bad dividend: {}", dividend_str)),
        };

        let divisor = RayTracerFloat::from_str(divisor_str).unwrap();

        let result = if negate {
            -dividend / divisor
        } else {
            dividend / divisor
        };

        Ok(MathExpr(result))
    }
}

#[derive(Debug, Parameter, Copy, Clone)]
#[param(regex = r"x|y|z")]
pub struct Axis(RotationAxis);
impl_single_value!(Axis, RotationAxis);

impl FromStr for Axis {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Axis(match s {
            "x" => RotationAxis::X,
            "y" => RotationAxis::Y,
            "z" => RotationAxis::Z,
            _ => unreachable!(),
        }))
    }
}

#[derive(Debug, Parameter)]
#[param(regex = r"x|y|z|w")]
pub enum TupleProperty {
    X,
    Y,
    Z,
    W,
}

impl FromStr for TupleProperty {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => TupleProperty::X,
            "y" => TupleProperty::Y,
            "z" => TupleProperty::Z,
            "w" => TupleProperty::W,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Parameter)]
#[param(regex = r"red|green|blue")]
pub enum ColorProperty {
    Red,
    Green,
    Blue,
}

impl FromStr for ColorProperty {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "red" => ColorProperty::Red,
            "green" => ColorProperty::Green,
            "blue" => ColorProperty::Blue,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Parameter)]
#[param(regex = r"[+-]")]
pub enum AddSub {
    Add,
    Sub,
}

impl FromStr for AddSub {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(AddSub::Add),
            "-" => Ok(AddSub::Sub),
            _ => unreachable!(),
        }
    }
}

impl Display for AddSub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AddSub::Add => "+",
            AddSub::Sub => "-",
        })
    }
}

#[derive(Debug, Parameter)]
#[param(regex = r"[*/]")]
pub enum MulDiv {
    Mul,
    Div,
}

impl FromStr for MulDiv {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(MulDiv::Mul),
            "/" => Ok(MulDiv::Div),
            _ => unreachable!(),
        }
    }
}

impl Display for MulDiv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MulDiv::Mul => "*",
            MulDiv::Div => "/",
        })
    }
}
