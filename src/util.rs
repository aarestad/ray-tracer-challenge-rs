use crate::tuple::Point;
use std::f64::consts::{PI, TAU};

pub type RayTracerFloat = f64;

pub const EPSILON: RayTracerFloat = 0.0001;

pub fn get_sphere_uv(p: Point) -> (RayTracerFloat, RayTracerFloat) {
    let theta = (-p.y()).acos();
    let phi = (-p.z()).atan2(p.x()) + PI;

    (phi / TAU, theta / PI)
}
