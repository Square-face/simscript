use bevy::{ecs::component::Component, math::{Mat3, Quat, Vec3}};

#[derive(Component, Debug)]
pub struct Inertia(pub Mat3);

impl Inertia {
    pub fn cylinder_z(height: f32, radius: f32, mass: f32) -> Self {
        let h2 = height.powi(2);
        let r2 = radius.powi(2);
        let m = mass;

        let side = m * h2 / 12.0 + m * r2 / 4.0;
        let front = m * r2 / 2.0;

        Self(Mat3::from_cols_array_2d(&[
            [side, 0.0, 0.0],
            [0.0, side, 0.0],
            [0.0, 0.0, front],
        ]))
    }

    pub fn cylinder_y(height: f32, radius: f32, mass: f32) -> Self {
        let h2 = height.powi(2);
        let r2 = radius.powi(2);
        let m = mass;

        let side = m * h2 / 12.0 + m * r2 / 4.0;
        let front = m * r2 / 2.0;

        Self(Mat3::from_cols_array_2d(&[
            [side, 0.0, 0.0],
            [0.0, front, 0.0],
            [0.0, 0.0, side],
        ]))
    }

    pub fn cylinder_x(height: f32, radius: f32, mass: f32) -> Self {
        let h2 = height.powi(2);
        let r2 = radius.powi(2);
        let m = mass;

        let side = m * h2 / 12.0 + m * r2 / 4.0;
        let front = m * r2 / 2.0;

        Self(Mat3::from_cols_array_2d(&[
            [front, 0.0, 0.0],
            [0.0, side, 0.0],
            [0.0, 0.0, side],
        ]))
    }

    pub fn get_angular_acceleration(&self, torque: Vec3) -> Vec3 {
        self.0.inverse().mul_vec3(torque)
    }
}
