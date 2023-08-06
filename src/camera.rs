use crate::{canvas::Canvas, ray::Ray, tuple::Point, util::RayTracerFloat, world::World};
use nalgebra::Matrix4;

#[derive(Debug, PartialEq)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: RayTracerFloat,
    pub transform: Matrix4<RayTracerFloat>,
    pub half_width: RayTracerFloat,
    pub half_height: RayTracerFloat,
    pub pixel_size: RayTracerFloat,
}

impl Camera {
    pub fn new(
        hsize: usize,
        vsize: usize,
        field_of_view: RayTracerFloat,
        transform: Matrix4<RayTracerFloat>,
    ) -> Self {
        let half_view = (field_of_view / 2.).tan();
        let aspect = (hsize as RayTracerFloat) / (vsize as RayTracerFloat);

        let (half_width, half_height) = if aspect >= 1. {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.) / (hsize as RayTracerFloat);

        Self {
            hsize,
            vsize,
            field_of_view,
            transform,
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // # the offset from the edge of the canvas to the pixel's center
        let xoffset = (x as RayTracerFloat + 0.5) * self.pixel_size;
        let yoffset = (y as RayTracerFloat + 0.5) * self.pixel_size;

        // # the untransformed coordinates of the pixel in world space.
        // # (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // # using the camera matrix, transform the canvas point and the origin,
        // # and then compute the ray's direction vector.
        // # (remember that the canvas is at z=-1)
        let xform_inv = &self
            .transform
            .try_inverse()
            .expect("cannot invert camera transform");

        let pixel = Point::point(world_x, world_y, -1.).transform(xform_inv);
        let origin = Point::point(0., 0., 0.).transform(xform_inv);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image.write(x, y, color);
            }
        }

        image
    }
}
