use bevy::ecs::{bundle::Bundle, component::Component};
use bevy::prelude::SpatialBundle;

use crate::components::acceleration::Acceleration;
use crate::components::inertia::Inertia;
use crate::components::velocity::{AngularVelocity, Velocity};

pub mod acceleration;
pub mod force;
pub mod inertia;
pub mod velocity;

#[derive(Bundle)]
pub struct SimulationBundle {
    pub spatial: SpatialBundle,
    pub sim: Simulated,
    pub vel: Velocity,
    pub angvel: AngularVelocity,
    pub inertia: Inertia,
    pub acc: Acceleration,
}

impl SimulationBundle {
    pub fn new(
        vel: Velocity,
        acc: Acceleration,
        angvel: AngularVelocity,
        inertia: Inertia,
    ) -> Self {
        Self {
            spatial: SpatialBundle::default(),
            sim: Simulated,
            vel,
            angvel,
            inertia,
            acc,
        }
    }
    pub fn new_with_gravity(vel: Velocity, inertia: Inertia) -> Self {
        Self::new(vel, Acceleration::GRAVITY, AngularVelocity::ZERO, inertia)
    }
}

/// Marker that designates entites to be simulated
///
/// Remove to easily stop something from being simulated.
#[derive(Component, Debug)]
pub struct Simulated;
