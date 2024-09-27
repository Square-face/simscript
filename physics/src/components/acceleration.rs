use bevy::{ecs::component::Component, math::Vec3};

///
/// Applies a constant acceleration
///
/// Works similar to [Velocity] in that the acceleration is represented as a Vec3 in global
/// cordinates
#[derive(Component, Debug)]
pub struct Accelerator(pub Vec3);

impl Accelerator {
    /// [Accelerator] that doesn't accelerate in any direction
    pub const ZERO: Self = Self(Vec3::ZERO);

    /// [Accelerator] that mimics gravity (-9.82 m/s^2 in y velocity)
    pub const GRAVITY: Self = Self(Vec3::new(0.0, -9.82, 0.0));
}

