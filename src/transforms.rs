use nalgebra::Matrix4;

nofmt::pls! {
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
}
