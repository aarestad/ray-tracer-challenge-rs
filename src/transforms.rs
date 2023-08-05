use nalgebra::Matrix4;

#[derive(Debug, Copy, Clone)]
pub enum RotationAxis {
    X,
    Y,
    Z,
}

// TODO fluent api?
nofmt::pls! {
    pub fn identity() -> Matrix4<f32> {
        Matrix4::identity()
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Matrix4<f32> {
        // remember, column-major!
        Matrix4::from_vec(vec![
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            x,  y,  z,  1.,
        ])
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Matrix4<f32> {
        Matrix4::from_vec(vec![
            x,  0., 0., 0.,
            0., y,  0., 0.,
            0., 0., z,  0.,
            0., 0., 0., 1.,
        ])
    }

    pub fn rotation(axis: RotationAxis, r: f32) -> Matrix4<f32> {
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

    pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix4<f32> {
        Matrix4::from_vec(vec![
            1., yx, zx, 0.,
            xy, 1., zy, 0.,
            xz, yz, 1., 0.,
            0., 0., 0., 1.,
        ])
    }
}
