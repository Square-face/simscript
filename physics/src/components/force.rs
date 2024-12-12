//! # Force, Torque, and Moment Components
//!
//! This module defines components for representing forces and torques in a Bevy ECS-based application.
//! It includes:
//! - `Moment`: Represents forces applied off-center, causing both translational and rotational effects.
//! - `Force`: Represents forces applied at the center of mass, causing pure translation.
//! - `Torque`: Represents rotational forces.
//!
//! ## Units
//! - Forces (`Force` and `Moment`) are represented as vectors in newtons (N).
//! - Torques are represented in newton-meters (Nm).
//!
//! ## Coordinate Systems
//! Each component is parameterized by a generic type `S` representing the coordinate system, which can be either `Global` or `Local`.
//!
//! ## Usage
//! These components can be added to Bevy entities to simulate realistic physical effects.
//!
//! ## Examples
//!
//! ### Spawning an Entity with a Force and Torque
//!
//! ```rust
//! use bevy::math::Vec3;
//! use physics::coordinate_systems::Global;
//! use physics::components::force::Moment;
//!
//! let t = Moment::<Global>::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(0.0, 5.0, 0.0));
//! let (torque, force) = t.get_parts();
//! assert_eq!(force.0, Vec3::new(0.0, 5.0, 0.0));
//! assert_eq!(torque.0, Vec3::new(0.0, 0.0, 50.0));
//! ```
extern crate overload;
use bevy::math::{Quat, Vec3};
use overload::overload;
use std::{marker::PhantomData, ops};

use crate::coordinate_systems::{CoordinateConvert, CoordinateSystem, Global, Local};

/// Represents a force that is not applied at the center of mass, causing both translation and rotation.
///
/// **Units:** Force in newtons (N), offset in meters (m).
///
/// # Generics
/// - `CoordinateSystem`: Specifies the coordinate system (e.g., `Global` or `Local`).
///
/// # Examples
///
/// ### Creating a Moment
/// ```rust
/// use bevy::math::Vec3;
/// use physics::coordinate_systems::Global;
/// use physics::components::force::Moment;
///
/// let moment = Moment::<Global>::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 10.0, 0.0));
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Moment<CordinateSystem: CoordinateSystem> {
    /// Offset the applied force from the origin
    offset: Vec3,

    /// The force being applied
    force: Vec3,

    coordinate_system: PhantomData<CordinateSystem>,
}

/// Represents a force applied at the center of mass.
///
/// **Units:** Newtons (N).
///
/// # Examples
/// ```rust
/// use bevy::math::Vec3;
/// use physics::coordinate_systems::Global;
/// use physics::components::force::Force;
///
/// let force = Force::<Global>::new(Vec3::new(5.0, 0.0, 0.0));
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Force<CordinateSystem: CoordinateSystem>(pub Vec3, PhantomData<CordinateSystem>);

/// Represents a torque applied to an object.
///
/// **Units:** Newton-meters (Nm).
///
/// # Examples
/// ```rust
/// use bevy::math::Vec3;
/// use physics::coordinate_systems::Global;
/// use physics::components::force::Torque;
///
/// let torque = Torque::<Global>::new(Vec3::new(0.0, 3.0, 0.0));
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Torque<CordinateSystem: CoordinateSystem>(pub Vec3, PhantomData<CordinateSystem>);

impl<S: CoordinateSystem> Moment<S> {
    pub const ZERO: Self = Self::new(Vec3::ZERO, Vec3::ZERO);

    #[inline]
    #[must_use]
    /// Create a new [Moment] from an offset and a force
    pub const fn new(offset: Vec3, force: Vec3) -> Self {
        Self {
            offset,
            force,
            coordinate_system: PhantomData,
        }
    }

    /// Extracts the translational component of the `Moment` as a `Force`.
    ///
    /// # Returns
    /// A `Force` instance representing the translational component.
    ///
    /// # Examples
    /// ```rust
    /// use bevy::math::Vec3;
    /// use physics::coordinate_systems::Global;
    /// use physics::components::force::{Moment, Force};
    ///
    /// let moment = Moment::<Global>::new(Vec3::ZERO, Vec3::X);
    /// assert_eq!(moment.get_force(), Force::new(Vec3::X));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_force(&self) -> Force<S> {
        Force::new(self.force)
    }

    /// Extracts the rotational component of the `Moment` as a `Torque`.
    ///
    /// # Returns
    /// A `Torque` instance representing the rotational component.
    ///
    /// # Examples
    /// ```rust
    /// use bevy::math::Vec3;
    /// use physics::coordinate_systems::Global;
    /// use physics::components::force::{Moment, Torque};
    ///
    /// let moment = Moment::<Global>::new(Vec3::X, Vec3::Y);
    /// assert_eq!(moment.get_torque(), Torque::new(Vec3::Z));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_torque(&self) -> Torque<S> {
        match self.offset.try_normalize() {
            None => Torque::new(Vec3::ZERO),
            Some(offset) => {
                let radial = self.force.project_onto_normalized(offset);
                let torque = self.offset.cross(self.force - radial);

                Torque::new(torque)
            }
        }
    }

    /// Extracts both the torque and force components as a tuple.
    ///
    /// # Returns
    /// A tuple `(Torque, Force)`.
    ///
    /// # Examples
    /// ```rust
    /// use bevy::math::Vec3;
    /// use physics::coordinate_systems::Global;
    /// use physics::components::force::Moment;
    ///
    /// let moment = Moment::<Global>::new(Vec3::Z, Vec3::ONE);
    /// let (torque, force) = moment.get_parts();
    ///
    /// assert_eq!(torque.0, Vec3::new(-1.0, 1.0, 0.0));
    /// assert_eq!(force.0, Vec3::ONE);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_parts(&self) -> (Torque<S>, Force<S>) {
        (self.get_torque(), self.get_force())
    }
}

impl<S: CoordinateSystem> Force<S> {
    #[inline]
    #[must_use]
    /// Create a new force
    pub const fn new(force: Vec3) -> Self {
        Force(force, PhantomData)
    }
}

impl<S: CoordinateSystem> Torque<S> {
    #[inline]
    #[must_use]
    /// Create a new torque
    pub const fn new(torque: Vec3) -> Self {
        Torque(torque, PhantomData)
    }
}

impl<S: CoordinateSystem> CoordinateConvert for Force<S> {
    type Global = Force<Global>;
    type Local = Force<Local>;

    #[inline]
    fn to_global(self, rot: Quat) -> Self::Global {
        Force(S::vec3_to_global(self.0, rot), PhantomData)
    }

    #[inline]
    fn to_local(self, rot: Quat) -> Self::Local {
        Force(S::vec3_to_local(self.0, rot), PhantomData)
    }
}

impl<S: CoordinateSystem> CoordinateConvert for Torque<S> {
    type Global = Torque<Global>;
    type Local = Torque<Local>;

    #[inline]
    fn to_global(self, rot: Quat) -> Self::Global {
        Torque(S::vec3_to_global(self.0, rot), PhantomData)
    }

    #[inline]
    fn to_local(self, rot: Quat) -> Self::Local {
        Torque(S::vec3_to_local(self.0, rot), PhantomData)
    }
}

impl<S: CoordinateSystem> CoordinateConvert for Moment<S> {
    type Global = Moment<Global>;
    type Local = Moment<Local>;

    #[inline]
    fn to_global(self, rot: Quat) -> Self::Global {
        Moment::new(
            S::vec3_to_global(self.offset, rot),
            S::vec3_to_global(self.force, rot),
        )
    }

    #[inline]
    fn to_local(self, rot: Quat) -> Self::Local {
        Moment::new(
            S::vec3_to_local(self.offset, rot),
            S::vec3_to_local(self.force, rot),
        )
    }
}

impl<S: CoordinateSystem> From<Moment<S>> for Force<S> {
    #[inline]
    fn from(value: Moment<S>) -> Self {
        value.get_force()
    }
}

// ==== Local Coordinate System ====
// impl x for Force<Local>
overload!((a: ?Force<Local>) + (b: ?Force<Local>) -> Force<Local> { Force::new(a.0 + b.0) });
overload!((a: ?Force<Local>) - (b: ?Force<Local>) -> Force<Local> { Force::new(a.0 - b.0) });
overload!((a: ?Force<Local>) * (b: ?Force<Local>) -> Force<Local> { Force::new(a.0 * b.0) });
overload!((a: ?Force<Local>) / (b: ?Force<Local>) -> Force<Local> { Force::new(a.0 / b.0) });

overload!((a: ?Force<Local>) * (b: f32) -> Force<Local> { Force::new(a.0 * b) });
overload!((a: ?Force<Local>) / (b: f32) -> Force<Local> { Force::new(a.0 / b) });

overload!(- (a: &mut Force<Local>) -> Force<Local> { Force::new(- a.0) });

// impl xAssign for Force<Local>
overload!((a: &mut Force<Local>) += (b: ?Force<Local>) { a.0 += b.0 });
overload!((a: &mut Force<Local>) -= (b: ?Force<Local>) { a.0 -= b.0 });
overload!((a: &mut Force<Local>) *= (b: ?Force<Local>) { a.0 *= b.0 });
overload!((a: &mut Force<Local>) /= (b: ?Force<Local>) { a.0 /= b.0 });

overload!((a: &mut Force<Local>) *= (b: f32) {a.0 *= b});
overload!((a: &mut Force<Local>) /= (b: f32) {a.0 /= b});

// ==== Global Coordinate System ====
// impl x for Force<Global>
overload!((a: ?Force<Global>) + (b: ?Force<Global>) -> Force<Global> { Force::new(a.0 + b.0) });
overload!((a: ?Force<Global>) - (b: ?Force<Global>) -> Force<Global> { Force::new(a.0 - b.0) });
overload!((a: ?Force<Global>) * (b: ?Force<Global>) -> Force<Global> { Force::new(a.0 * b.0) });
overload!((a: ?Force<Global>) / (b: ?Force<Global>) -> Force<Global> { Force::new(a.0 / b.0) });

overload!((a: ?Force<Global>) * (b: f32) -> Force<Global> { Force::new(a.0 * b) });
overload!((a: ?Force<Global>) / (b: f32) -> Force<Global> { Force::new(a.0 / b) });

overload!(- (a: &mut Force<Global>) -> Force<Global> { Force::new(- a.0) });

// impl xAssign for Force<Global>
overload!((a: &mut Force<Global>) += (b: ?Force<Global>) { a.0 += b.0 });
overload!((a: &mut Force<Global>) -= (b: ?Force<Global>) { a.0 -= b.0 });
overload!((a: &mut Force<Global>) *= (b: ?Force<Global>) { a.0 *= b.0 });
overload!((a: &mut Force<Global>) /= (b: ?Force<Global>) { a.0 /= b.0 });

overload!((a: &mut Force<Global>) *= (b: f32) {a.0 *= b});
overload!((a: &mut Force<Global>) /= (b: f32) {a.0 /= b});

impl<S: CoordinateSystem> From<Moment<S>> for Torque<S> {
    #[inline]
    fn from(value: Moment<S>) -> Self {
        value.get_torque()
    }
}

// ==== Local Coordinate System ====
// impl x for Torque<Local>
overload!((a: ?Torque<Local>) + (b: ?Torque<Local>) -> Torque<Local> { Torque::new(a.0 + b.0) });
overload!((a: ?Torque<Local>) - (b: ?Torque<Local>) -> Torque<Local> { Torque::new(a.0 - b.0) });
overload!((a: ?Torque<Local>) * (b: ?Torque<Local>) -> Torque<Local> { Torque::new(a.0 * b.0) });
overload!((a: ?Torque<Local>) / (b: ?Torque<Local>) -> Torque<Local> { Torque::new(a.0 / b.0) });

overload!((a: ?Torque<Local>) * (b: f32) -> Torque<Local> { Torque::new(a.0 * b) });
overload!((a: ?Torque<Local>) / (b: f32) -> Torque<Local> { Torque::new(a.0 / b) });

overload!(- (a: &mut Torque<Local>) -> Torque<Local> { Torque::new(- a.0) });

// impl xAssign for Torque<Local>
overload!((a: &mut Torque<Local>) += (b: ?Torque<Local>) { a.0 += b.0 });
overload!((a: &mut Torque<Local>) -= (b: ?Torque<Local>) { a.0 -= b.0 });
overload!((a: &mut Torque<Local>) *= (b: ?Torque<Local>) { a.0 *= b.0 });
overload!((a: &mut Torque<Local>) /= (b: ?Torque<Local>) { a.0 /= b.0 });

overload!((a: &mut Torque<Local>) *= (b: f32) {a.0 *= b});
overload!((a: &mut Torque<Local>) /= (b: f32) {a.0 /= b});

// ==== Global Coordinate System ====
// impl x for Torque<Global>
overload!((a: ?Torque<Global>) + (b: ?Torque<Global>) -> Torque<Global> { Torque::new(a.0 + b.0) });
overload!((a: ?Torque<Global>) - (b: ?Torque<Global>) -> Torque<Global> { Torque::new(a.0 - b.0) });
overload!((a: ?Torque<Global>) * (b: ?Torque<Global>) -> Torque<Global> { Torque::new(a.0 * b.0) });
overload!((a: ?Torque<Global>) / (b: ?Torque<Global>) -> Torque<Global> { Torque::new(a.0 / b.0) });

overload!((a: ?Torque<Global>) * (b: f32) -> Torque<Global> { Torque::new(a.0 * b) });
overload!((a: ?Torque<Global>) / (b: f32) -> Torque<Global> { Torque::new(a.0 / b) });

overload!(- (a: &mut Torque<Global>) -> Torque<Global> { Torque::new(- a.0) });

// impl xAssign for Torque<Global>
overload!((a: &mut Torque<Global>) += (b: ?Torque<Global>) { a.0 += b.0 });
overload!((a: &mut Torque<Global>) -= (b: ?Torque<Global>) { a.0 -= b.0 });
overload!((a: &mut Torque<Global>) *= (b: ?Torque<Global>) { a.0 *= b.0 });
overload!((a: &mut Torque<Global>) /= (b: ?Torque<Global>) { a.0 /= b.0 });

overload!((a: &mut Torque<Global>) *= (b: f32) {a.0 *= b});
overload!((a: &mut Torque<Global>) /= (b: f32) {a.0 /= b});

#[cfg(test)]
mod parts {
    use crate::coordinate_systems::Global;

    use super::Moment;
    use bevy::math::Vec3;

    #[test]
    fn torque() {
        let get_torque = |offset, force| Moment::<Global>::new(offset, force).get_torque().0;

        assert_eq!(get_torque(Vec3::Z, Vec3::ONE), Vec3::new(-1.0, 1.0, 0.0));

        // no offset or force, always no torque
        assert_eq!(get_torque(Vec3::Y, Vec3::ZERO), Vec3::ZERO);
        assert_eq!(get_torque(Vec3::ZERO, Vec3::Y), Vec3::ZERO);

        // radial force, always no torque
        assert_eq!(get_torque(Vec3::Y, Vec3::Y), Vec3::ZERO);
        assert_eq!(get_torque(Vec3::X, Vec3::X), Vec3::ZERO);
        assert_eq!(get_torque(Vec3::Z, Vec3::Z), Vec3::ZERO);

        // simple unit length cross product
        assert_eq!(get_torque(Vec3::Y, Vec3::Z), Vec3::X);
        assert_eq!(get_torque(Vec3::X, Vec3::Y), Vec3::Z);
        assert_eq!(get_torque(Vec3::X, Vec3::Z), Vec3::NEG_Y);

        // double force or offset, double torque
        assert_eq!(get_torque(Vec3::Y, Vec3::Z * 2.0), Vec3::X * 2.0);
        assert_eq!(get_torque(Vec3::Y * 2.0, Vec3::Z), Vec3::X * 2.0);
    }
}
