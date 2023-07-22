const EPSILON: f32 = 0.00001;

pub fn approx(lhs: f32, rhs: f32) -> bool {
    (lhs - rhs).abs() < EPSILON
}
