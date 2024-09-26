use bevy::ecs::{bundle::Bundle, component::Component};
use bevy::math::Vec3;
use bevy::prelude::SpatialBundle;

use crate::components::inertia::Inertia;
use crate::components::velocity::{AngularVelocity, Velocity};
use crate::components::acceleration::Accelerator;

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
    pub acc: Accelerator,
}

impl SimulationBundle {
    pub fn new(vel: Velocity, acc: Accelerator, angvel: AngularVelocity, inertia: Inertia) -> Self {
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
        Self::new(
            vel,
            Accelerator::GRAVITY,
            AngularVelocity::ZERO,
            inertia,
        )
    }
}

/// Marker that designates entites to be simulated
///
/// Remove to easily stop something from being simulated.
#[derive(Component, Debug)]
pub struct Simulated;
