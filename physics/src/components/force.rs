use bevy::math::Vec3;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Represents a force that is not applied at the center of mass
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Moment {
    /// Offset the applied force from the origin
    offset: Vec3,

    /// The force being applied
    force: Vec3,
}

/// Represents a force applied at the center of mass
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Force(pub Vec3);

/// Represents a torque being applied on a object
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Torque(pub Vec3);

impl Moment {
    /// [Moment] with no force in any direction
    pub const ZERO: Self = Self::new(Vec3::ZERO, Vec3::ZERO);

    /// Create a new [Moment] from an offset and a force
    #[inline]
    #[must_use]
    pub const fn new(offset: Vec3, force: Vec3) -> Self {
        Self { offset, force }
    }

    /// Gets the part of the moment that affects translation
    ///
    /// ```rust
    /// # use bevy::math::Vec3;
    /// # use physics::components::force::{Moment, Force};
    /// let m = Moment::new(Vec3::ZERO, Vec3::X);
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
    /// let m = Moment::new(Vec3::X, Vec3::Y);
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
    /// let m = Moment::new(Vec3::Z, Vec3::ONE);
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

impl From<Moment> for Force {
    #[inline]
    fn from(value: Moment) -> Self {
        value.get_force()
    }
}

impl Add for Force {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Force {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Force {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Force {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for Force {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for Force {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Div for Force {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for Force {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl Mul<f32> for Force {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<f32> for Force {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs
    }
}

impl From<Moment> for Torque {
    #[inline]
    fn from(value: Moment) -> Self {
        value.get_torque()
    }
}

impl Add for Torque {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Torque {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Torque {
    type Output = Torque;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Torque {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for Torque {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for Torque {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Div for Torque {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for Torque {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl Mul<f32> for Torque {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<f32> for Torque {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs
    }
}

#[cfg(test)]
mod parts {
    use super::Moment;
    use bevy::math::Vec3;

    #[test]
    fn torque() {
        let get_torque = |offset, force| Moment::new(offset, force).get_torque().0;

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
