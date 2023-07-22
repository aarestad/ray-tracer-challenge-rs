#[derive(Debug, Default)]
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
}
