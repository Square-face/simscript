extern crate overload;
use overload::overload;
use std::{marker::PhantomData, ops};

use bevy::math::{Quat, Vec3};

use crate::cordinate_systems::{self, ConvertCordinateSystem, Global, Local};

use super::{acceleration::Acceleration, inertia::Inertia};

/// Represents a force that is not applied at the center of mass
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Moment<CordinateSystem: ConvertCordinateSystem> {
    /// Offset the applied force from the origin
    offset: Vec3,

    /// The force being applied
    force: Vec3,

    cordinate_system: PhantomData<CordinateSystem>,
}

/// Represents a force applied at the center of mass
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Force(pub Vec3);

/// Represents a torque being applied on a object
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Torque(pub Vec3);

impl Moment<Local> {
    /// [Moment] with no force in any direction
    pub const ZERO: Self = Self::new_local(Vec3::ZERO, Vec3::ZERO);

    /// Create a new [Moment] from an offset and a force
    #[inline]
    #[must_use]
    pub const fn new_local(offset: Vec3, force: Vec3) -> Self {
        Self { offset, force, cordinate_system: PhantomData }
    }

    pub fn to_global(self, rot: Quat) -> Moment<Global> {
        let offset = cordinate_systems::Local::to_global(self.offset, rot);
        let force = cordinate_systems::Local::to_global(self.force, rot);
        Moment::new_global(offset, force)
    }
}

impl Moment<Global> {
    /// Create a new [Moment] from an offset and a force
    #[inline]
    #[must_use]
    pub const fn new_global(offset: Vec3, force: Vec3) -> Self {
        Self { offset, force, cordinate_system: PhantomData }
    }

    pub fn to_local(self, rot: Quat) -> Moment<Local> {
        let offset = cordinate_systems::Global::to_local(self.offset, rot);
        let force = cordinate_systems::Global::to_local(self.force, rot);
        Moment::new_local(offset, force)
    }
}

impl<S: ConvertCordinateSystem> Moment<S> {
    /// Gets the part of the moment that affects translation
    ///
    /// ```rust
    /// # use bevy::math::Vec3;
    /// # use physics::components::force::{Moment, Force};
    /// let m = Moment::new_global(Vec3::ZERO, Vec3::X);
    ///
    /// assert_eq!(m.get_force(), Force(Vec3::X));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_force(&self) -> Force {
        Force(self.force)
    }

    /// Gets the part of the moment affecting rotation
    ///
    /// ```rust
    /// # use bevy::math::Vec3;
    /// # use physics::components::force::{Moment, Torque};
    /// let m = Moment::new_global(Vec3::X, Vec3::Y);
    ///
    /// assert_eq!(m.get_torque(), Torque(Vec3::Z));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_torque(&self) -> Torque {
        match self.offset.try_normalize() {
            None => Torque(Vec3::ZERO),
            Some(offset) => {
                let radial = self.force.project_onto_normalized(offset);
                let torque = self.offset.cross(self.force - radial);

                Torque(torque)
            }
        }
    }

    /// Gets both the torque and force as a tuple
    ///
    /// ```rust
    /// # use bevy::math::Vec3;
    /// # use physics::components::force::Moment;
    /// let m = Moment::new_global(Vec3::Z, Vec3::ONE);
    ///
    /// let (t, f) = m.get_parts();
    ///
    /// assert_eq!(t.0, Vec3::new(-1.0, 1.0, 0.0));
    /// assert_eq!(f.0, Vec3::ONE);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_parts(&self) -> (Torque, Force) {
        (self.get_torque(), self.get_force())
    }
}

impl<S: ConvertCordinateSystem> From<Moment<S>> for Force {
    #[inline]
    fn from(value: Moment<S>) -> Self {
        value.get_force()
    }
}

// Addition
overload!((a: ?Force) + (b: ?Force) -> Force {Force(a.0 + b.0)});
overload!((a: &mut Force) += (b: ?Force) {a.0 += b.0});

// Subtraction
overload!((a: ?Force) - (b: ?Force) -> Force {Force(a.0 - b.0)});
overload!((a: &mut Force) -= (b: ?Force) {a.0 -= b.0});

// Multiplication
overload!((a: ?Force) * (b: ?Force) -> Force {Force(a.0 * b.0)});
overload!((a: ?Force) * (b: f32) -> Force {Force(a.0 * b)});
overload!((a: &mut Force) *= (b: ?Force) {a.0 *= b.0});
overload!((a: &mut Force) *= (b: f32) {a.0 *= b});

// Divivision
overload!((a: ?Force) / (b: ?Force) -> Force {Force(a.0 / b.0)});
overload!((a: ?Force) / (b: ?Inertia) -> Acceleration {Acceleration(a.0 / b.mass)});
overload!((a: ?Force) / (b: f32) -> Force {Force(a.0 / b)});
overload!((a: &mut Force) /= (b: ?Force) {a.0 /= b.0});
overload!((a: &mut Force) /= (b: f32) {a.0 /= b});

// Negate
overload!(- (a: &mut Force) -> Force {Force(- a.0)});

impl<S: ConvertCordinateSystem> From<Moment<S>> for Torque {
    #[inline]
    fn from(value: Moment<S>) -> Self {
        value.get_torque()
    }
}

// Addition
overload!((a: ?Torque) + (b: ?Torque) -> Torque {Torque(a.0 + b.0)});
overload!((a: &mut Torque) += (b: ?Torque) {a.0 += b.0});

// Subtraction
overload!((a: ?Torque) - (b: ?Torque) -> Torque {Torque(a.0 - b.0)});
overload!((a: &mut Torque) -= (b: ?Torque) {a.0 -= b.0});

// Multiplication
overload!((a: ?Torque) * (b: ?Torque) -> Torque {Torque(a.0 * b.0)});
overload!((a: ?Torque) * (b: f32) -> Torque {Torque(a.0 * b)});
overload!((a: &mut Torque) *= (b: ?Torque) {a.0 *= b.0});
overload!((a: &mut Torque) *= (b: f32) {a.0 *= b});

// Divivision
overload!((a: ?Torque) / (b: ?Torque) -> Torque {Torque(a.0 / b.0)});
overload!((a: ?Torque) / (b: f32) -> Torque {Torque(a.0 / b)});
overload!((a: &mut Torque) /= (b: ?Torque) {a.0 /= b.0});
overload!((a: &mut Torque) /= (b: f32) {a.0 /= b});

// Negate
overload!(- (a: &mut Torque) -> Torque {Torque(- a.0)});

#[cfg(test)]
mod parts {
    use super::Moment;
    use bevy::math::Vec3;

    #[test]
    fn torque() {
        let get_torque = |offset, force| Moment::new_local(offset, force).get_torque().0;

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
