use std::marker::PhantomData;

use bevy::math::Quat;
use bevy::{ecs::component::Component, math::Mat3};

use crate::cordinate_systems::{ConvertCordinateSystem, Global, Local};

use super::acceleration::{Acceleration, AngularAcceleration};
use super::force::{Force, Torque};

/// An objects mass and inertia tesnsor.
///
/// Used when calculating forces and moments being applied to get a correct rotational and
/// translational acceleration
#[derive(Component, Debug)]
pub struct Inertia<CordinateSystem: ConvertCordinateSystem> {
    pub mass: f32,
    pub tensor: Mat3,
    state: PhantomData<CordinateSystem>,
}

impl<S: ConvertCordinateSystem> Inertia<S> {
    /// Calculate the local acceleration from applying a local force on the object
    pub fn get_linear_acceleration(&self, force: &Force<S>) -> Acceleration {
        Acceleration(force.0 / self.mass)
    }

    /// Calculate the resulting angular acceleration when applying a torque
    pub fn get_angular_acceleration(&self, torque: &Torque<S>) -> AngularAcceleration {
        AngularAcceleration(self.tensor.inverse().mul_vec3(torque.0))
    }

    pub fn to_global(self, rot: Quat) -> Inertia<Global> {
        Inertia{
            mass: self.mass,
            tensor: S::mat3_to_global(self.tensor, rot),
            state: PhantomData
        }
    }

    pub fn to_local(self, rot: Quat) -> Inertia<Local> {
        Inertia{
            mass: self.mass,
            tensor: S::mat3_to_local(self.tensor, rot),
            state: PhantomData
        }
    }
}

/// Contrsuctors
impl Inertia<Local> {
    /// Returns a cylinder with the height going in the x direction
    pub fn cylinder_x(height: f32, radius: f32, mass: f32) -> Self {
        let h2 = height.powi(2);
        let r2 = radius.powi(2);
        let m = mass;

        let side = m * h2 / 12.0 + m * r2 / 4.0;
        let front = m * r2 / 2.0;

        Self {
            mass,
            tensor: Mat3::from_cols_array_2d(&[
                [front, 0.0, 0.0],
                [0.0, side, 0.0],
                [0.0, 0.0, side],
            ]),
            state: PhantomData
        }
    }

    /// Returns a cylinder with the height going in the y direction
    pub fn cylinder_y(height: f32, radius: f32, mass: f32) -> Self {
        let h2 = height.powi(2);
        let r2 = radius.powi(2);
        let m = mass;

        let side = m * h2 / 12.0 + m * r2 / 4.0;
        let front = m * r2 / 2.0;

        Self {
            mass,
            tensor: Mat3::from_cols_array_2d(&[
                [side, 0.0, 0.0],
                [0.0, front, 0.0],
                [0.0, 0.0, side],
            ]),
            state: PhantomData
        }
    }

    /// Returns a cylinder with the height going in the z direction
    pub fn cylinder_z(height: f32, radius: f32, mass: f32) -> Self {
        let h2 = height.powi(2);
        let r2 = radius.powi(2);
        let m = mass;

        let side = m * h2 / 12.0 + m * r2 / 4.0;
        let front = m * r2 / 2.0;

        Self {
            mass,
            tensor: Mat3::from_cols_array_2d(&[
                [side, 0.0, 0.0],
                [0.0, side, 0.0],
                [0.0, 0.0, front],
            ]),
            state: PhantomData
        }
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
                Inertia::cylinder_x(4.0, 0.5, 20.0).tensor,
                Mat3::from_cols_array_2d(&[
                    [5.0 / 2.0, 0.0, 0.0],
                    [0.0, 335.0 / 12.0, 0.0],
                    [0.0, 0.0, 335.0 / 12.0],
                ])
            );

            assert_eq!(
                Inertia::cylinder_y(4.0, 0.5, 20.0).tensor,
                Mat3::from_cols_array_2d(&[
                    [335.0 / 12.0, 0.0, 0.0],
                    [0.0, 5.0 / 2.0, 0.0],
                    [0.0, 0.0, 335.0 / 12.0],
                ])
            );

            assert_eq!(
                Inertia::cylinder_z(4.0, 0.5, 20.0).tensor,
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
            let cyl = Inertia::cylinder_x(1.0, 1.0, 1.0).tensor;
            assert_eq!(
                cyl,
                Mat3::from_cols_array_2d(&[
                    [1.0 / 2.0, 0.0, 0.0],
                    [0.0, 1.0 / 3.0, 0.0],
                    [0.0, 0.0, 1.0 / 3.0]
                ])
            )
        }

        #[test]
        fn y_cylinder() {
            let cyl = Inertia::cylinder_y(1.0, 1.0, 1.0).tensor;
            assert_eq!(
                cyl,
                Mat3::from_cols_array_2d(&[
                    [1.0 / 3.0, 0.0, 0.0],
                    [0.0, 1.0 / 2.0, 0.0],
                    [0.0, 0.0, 1.0 / 3.0]
                ])
            )
        }

        #[test]
        fn z_cylinder() {
            let cyl = Inertia::cylinder_z(1.0, 1.0, 1.0).tensor;
            assert_eq!(
                cyl,
                Mat3::from_cols_array_2d(&[
                    [1.0 / 3.0, 0.0, 0.0],
                    [0.0, 1.0 / 3.0, 0.0],
                    [0.0, 0.0, 1.0 / 2.0]
                ])
            )
        }
    }
}
