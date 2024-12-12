//! # Inertia Module
//!
//! This module defines the `Inertia` struct and its associated methods for representing
//! and calculating an object's mass and inertia tensor in the context of a physics simulation.
//!
//! The `Inertia` struct is parameterized by a `CoordinateSystem` (either `Global` or `Local`),
//! allowing it to operate in different spatial contexts. It supports operations such as calculating
//! linear and angular accelerations, as well as converting inertia tensors between coordinate systems.
//!
//! ## Key Features
//! - **Mass and Tensor Representation**: Represents the mass as a scalar and the inertia tensor as a 3x3 matrix.
//! - **Force and Torque Calculations**: Provides methods to compute linear and angular accelerations
//!   resulting from applied forces and torques.
//! - **Coordinate System Flexibility**: Supports conversion between local and global coordinate systems.

use bevy::math::Quat;
use bevy::{ecs::component::Component, math::Mat3};
use std::marker::PhantomData;

use crate::components::acceleration::{Acceleration, AngularAcceleration};
use crate::components::force::{Force, Torque};
use crate::coordinate_systems::{CoordinateConvert, CoordinateSystem, Global, Local};

/// Represents an object's mass and inertia tensor.
///
/// The `Inertia` struct is used to calculate both translational and rotational acceleration
/// based on the forces and moments applied to the object. It is parameterized by a `CoordinateSystem`
/// (either [`Global`] or [`Local`]), allowing inertia to be represented in different coordinate systems.
///
/// The inertia tensor is a 3x3 matrix representing the distribution of mass in the object
/// relative to its center of mass, while the mass is a scalar value representing the object's mass.
#[derive(Component, Debug)]
pub struct Inertia<CordinateSystem: CoordinateSystem> {
    /// Mass of the object in kilograms.
    pub mass: f32,

    /// Inertia tensor, represented as a 3x3 matrix.
    pub tensor: Mat3,

    /// Phantom data used to associate the inertia with a specific coordinate system (Global or Local).
    state: PhantomData<CordinateSystem>,
}

impl<S: CoordinateSystem> Inertia<S> {
    /// Calculates the linear acceleration of the object given a force.
    ///
    /// This method computes the linear acceleration using Newton's second law:
    /// `a = F / m`, where `F` is the force applied and `m` is the mass of the object.
    ///
    /// # Arguments
    /// * `force` - A reference to the [`Force<S>`] applied to the object.
    ///
    /// # Returns
    /// An [`Acceleration<S>`] representing the object's linear acceleration.
    #[inline]
    #[must_use]
    pub fn get_linear_acceleration(&self, force: &Force<S>) -> Acceleration<S> {
        Acceleration::new(force.0 / self.mass)
    }

    /// Calculates the angular acceleration of the object given a torque.
    ///
    /// This method calculates the angular acceleration using the formula:
    /// `ɑ = I⁻¹ * T`, where `I` is the inertia tensor and `T` is the torque.
    ///
    /// # Arguments
    /// * `torque` - A reference to the [`Torque<S>`] applied to the object.
    ///
    /// # Returns
    /// An [`AngularAcceleration<S>`] representing the object's angular acceleration.
    #[inline]
    #[must_use]
    pub fn get_angular_acceleration(&self, torque: &Torque<S>) -> AngularAcceleration<S> {
        AngularAcceleration::new(self.tensor.inverse().mul_vec3(torque.0))
    }
}
impl<S: CoordinateSystem>CoordinateConvert for Inertia<S> {
    type Global = Inertia<Global>;

    type Local = Inertia<Local>;

    /// Converts the inertia to the global coordinate system using a rotation quaternion.
    ///
    /// # Arguments
    /// * `rot` - A `Quat` representing the rotation from the local coordinate system to the global coordinate system.
    ///
    /// # Returns
    /// A new [`Inertia<Global>`] object with the same mass but a transformed inertia tensor.
    #[inline]
    fn to_global(self, rot: Quat) -> Inertia<Global> {
        Inertia{
            mass: self.mass,
            tensor: S::mat3_to_global(self.tensor, rot),
            state: PhantomData,
        }
    }

    /// Converts the inertia to the local coordinate system using a rotation quaternion.
    ///
    /// # Arguments
    /// * `rot` - A `Quat` representing the rotation from the global coordinate system to the local coordinate system.
    ///
    /// # Returns
    /// A new [`Inertia<Local>`] object with the same mass but a transformed inertia tensor.
    #[inline]
    fn to_local(self, rot: Quat) -> Inertia<Local> {
        Inertia {
            mass: self.mass,
            tensor: S::mat3_to_local(self.tensor, rot),
            state: PhantomData,
        }
    }
}

/// Constructor methods for [`Inertia`] when the coordinate system is [`Local`].
impl Inertia<Local> {
    /// Creates an inertia tensor for a uniform cylinder with its height along the x-axis.
    ///
    /// # Arguments
    /// * `height` - The height of the cylinder.
    /// * `radius` - The radius of the cylinder's cross-section.
    /// * `mass` - The mass of the cylinder.
    ///
    /// # Returns
    /// An [`Inertia<Local>`] object representing the cylinder.
    #[inline]
    #[must_use]
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
            state: PhantomData,
        }
    }

    /// Creates an inertia tensor for a uniform cylinder with its height along the y-axis.
    ///
    /// # Arguments
    /// * `height` - The height of the cylinder.
    /// * `radius` - The radius of the cylinder's cross-section.
    /// * `mass` - The mass of the cylinder.
    ///
    /// # Returns
    /// An [`Inertia<Local>`] object representing the cylinder.
    #[inline]
    #[must_use]
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
            state: PhantomData,
        }
    }

    /// Creates an inertia tensor for a uniform cylinder with its height along the z-axis.
    ///
    /// # Arguments
    /// * `height` - The height of the cylinder.
    /// * `radius` - The radius of the cylinder's cross-section.
    /// * `mass` - The mass of the cylinder.
    ///
    /// # Returns
    /// An [`Inertia<Local>`] object representing the cylinder.
    #[inline]
    #[must_use]
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
            state: PhantomData,
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
