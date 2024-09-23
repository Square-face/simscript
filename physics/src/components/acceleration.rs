use bevy::{ecs::component::Component, math::Vec3};

///
/// Applies a constant acceleration
///
/// Works similar to [Velocity] in that the acceleration is represented as a Vec3 in global
/// cordinates
#[derive(Component, Debug)]
pub struct Accelerator(pub Vec3);

impl Accelerator {
    /// Returns a accelerator that simulates gravity.
    /// i.e a negative Y acceleration of 9.82 m/s^2
    ///
    /// ```
    /// # use bevy::prelude::Vec3;
    /// # use physics::components::acceleration::Accelerator;
    /// let acc = Accelerator::gravity();
    ///
    /// assert_eq!(acc.0, Vec3{x:0.0, y:-9.82, z:0.0});
    /// ```
    pub fn gravity() -> Self {
        Self(Vec3::NEG_Y * 9.82)
    }
}

