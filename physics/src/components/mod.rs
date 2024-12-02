use bevy::ecs::{bundle::Bundle, component::Component};
use bevy::prelude::SpatialBundle;

use crate::components::acceleration::Acceleration;
use crate::components::inertia::Inertia;
use crate::components::velocity::{AngularVelocity, Velocity};
use crate::cordinate_systems::{CoordinateSystem, Local};

pub mod acceleration;
pub mod force;
pub mod inertia;
pub mod velocity;

#[derive(Bundle)]
pub struct SimulationBundle<V: CoordinateSystem + Component> {
    pub spatial: SpatialBundle,
    pub sim: Simulated,
    pub vel: Velocity<V>,
    pub angvel: AngularVelocity,
    pub inertia: Inertia<Local>,
    pub acc: Acceleration,
}

impl<V: CoordinateSystem + Component> SimulationBundle<V> {
    pub fn new(
        vel: Velocity<V>,
        acc: Acceleration,
        angvel: AngularVelocity,
        inertia: Inertia<Local>,
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
    pub fn new_with_gravity(vel: Velocity<V>, inertia: Inertia<Local>) -> Self {
        Self::new(vel, Acceleration::GRAVITY, AngularVelocity::ZERO, inertia)
    }
}

/// Marker that designates entites to be simulated
///
/// Remove to easily stop something from being simulated.
#[derive(Component, Debug)]
pub struct Simulated;
