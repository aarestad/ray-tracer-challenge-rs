use crate::transforms::identity;
use nalgebra::Matrix4;

#[derive(Debug, PartialEq)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f32,
    pub transform: Matrix4<f32>,
    pub half_width: f32,
    pub half_height: f32,
    pub pixel_size: f32,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Self {
        let half_view = (field_of_view / 2.).tan();
        let aspect = (hsize as f32) / (vsize as f32);

        let (half_width, half_height) = if aspect >= 1. {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.) / (hsize as f32);

        Self {
            hsize,
            vsize,
            field_of_view,
            transform: identity(),
            half_width,
            half_height,
            pixel_size,
        }
    }
}
