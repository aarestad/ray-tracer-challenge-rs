use nalgebra::Matrix4;

use crate::util::RayTracerFloat;

pub type Transform = Matrix4<RayTracerFloat>;

#[derive(Debug, Copy, Clone)]
pub enum RotationAxis {
    X,
    Y,
    Z,
}

// TODO fluent api?
nofmt::pls! {
    pub fn identity() -> Transform {
        Matrix4::identity()
    }

    pub fn translation(x: RayTracerFloat, y: RayTracerFloat, z: RayTracerFloat) -> Transform {
        // remember, column-major!
        Matrix4::from_vec(vec![
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            x,  y,  z,  1.,
        ])
    }

    pub fn scaling(x: RayTracerFloat, y: RayTracerFloat, z: RayTracerFloat) ->Transform {
        Matrix4::from_vec(vec![
            x,  0., 0., 0.,
            0., y,  0., 0.,
            0., 0., z,  0.,
            0., 0., 0., 1.,
        ])
    }

    pub fn rotation(axis: RotationAxis, r: RayTracerFloat) -> Transform {
        match axis {
            RotationAxis::X => Matrix4::from_vec(vec![
                1., 0.,       0.,      0.,
                0., r.cos(),  r.sin(), 0.,
                0., -r.sin(), r.cos(), 0.,
                0., 0.,       0.,      1.,
            ]),
            RotationAxis::Y => Matrix4::from_vec(vec![
                r.cos(), 0., -r.sin(), 0.,
                0.,      1., 0.,       0.,
                r.sin(), 0., r.cos(),  0.,
                0.,      0., 0.,       1.,
            ]),
            RotationAxis::Z => Matrix4::from_vec(vec![
                r.cos(),  r.sin(), 0., 0.,
                -r.sin(), r.cos(), 0., 0.,
                0.,       0.,      1., 0.,
                0.,       0.,      0., 1.,
           ]),
        }
    }

    pub fn shearing(xy: RayTracerFloat, xz: RayTracerFloat, yx: RayTracerFloat, yz: RayTracerFloat, zx: RayTracerFloat, zy: RayTracerFloat) -> Transform {
        Matrix4::from_vec(vec![
            1., yx, zx, 0.,
            xy, 1., zy, 0.,
            xz, yz, 1., 0.,
            0., 0., 0., 1.,
        ])
    }
}
