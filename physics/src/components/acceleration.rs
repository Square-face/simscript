//! # Acceleration Components
//!
//! This file defines components for representing linear and angular acceleration in various coordinate systems.
//! The components are designed to work with the [`Global`] and [`Local`] typestates to enforce compile-time correctness
//! and facilitate transformations between coordinate spaces.
//!
//! ## Examples
//! ```rust
//! # use physics::components::acceleration::Acceleration;
//! # use physics::coordinate_systems::{Global, Local};
//! # use glam::{Quat, Vec3};
//! # use std::f32::consts::PI;
//!
//! // Define an acceleration in global space
//! let gravity = Acceleration::<Global>::GRAVITY;
//! assert_eq!(gravity.0, Vec3::new(0.0, -9.82, 0.0));
//!
//! // Convert the acceleration to local space
//! let rotation = Quat::from_rotation_z(PI/4.); // Example rotation (Pitched 45° up).
//! let local_gravity = gravity.to_local(rotation);
//! println!("Local gravity: {:?}", local_gravity.0);
//! ```
//!
extern crate overload;
use glam::{Quat, Vec3};
use overload::overload;
use std::{marker::PhantomData, ops};

use crate::{
    components::velocity::{AngularVelocity, Velocity},
    coordinate_systems::{CoordinateConvert, CoordinateSystem, Global, Local},
};

/// Represents linear acceleration in a specific coordinate system.
///
/// The generic parameter `S` indicates the coordinate system, such as [`Global`] or [`Local`].
/// Acceleration values are stored as a [`Vec3`] with units of `m/s²`.
///
/// # Examples
/// ```rust
/// # use physics::components::acceleration::Acceleration;
/// # use physics::coordinate_systems::{Global, Local};
/// # use glam::{Quat, Vec3};
/// # use std::f32::consts::PI;
///
/// // Define an acceleration in global space
/// let gravity = Acceleration::<Global>::GRAVITY;
/// assert_eq!(gravity.0, Vec3::new(0.0, -9.82, 0.0));
///
/// // Convert the acceleration to local space
/// let rotation = Quat::from_rotation_z(PI/4.); // Example rotation (Pitched 45° up).
/// let local_gravity = gravity.to_local(rotation);
/// println!("Local gravity: {:?}", local_gravity.0);
/// ```
#[derive(Debug, PartialEq)]
pub struct Acceleration<S: CoordinateSystem>(pub Vec3, PhantomData<S>);

/// Represents angular acceleration in a specific coordinate system.
///
/// The generic parameter `S` indicates the coordinate system, such as [`Global`] or [`Local`].
/// Angular acceleration values are stored as a [`Vec3`] with units of `rad/s^2`.
///
/// # Examples
/// ```rust
/// # use physics::components::acceleration::AngularAcceleration;
/// # use physics::coordinate_systems::Local;
/// # use glam::{Quat, Vec3};
///
/// let angular_acceleration = AngularAcceleration::<Local>::new(Vec3::new(0.1, 0.2, 0.3));
/// assert_eq!(angular_acceleration.0, Vec3::new(0.1, 0.2, 0.3));
///
/// let rotation = Quat::IDENTITY; // Example rotation.
/// let global_angular_acc = angular_acceleration.to_global(rotation);
/// println!("Global Angular Acceleration: {:?}", global_angular_acc.0);
/// ```
#[derive(Debug, PartialEq)]
pub struct AngularAcceleration<S: CoordinateSystem>(pub Vec3, PhantomData<S>);

impl<S: CoordinateSystem> Acceleration<S> {
    /// [Acceleration] that doesn't accelerate in any direction
    pub const ZERO: Self = Self::new(Vec3::ZERO);

    /// [Acceleration] that mimics gravity (-9.82 m/s² in y velocity)
    pub const GRAVITY: Self = Self::new(Vec3::new(0.0, -9.82, 0.0));

    #[inline]
    #[must_use]
    pub const fn new(acc: Vec3) -> Acceleration<S> {
        Acceleration(acc, PhantomData)
    }

    pub fn to_local(&self, rot: Quat) -> Acceleration<Local> {
        Acceleration::new(S::vec3_to_local(self.0, rot))
    }

    pub fn to_global(&self, rot: Quat) -> Acceleration<Global> {
        Acceleration::new(S::vec3_to_global(self.0, rot))
    }
}

impl<S: CoordinateSystem> AngularAcceleration<S> {
    pub const ZERO: Self = Self::new(Vec3::ZERO);

    #[inline]
    #[must_use]
    pub const fn new(acc: Vec3) -> AngularAcceleration<S> {
        AngularAcceleration(acc, PhantomData)
    }

    pub fn to_local(&self, rot: Quat) -> AngularAcceleration<Local> {
        AngularAcceleration::new(S::vec3_to_local(self.0, rot))
    }

    pub fn to_global(&self, rot: Quat) -> AngularAcceleration<Global> {
        AngularAcceleration::new(S::vec3_to_global(self.0, rot))
    }
}

impl<S: CoordinateSystem> CoordinateConvert for Acceleration<S> {
    type Global = Acceleration<Global>;

    type Local = Acceleration<Local>;

    #[inline]
    fn to_global(self, rot: Quat) -> Self::Global {
        Acceleration::<Global>::new(S::vec3_to_global(self.0, rot))
    }

    #[inline]
    fn to_local(self, rot: Quat) -> Self::Local {
        Acceleration::<Local>::new(S::vec3_to_local(self.0, rot))
    }
}

impl<S: CoordinateSystem> CoordinateConvert for AngularAcceleration<S> {
    type Global = AngularAcceleration<Global>;

    type Local = AngularAcceleration<Local>;

    #[inline]
    fn to_global(self, rot: Quat) -> Self::Global {
        AngularAcceleration::<Global>::new(S::vec3_to_global(self.0, rot))
    }

    #[inline]
    fn to_local(self, rot: Quat) -> Self::Local {
        AngularAcceleration::<Local>::new(S::vec3_to_global(self.0, rot))
    }
}

// Other
impl<S: CoordinateSystem + PartialOrd> PartialOrd for Acceleration<S> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let this = self.0.element_sum();
        let other = other.0.element_sum();

        this.partial_cmp(&other)
    }
}

// ==== Local Coordinate System ====
// impl x for Acceleration<Local>
overload!((a: ?Acceleration<Local>) + (b: ?Acceleration<Local>) -> Acceleration<Local> { Acceleration::new(a.0 + b.0) });
overload!((a: ?Acceleration<Local>) - (b: ?Acceleration<Local>) -> Acceleration<Local> { Acceleration::new(a.0 - b.0) });
overload!((a: ?Acceleration<Local>) * (b: ?Acceleration<Local>) -> Acceleration<Local> { Acceleration::new(a.0 * b.0) });
overload!((a: ?Acceleration<Local>) / (b: ?Acceleration<Local>) -> Acceleration<Local> { Acceleration::new(a.0 / b.0) });

overload!((a: ?Acceleration<Local>) * (b: f32) -> Velocity<Local> { Velocity::new(a.0 * b) });
overload!((a: ?Acceleration<Local>) / (b: f32) -> Acceleration<Local> { Acceleration::new(a.0 / b) });

overload!(- (a: &mut Acceleration<Local>) -> Acceleration<Local> { Acceleration::new(- a.0) });

// impl xAssign for Acceleration<Local>
overload!((a: &mut Acceleration<Local>) += (b: ?Acceleration<Local>) { a.0 += b.0 });
overload!((a: &mut Acceleration<Local>) -= (b: ?Acceleration<Local>) { a.0 -= b.0 });
overload!((a: &mut Acceleration<Local>) *= (b: ?Acceleration<Local>) { a.0 *= b.0 });
overload!((a: &mut Acceleration<Local>) /= (b: ?Acceleration<Local>) { a.0 /= b.0 });

overload!((a: &mut Acceleration<Local>) *= (b: f32) {a.0 *= b});
overload!((a: &mut Acceleration<Local>) /= (b: f32) {a.0 /= b});

// ==== Global Coordinate System ====
// impl x for Acceleration<Global>
overload!((a: ?Acceleration<Global>) + (b: ?Acceleration<Global>) -> Acceleration<Global> { Acceleration::new(a.0 + b.0) });
overload!((a: ?Acceleration<Global>) - (b: ?Acceleration<Global>) -> Acceleration<Global> { Acceleration::new(a.0 - b.0) });
overload!((a: ?Acceleration<Global>) * (b: ?Acceleration<Global>) -> Acceleration<Global> { Acceleration::new(a.0 * b.0) });
overload!((a: ?Acceleration<Global>) / (b: ?Acceleration<Global>) -> Acceleration<Global> { Acceleration::new(a.0 / b.0) });

overload!((a: ?Acceleration<Global>) * (b: f32) -> Velocity<Global> { Velocity::new(a.0 * b) });
overload!((a: ?Acceleration<Global>) / (b: f32) -> Acceleration<Global> { Acceleration::new(a.0 / b) });

overload!(- (a: &mut Acceleration<Global>) -> Acceleration<Global> { Acceleration::new(- a.0) });

// impl xAssign for Acceleration<Global>
overload!((a: &mut Acceleration<Global>) += (b: ?Acceleration<Global>) { a.0 += b.0 });
overload!((a: &mut Acceleration<Global>) -= (b: ?Acceleration<Global>) { a.0 -= b.0 });
overload!((a: &mut Acceleration<Global>) *= (b: ?Acceleration<Global>) { a.0 *= b.0 });
overload!((a: &mut Acceleration<Global>) /= (b: ?Acceleration<Global>) { a.0 /= b.0 });

overload!((a: &mut Acceleration<Global>) *= (b: f32) {a.0 *= b});
overload!((a: &mut Acceleration<Global>) /= (b: f32) {a.0 /= b});

// ==== Local Coordinate System ====
// impl x for AngularAcceleration<Local>
overload!((a: ?AngularAcceleration<Local>) + (b: ?AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new(a.0 + b.0) });
overload!((a: ?AngularAcceleration<Local>) - (b: ?AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new(a.0 - b.0) });
overload!((a: ?AngularAcceleration<Local>) * (b: ?AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new(a.0 * b.0) });
overload!((a: ?AngularAcceleration<Local>) / (b: ?AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new(a.0 / b.0) });

overload!((a: ?AngularAcceleration<Local>) * (b: f32) -> AngularVelocity<Local> { AngularVelocity::new(a.0 * b) });
overload!((a: ?AngularAcceleration<Local>) / (b: f32) -> AngularAcceleration<Local> { AngularAcceleration::new(a.0 / b) });

overload!(- (a: &mut AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new(- a.0) });

// impl xAssign for AngularAcceleration<Local>
overload!((a: &mut AngularAcceleration<Local>) += (b: ?AngularAcceleration<Local>) { a.0 += b.0 });
overload!((a: &mut AngularAcceleration<Local>) -= (b: ?AngularAcceleration<Local>) { a.0 -= b.0 });
overload!((a: &mut AngularAcceleration<Local>) *= (b: ?AngularAcceleration<Local>) { a.0 *= b.0 });
overload!((a: &mut AngularAcceleration<Local>) /= (b: ?AngularAcceleration<Local>) { a.0 /= b.0 });

overload!((a: &mut AngularAcceleration<Local>) *= (b: f32) {a.0 *= b});
overload!((a: &mut AngularAcceleration<Local>) /= (b: f32) {a.0 /= b});

// ==== Global Coordinate System ====
// impl x for AngularAcceleration<Global>
overload!((a: ?AngularAcceleration<Global>) + (b: ?AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new(a.0 + b.0) });
overload!((a: ?AngularAcceleration<Global>) - (b: ?AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new(a.0 - b.0) });
overload!((a: ?AngularAcceleration<Global>) * (b: ?AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new(a.0 * b.0) });
overload!((a: ?AngularAcceleration<Global>) / (b: ?AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new(a.0 / b.0) });

overload!((a: ?AngularAcceleration<Global>) * (b: f32) -> AngularVelocity<Global> { AngularVelocity::new(a.0 * b) });
overload!((a: ?AngularAcceleration<Global>) / (b: f32) -> AngularAcceleration<Global> { AngularAcceleration::new(a.0 / b) });

overload!(- (a: &mut AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new(- a.0) });

// impl xAssign for AngularAcceleration<Global>
overload!((a: &mut AngularAcceleration<Global>) += (b: ?AngularAcceleration<Global>) { a.0 += b.0 });
overload!((a: &mut AngularAcceleration<Global>) -= (b: ?AngularAcceleration<Global>) { a.0 -= b.0 });
overload!((a: &mut AngularAcceleration<Global>) *= (b: ?AngularAcceleration<Global>) { a.0 *= b.0 });
overload!((a: &mut AngularAcceleration<Global>) /= (b: ?AngularAcceleration<Global>) { a.0 /= b.0 });

overload!((a: &mut AngularAcceleration<Global>) *= (b: f32) {a.0 *= b});
overload!((a: &mut AngularAcceleration<Global>) /= (b: f32) {a.0 /= b});
