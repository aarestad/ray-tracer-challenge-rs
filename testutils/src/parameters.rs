use core::convert::Infallible;
use cucumber::Parameter;
use std::f32::consts::PI;
use std::str::FromStr;

use ray_tracer_challenge_rs::transforms::RotationAxis;

#[derive(Debug, Parameter)]
#[param(regex = r"(-?(π|√\d+)\s*/\s*\d+)|(-?\d+)")]
pub struct MathExpr(f32);

impl MathExpr {
    pub fn val(&self) -> f32 {
        self.0
    }
}

impl FromStr for MathExpr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = f32::from_str(s) {
            return Ok(MathExpr(val));
        }

        let mut chars = s.chars();
        let negate = chars.next().expect("empty string?") == '-';

        let rest = if negate {
            chars.collect::<String>()
        } else {
            s.to_string()
        };

        let parts = rest.split("/").map(|s| s.trim()).collect::<Vec<&str>>();

        let [dividend_str, divisor_str] = parts[0..2] else {
          return Err("bad format".to_string());
        };

        let dividend = match dividend_str {
            "π" => PI,
            d if dividend_str.starts_with("√") => {
                let operand = d.chars().skip(1).collect::<String>();
                (f32::from_str(&operand).expect(&format!("bad dividend: {}", dividend_str))).sqrt()
            }
            _ => return Err(format!("bad dividend: {}", dividend_str)),
        };

        let divisor = f32::from_str(&divisor_str).expect(&format!("bad divisor: {}", divisor_str));

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

impl Axis {
    pub fn val(&self) -> RotationAxis {
        self.0
    }
}
