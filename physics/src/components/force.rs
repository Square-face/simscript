use bevy::math::Vec3;

/// Represents a force that is not applied at the center of mass
#[derive(Debug, PartialEq)]
pub struct Moment {
    /// Offset the applied force from the origin
    offset: Vec3,

    /// The force being applied
    force: Vec3,
}

/// Represents a force applied at the center of mass
#[derive(Debug, PartialEq)]
pub struct Force(pub Vec3);

/// Represents a torque being applied on a object
#[derive(Debug, PartialEq)]
pub struct Torque(pub Vec3);

impl Moment {
    /// Create a new [Moment] from an offset and a force
    #[inline]
    #[must_use]
    pub fn new(offset: Vec3, force: Vec3) -> Self {
        Self { offset, force }
    }

    /// Create a new [Moment] that is just a force and no offset
    #[inline]
    #[must_use]
    pub fn from_force(force: Vec3) -> Self {
        Self::new(Vec3::ZERO, force)
    }

    /// Gets the part of the force not participating in creating torque
    ///
    /// ```rust
    /// # use bevy::math::Vec3;
    /// # use physics::force::{Moment, Force};
    /// let m = Moment::new(Vec3::ZERO, Vec3::X);
    ///
    /// assert_eq!(m.get_force(), Force(Vec3::X));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_force(&self) -> Force {
        self.get_parts().1
    }

    /// Gets the part of the force not participating in creating torque
    ///
    /// ```rust
    /// # use bevy::math::Vec3;
    /// # use physics::force::{Moment, Torque};
    /// let m = Moment::new(Vec3::X, Vec3::Y);
    ///
    /// assert_eq!(m.get_torque(), Torque(Vec3::Z));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_torque(&self) -> Torque {
        self.get_parts().0
    }

    /// Gets both the torque and force as a tuple
    ///
    /// ```rust
    /// # use bevy::math::Vec3;
    /// # use physics::force::{Moment, Torque};
    /// let m = Moment::new(Vec3::Z, Vec3::ONE);
    ///
    /// let (t, f) = m.get_parts();
    ///
    /// assert_eq!(t.0, Vec3::new(-1.0, 1.0, 0.0));
    /// assert_eq!(f.0, Vec3::Z);
    /// ```
    #[must_use]
    pub fn get_parts(&self) -> (Torque, Force) {
        // If its not possible to normilize the offset, then it consists of only Zeroes and there
        // is no torque
        match self.offset.try_normalize() {
            None => (Torque(Vec3::ZERO), Force(self.force)),
            Some(offset) => {
                let force = self.force.project_onto_normalized(offset);
                let torq = self.offset.cross(self.force - force);

                (Torque(torq), Force(force))
            }
        }
    }
}

impl From<Moment> for Force {
    fn from(value: Moment) -> Self {
        value.get_force()
    }
}

impl From<Moment> for Torque {
    fn from(value: Moment) -> Self {
        value.get_torque()
    }
}

#[cfg(test)]
mod parts {
    use super::Moment;
    use bevy::math::Vec3;

    #[test]
    fn force() {
        for v in [
            Vec3::ZERO,
            Vec3::X,
            Vec3::Y,
            Vec3::Z,
            Vec3::NEG_X,
            Vec3::NEG_Y,
            Vec3::NEG_Z,
        ] {
            let f = Moment::from_force(v);
            assert_eq!(f.get_force().0, v, "pure force failed: {f:?}");

            let f = Moment::new(v, v);
            assert_eq!(f.get_force().0, v, "double {f:?}");
        }

        let f = Moment::new(Vec3::X, Vec3::ONE);
        assert_eq!(f.get_force().0, Vec3::X);
    }

    #[test]
    fn torque() {
        assert_eq!(Moment::new(Vec3::Y, Vec3::Z).get_torque().0, Vec3::X);
        assert_eq!(Moment::new(Vec3::X, Vec3::Y).get_torque().0, Vec3::Z);
        assert_eq!(Moment::new(Vec3::X, Vec3::Z).get_torque().0, Vec3::NEG_Y);
    }
}
