use bevy::math::Vec3;

#[derive(Debug, PartialEq)]
pub struct Moment {
    /// Offset the applied force from the origin
    offset: Vec3,

    /// The force being applied
    magnitude: Vec3,
}

impl Moment {
    pub fn new(offset: Vec3, magnitude: Vec3) -> Self {
        Self { offset, magnitude }
    }

    /// Gets the part of the force not participating in creating torque
    ///
    /// ```rust
    /// # use bevy::math::Vec3;
    /// # use physics::force::Force;
    /// let f = Force::new(Vec3::ZERO, Vec3::X);
    ///
    /// assert_eq!(f.get_residual(), Vec3::X);
    /// ```
    #[inline]
    pub fn get_residual(&self) -> Vec3 {
        self.get_parts().1
    }

    /// Gets the part of the force not participating in creating torque
    ///
    /// ```rust
    /// # use bevy::math::Vec3;
    /// # use physics::force::Force;
    /// let f = Force::new(Vec3::X, Vec3::Y);
    ///
    /// assert_eq!(f.get_torque(), Vec3::Z);
    /// ```
    #[inline]
    pub fn get_torque(&self) -> Vec3 {
        self.get_parts().0
    }

    pub fn get_parts(&self) -> (Vec3, Vec3) {
        match self.offset.try_normalize() {
            None => (Vec3::ZERO, self.magnitude),
            Some(_) => {
                let res = self.magnitude.project_onto(self.offset);
                let torq = self.offset.cross(self.magnitude - res);

                (torq, res)
            }
        }
    }
}

#[cfg(test)]
mod parts {
    use super::Moment;
    use bevy::math::Vec3;

    #[test]
    fn residual() {
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
            assert_eq!(f.get_residual(), v, "pure force failed: {f:?}");

            let f = Moment::new(v, v);
            assert_eq!(f.get_residual(), v, "double {f:?}");
        }

        let f = Moment::new(Vec3::X, Vec3::ONE);
        assert_eq!(f.get_residual(), Vec3::ONE.with_y(0.0).with_z(0.0));
    }

    #[test]
    fn torque() {
        assert_eq!(Moment::new(Vec3::Y, Vec3::Z).get_torque(), Vec3::X);
        assert_eq!(Moment::new(Vec3::X, Vec3::Y).get_torque(), Vec3::Z);
        assert_eq!(Moment::new(Vec3::X, Vec3::Z).get_torque(), Vec3::NEG_Y);
    }
}
