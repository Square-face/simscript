use bevy::math::Vec3;

/// Represents a force that is not applied at the center of mass
#[derive(Debug, PartialEq)]
pub struct Moment {
    /// Offset the applied force from the origin
    offset: Vec3,

    /// The force being applied
    magnitude: Vec3,
}

/// Represents a force applied at the center of mass
#[derive(Debug, PartialEq)]
pub struct Force(pub Vec3);

/// Represents a torque being applied on a object
#[derive(Debug, PartialEq)]
pub struct Torque(pub Vec3);

impl Moment {
    pub fn new(offset: Vec3, magnitude: Vec3) -> Self {
        Self { offset, magnitude }
    }

    /// Gets the part of the force not participating in creating torque
    ///
    /// ```rust
    /// # use bevy::math::Vec3;
    /// # use physics::force::{Moment, Force};
    /// let f = Moment::new(Vec3::ZERO, Vec3::X);
    ///
    /// assert_eq!(f.get_residual(), Force(Vec3::X));
    /// ```
    #[inline]
    pub fn get_force(&self) -> Force {
        self.get_parts().1
    }

    /// Gets the part of the force not participating in creating torque
    ///
    /// ```rust
    /// # use bevy::math::Vec3;
    /// # use physics::force::{Moment, Torque};
    /// let f = Moment::new(Vec3::X, Vec3::Y);
    ///
    /// assert_eq!(f.get_torque(), Torque(Vec3::Z));
    /// ```
    #[inline]
    pub fn get_torque(&self) -> Torque {
        self.get_parts().0
    }

    pub fn get_parts(&self) -> (Torque, Force) {
        match self.offset.try_normalize() {
            None => (Torque(Vec3::ZERO), Force(self.magnitude)),
            Some(_) => {
                let res = self.magnitude.project_onto(self.offset);
                let torq = self.offset.cross(self.magnitude - res);

                (Torque(torq), Force(res))
            }
        }
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
            let f = Moment::new(Vec3::ZERO, v);
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
