//! # Velocity and AngularVelocity Components
//!
//! This module provides components to represent and manipulate the translational and angular velocities of entities in a Bevy ECS-based application. 
//! The velocities are parameterized by a coordinate system, allowing flexibility between global and local contexts.
//!
//! ## Components
//!
//! - [`Velocity<S>`]: Represents translational velocity as a [`Vec3`] in a specified coordinate system. (unit: m/s)
//! - [`AngularVelocity<S>`]: Represents angular velocity as a [`Vec3`] in a specified coordinate system. (unit: rad/s)
//!
//! ## Usage
//!
//! These components can be added to Bevy entities to track and manipulate their velocities.
//! The coordinate system is specified using the generic type `S`, which could be [`Global`] or [`Local`].
//!
//! ## Examples
//!
//! ```rust
//! # use bevy::prelude::*;
//! # use physics::coordinate_systems::{Global, Local};
//! # use physics::components::velocity::{Velocity, AngularVelocity};
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn((
//!         Velocity::<Global>::new(Vec3::new(1.0, 0.0, 0.0)), // 1 m/sec in the x-direction
//!         AngularVelocity::<Local>::new(Vec3::new(0.0, 0.1, 0.0)), // 0.1 rad/sec rotation about the y-axis
//!     ));
//! }
//! ```
//!
//! ### Using Zero Velocity
//!
//! ```rust
//! # use physics::coordinate_systems::{Global, Local};
//! # use physics::components::velocity::{Velocity, AngularVelocity};
//! let zero_velocity = Velocity::<Global>::ZERO;
//! let zero_angular_velocity = AngularVelocity::<Local>::ZERO;
//! ```
extern crate overload;
use overload::overload;
use std::{marker::PhantomData, ops};

use bevy::{
    ecs::component::Component,
    math::{Quat, Vec3},
};

use crate::coordinate_systems::{CoordinateConvert, CoordinateSystem, Global, Local};

/// Stores the current translational velocity of an entity.
///
/// The velocity is represented as a [`Vec3`] in a given coordinate system. 
/// The coordinate system type `S` is defined using the [`CoordinateSystem`] trait, which could be [`Global`] or [`Local`].
/// 
/// # Generics
/// - `S`: The coordinate system type, implementing the [`CoordinateSystem`] trait.
#[derive(Component, Debug)]
pub struct Velocity<S: CoordinateSystem>(pub Vec3, PhantomData<S>);

/// Stores the current angular velocity of an entity.
///
/// The angular velocity is represented as a [`Vec3`] in a given coordinate system.
/// The coordinate system type `S` is defined using the [`CoordinateSystem`] trait.
/// 
/// # Generics
/// - `S`: The coordinate system type, implementing the [`CoordinateSystem`] trait.
#[derive(Component, Debug)]
pub struct AngularVelocity<S: CoordinateSystem>(pub Vec3, PhantomData<S>);

impl<S: CoordinateSystem> Velocity<S> {
    /// Represents a velocity of zero in all directions.
    pub const ZERO: Self = Self::new(Vec3::ZERO);

    #[inline]
    #[must_use]
    /// Creates a new [`Velocity`] instance with the given [`Vec3`].
    ///
    /// # Arguments
    /// - `vel`: A [`Vec3`] representing the translational velocity.
    ///
    /// # Returns
    /// A `Velocity` instance containing the given velocity vector.
    pub const fn new(vel: Vec3) -> Velocity<S> {
        Velocity(vel, PhantomData)
    }
}

impl<S: CoordinateSystem> AngularVelocity<S> {
    /// Represents an angular velocity of zero in all directions.
    pub const ZERO: Self = Self::new(Vec3::ZERO);

    #[inline]
    #[must_use]
    /// Creates a new `AngularVelocity` instance with the given [`Vec3`].
    ///
    /// # Arguments
    /// - `vel`: A [`Vec3`] representing the angular velocity.
    ///
    /// # Returns
    /// An `AngularVelocity` instance containing the given angular velocity vector.
    pub const fn new(vel: Vec3) -> AngularVelocity<S> {
        AngularVelocity(vel, PhantomData)
    }
}

impl<S: CoordinateSystem> Velocity<S> {
    /// Converts the velocity into a directional quaternion.
    ///
    /// This method interprets the velocity as a direction in global coordinates
    /// and computes a quaternion representing its orientation based on yaw and pitch angles.
    ///
    /// # Returns
    /// A [`Quat`] representing the directional orientation of the velocity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use physics::coordinate_systems::Global;
    /// # use physics::components::velocity::Velocity;
    /// # use bevy::math::{Vec3, Quat};
    ///
    /// let velocity = Velocity::<Global>::new(Vec3::new(1.0, 0.0, 0.0));
    /// let direction = velocity.to_direction();
    /// assert_eq!(direction, Quat::IDENTITY);
    /// ```
    #[must_use]
    pub fn to_direction(&self) -> Quat {
        Quat::from_euler(bevy::math::EulerRot::YXZ, self.yaw(), 0.0, self.pitch())
    }

    /// Calculates the pitch (vertical angle) of the velocity vector.
    ///
    /// The pitch is calculated based on the ratio of the vertical velocity component (`y`) 
    /// to the horizontal distance (`sqrt(x² + z²)`).
    ///
    /// # Returns
    /// The pitch angle in radians.
    fn pitch(&self) -> f32 {
        let vec = self.0;
        let fdist = (vec.x.powi(2) + vec.z.powi(2)).sqrt();
        (vec.y / fdist).atan()
    }

    /// Calculates the yaw (horizontal angle) of the velocity vector.
    ///
    /// The yaw is calculated using the `atan2` function, which determines the angle between 
    /// the vector's projection on the `x-z` plane and the positive x-axis.
    ///
    /// # Returns
    /// The yaw angle in radians.
    #[must_use]
    fn yaw(&self) -> f32 {
        let vec = self.0;
        -vec.z.atan2(vec.x)
    }
}

impl<S: CoordinateSystem> CoordinateConvert for Velocity<S> {
    type Global = Velocity<Global>;

    type Local = Velocity<Local>;

    #[inline]
    fn to_global(self, rot: Quat) -> Self::Global {
        Velocity::<Global>::new(S::vec3_to_global(self.0, rot))
    }

    #[inline]
    fn to_local(self, rot: Quat) -> Self::Local {
        Velocity::<Local>::new(S::vec3_to_local(self.0, rot))
    }
}

impl<S: CoordinateSystem> CoordinateConvert for AngularVelocity<S> {
    type Global = AngularVelocity<Global>;

    type Local = AngularVelocity<Local>;

    #[inline]
    fn to_global(self, rot: Quat) -> Self::Global {
        AngularVelocity::<Global>::new(S::vec3_to_global(self.0, rot))
    }

    #[inline]
    fn to_local(self, rot: Quat) -> Self::Local {
        AngularVelocity::<Local>::new(S::vec3_to_local(self.0, rot))
    }
}

// ==== Local Coordinate System ====
// impl x for Velocity<Local>
overload!((a: ?Velocity<Local>) + (b: ?Velocity<Local>) -> Velocity<Local> { Velocity::new(a.0 + b.0) });
overload!((a: ?Velocity<Local>) - (b: ?Velocity<Local>) -> Velocity<Local> { Velocity::new(a.0 - b.0) });
overload!((a: ?Velocity<Local>) * (b: ?Velocity<Local>) -> Velocity<Local> { Velocity::new(a.0 * b.0) });
overload!((a: ?Velocity<Local>) / (b: ?Velocity<Local>) -> Velocity<Local> { Velocity::new(a.0 / b.0) });

overload!((a: ?Velocity<Local>) * (b: f32) -> Velocity<Local> { Velocity::new(a.0 * b) });
overload!((a: ?Velocity<Local>) / (b: f32) -> Velocity<Local> { Velocity::new(a.0 / b) });

overload!(- (a: &mut Velocity<Local>) -> Velocity<Local> { Velocity::new(- a.0) });

// impl xAssign for Velocity<Local>
overload!((a: &mut Velocity<Local>) += (b: ?Velocity<Local>) { a.0 += b.0 });
overload!((a: &mut Velocity<Local>) -= (b: ?Velocity<Local>) { a.0 -= b.0 });
overload!((a: &mut Velocity<Local>) *= (b: ?Velocity<Local>) { a.0 *= b.0 });
overload!((a: &mut Velocity<Local>) /= (b: ?Velocity<Local>) { a.0 /= b.0 });

overload!((a: &mut Velocity<Local>) *= (b: f32) {a.0 *= b});
overload!((a: &mut Velocity<Local>) /= (b: f32) {a.0 /= b});

// ==== Global Coordinate System ====
// impl x for Velocity<Global>
overload!((a: ?Velocity<Global>) + (b: ?Velocity<Global>) -> Velocity<Global> { Velocity::new(a.0 + b.0) });
overload!((a: ?Velocity<Global>) - (b: ?Velocity<Global>) -> Velocity<Global> { Velocity::new(a.0 - b.0) });
overload!((a: ?Velocity<Global>) * (b: ?Velocity<Global>) -> Velocity<Global> { Velocity::new(a.0 * b.0) });
overload!((a: ?Velocity<Global>) / (b: ?Velocity<Global>) -> Velocity<Global> { Velocity::new(a.0 / b.0) });

overload!((a: ?Velocity<Global>) * (b: f32) -> Velocity<Global> { Velocity::new(a.0 * b) });
overload!((a: ?Velocity<Global>) / (b: f32) -> Velocity<Global> { Velocity::new(a.0 / b) });

overload!(- (a: &mut Velocity<Global>) -> Velocity<Global> { Velocity::new(- a.0) });

// impl xAssign for Velocity<Global>
overload!((a: &mut Velocity<Global>) += (b: ?Velocity<Global>) { a.0 += b.0 });
overload!((a: &mut Velocity<Global>) -= (b: ?Velocity<Global>) { a.0 -= b.0 });
overload!((a: &mut Velocity<Global>) *= (b: ?Velocity<Global>) { a.0 *= b.0 });
overload!((a: &mut Velocity<Global>) /= (b: ?Velocity<Global>) { a.0 /= b.0 });

overload!((a: &mut Velocity<Global>) *= (b: f32) {a.0 *= b});
overload!((a: &mut Velocity<Global>) /= (b: f32) {a.0 /= b});

// ==== Local Coordinate System ====
// impl x for AngularVelocity<Local>
overload!((a: ?AngularVelocity<Local>) + (b: ?AngularVelocity<Local>) -> AngularVelocity<Local> { AngularVelocity::new(a.0 + b.0) });
overload!((a: ?AngularVelocity<Local>) - (b: ?AngularVelocity<Local>) -> AngularVelocity<Local> { AngularVelocity::new(a.0 - b.0) });
overload!((a: ?AngularVelocity<Local>) * (b: ?AngularVelocity<Local>) -> AngularVelocity<Local> { AngularVelocity::new(a.0 * b.0) });
overload!((a: ?AngularVelocity<Local>) / (b: ?AngularVelocity<Local>) -> AngularVelocity<Local> { AngularVelocity::new(a.0 / b.0) });

overload!((a: ?AngularVelocity<Local>) * (b: f32) -> AngularVelocity<Local> { AngularVelocity::new(a.0 * b) });
overload!((a: ?AngularVelocity<Local>) / (b: f32) -> AngularVelocity<Local> { AngularVelocity::new(a.0 / b) });

overload!(- (a: &mut AngularVelocity<Local>) -> AngularVelocity<Local> { AngularVelocity::new(- a.0) });

// impl xAssign for AngularVelocity<Local>
overload!((a: &mut AngularVelocity<Local>) += (b: ?AngularVelocity<Local>) { a.0 += b.0 });
overload!((a: &mut AngularVelocity<Local>) -= (b: ?AngularVelocity<Local>) { a.0 -= b.0 });
overload!((a: &mut AngularVelocity<Local>) *= (b: ?AngularVelocity<Local>) { a.0 *= b.0 });
overload!((a: &mut AngularVelocity<Local>) /= (b: ?AngularVelocity<Local>) { a.0 /= b.0 });

overload!((a: &mut AngularVelocity<Local>) *= (b: f32) {a.0 *= b});
overload!((a: &mut AngularVelocity<Local>) /= (b: f32) {a.0 /= b});

// ==== Global Coordinate System ====
// impl x for AngularVelocity<Global>
overload!((a: ?AngularVelocity<Global>) + (b: ?AngularVelocity<Global>) -> AngularVelocity<Global> { AngularVelocity::new(a.0 + b.0) });
overload!((a: ?AngularVelocity<Global>) - (b: ?AngularVelocity<Global>) -> AngularVelocity<Global> { AngularVelocity::new(a.0 - b.0) });
overload!((a: ?AngularVelocity<Global>) * (b: ?AngularVelocity<Global>) -> AngularVelocity<Global> { AngularVelocity::new(a.0 * b.0) });
overload!((a: ?AngularVelocity<Global>) / (b: ?AngularVelocity<Global>) -> AngularVelocity<Global> { AngularVelocity::new(a.0 / b.0) });

overload!((a: ?AngularVelocity<Global>) * (b: f32) -> AngularVelocity<Global> { AngularVelocity::new(a.0 * b) });
overload!((a: ?AngularVelocity<Global>) / (b: f32) -> AngularVelocity<Global> { AngularVelocity::new(a.0 / b) });

overload!(- (a: &mut AngularVelocity<Global>) -> AngularVelocity<Global> { AngularVelocity::new(- a.0) });

// impl xAssign for AngularVelocity<Global>
overload!((a: &mut AngularVelocity<Global>) += (b: ?AngularVelocity<Global>) { a.0 += b.0 });
overload!((a: &mut AngularVelocity<Global>) -= (b: ?AngularVelocity<Global>) { a.0 -= b.0 });
overload!((a: &mut AngularVelocity<Global>) *= (b: ?AngularVelocity<Global>) { a.0 *= b.0 });
overload!((a: &mut AngularVelocity<Global>) /= (b: ?AngularVelocity<Global>) { a.0 /= b.0 });

overload!((a: &mut AngularVelocity<Global>) *= (b: f32) {a.0 *= b});
overload!((a: &mut AngularVelocity<Global>) /= (b: f32) {a.0 /= b});

#[cfg(test)]
mod linear_velocity {
    use std::f32::consts::PI;

    use bevy::math::{Quat, Vec3};
    use float_cmp::assert_approx_eq;

    use crate::{components::Velocity, coordinate_systems::Global};

    #[test]
    fn to_direction() {
        let x = Velocity::<Global>::new(Vec3::X).to_direction().to_array();
        let y = Velocity::<Global>::new(Vec3::Y).to_direction().to_array();
        let z = Velocity::<Global>::new(Vec3::Z).to_direction().to_array();

        let ang45 = Velocity::<Global>::new(Vec3 {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        })
        .to_direction()
        .to_array();

        assert_approx_eq!(&[f32], &x, &Quat::default().to_array());
        assert_approx_eq!(&[f32], &y, &Quat::from_rotation_z(PI / 2.0).to_array());
        assert_approx_eq!(&[f32], &z, &Quat::from_rotation_y(-PI / 2.0).to_array());
        assert_approx_eq!(&[f32], &ang45, &Quat::from_rotation_z(PI / 4.0).to_array());
    }

    #[test]
    fn pitch() {
        let x = Velocity::<Global>::new(Vec3::X);
        let y = Velocity::<Global>::new(Vec3::Y);
        let z = Velocity::<Global>::new(Vec3::Z);

        assert_approx_eq!(f32, x.pitch(), 0.0);
        assert_approx_eq!(f32, y.pitch(), PI / 2.0);
        assert_approx_eq!(f32, z.pitch(), 0.0);
    }

    #[test]
    fn yaw() {
        let x = Velocity::<Global>::new(Vec3::X);
        let y = Velocity::<Global>::new(Vec3::Y);
        let z = Velocity::<Global>::new(Vec3::Z);

        let nx = Velocity::<Global>::new(Vec3::NEG_X);
        let ny = Velocity::<Global>::new(Vec3::NEG_Y);
        let nz = Velocity::<Global>::new(Vec3::NEG_Z);

        assert_approx_eq!(f32, x.yaw(), 0.0);
        assert_approx_eq!(f32, y.yaw(), 0.0);
        assert_approx_eq!(f32, z.yaw(), -PI / 2.0);

        assert_approx_eq!(f32, nx.yaw(), -PI);
        assert_approx_eq!(f32, ny.yaw(), 0.0);
        assert_approx_eq!(f32, nz.yaw(), PI / 2.0);
    }
}
