use bevy::{
    ecs::component::Component,
    math::{Mat3, Vec3},
};

use crate::force::Torque;

/// An objects mass and inertia tesnsor.
///
/// Used when calculating forces and moments being applied to get a correct rotational and
/// translational acceleration
#[derive(Component, Debug)]
pub struct Inertia(pub Mat3);

impl Inertia {
    /// Returns a cylinder with the height going in the x direction
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

    /// Returns a cylinder with the height going in the y direction
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

    /// Returns a cylinder with the height going in the z direction
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

    /// Computes the resulting angular acceleration when applying a certain torque
    pub fn get_angular_acceleration(&self, torque: Torque) -> Vec3 {
        self.0.inverse().mul_vec3(torque.0)
    }
}

#[cfg(test)]
mod constructors {
    #[cfg(test)]
    mod specific {

        use super::super::Inertia;
        use bevy::math::Mat3;

        #[test]
        fn thin() {
            assert_eq!(
                Inertia::cylinder_x(4.0, 0.5, 20.0).0,
                Mat3::from_cols_array_2d(&[
                    [5.0 / 2.0, 0.0, 0.0],
                    [0.0, 335.0 / 12.0, 0.0],
                    [0.0, 0.0, 335.0 / 12.0],
                ])
            );

            assert_eq!(
                Inertia::cylinder_y(4.0, 0.5, 20.0).0,
                Mat3::from_cols_array_2d(&[
                    [335.0 / 12.0, 0.0, 0.0],
                    [0.0, 5.0 / 2.0, 0.0],
                    [0.0, 0.0, 335.0 / 12.0],
                ])
            );

            assert_eq!(
                Inertia::cylinder_z(4.0, 0.5, 20.0).0,
                Mat3::from_cols_array_2d(&[
                    [335.0 / 12.0, 0.0, 0.0],
                    [0.0, 335.0 / 12.0, 0.0],
                    [0.0, 0.0, 5.0 / 2.0],
                ])
            );
        }
    }

    #[cfg(test)]
    mod unit {
        use super::super::Inertia;
        use bevy::math::Mat3;

        #[test]
        fn x_cylinder() {
            let cyl = Inertia::cylinder_x(1.0, 1.0, 1.0);
            assert_eq!(
                cyl.0,
                Mat3::from_cols_array_2d(&[
                    [1.0 / 2.0, 0.0, 0.0],
                    [0.0, 1.0 / 3.0, 0.0],
                    [0.0, 0.0, 1.0 / 3.0]
                ])
            )
        }

        #[test]
        fn y_cylinder() {
            let cyl = Inertia::cylinder_y(1.0, 1.0, 1.0);
            assert_eq!(
                cyl.0,
                Mat3::from_cols_array_2d(&[
                    [1.0 / 3.0, 0.0, 0.0],
                    [0.0, 1.0 / 2.0, 0.0],
                    [0.0, 0.0, 1.0 / 3.0]
                ])
            )
        }

        #[test]
        fn z_cylinder() {
            let cyl = Inertia::cylinder_z(1.0, 1.0, 1.0);
            assert_eq!(
                cyl.0,
                Mat3::from_cols_array_2d(&[
                    [1.0 / 3.0, 0.0, 0.0],
                    [0.0, 1.0 / 3.0, 0.0],
                    [0.0, 0.0, 1.0 / 2.0]
                ])
            )
        }
    }
}
