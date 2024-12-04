use bevy::ecs::{bundle::Bundle, component::Component};
use bevy::prelude::Transform;

use crate::components::acceleration::Acceleration;
use crate::components::inertia::Inertia;
use crate::components::velocity::{AngularVelocity, Velocity};
use crate::cordinate_systems::{CoordinateSystem, Local};

pub mod acceleration;
pub mod force;
pub mod inertia;
pub mod velocity;

#[derive(Bundle)]
pub struct SimulationBundle<
    LV: CoordinateSystem + Component,
    AV: CoordinateSystem + Component,
    LA: CoordinateSystem + Component,
> {
    pub transform: Transform,
    pub sim: Simulated,
    pub vel: Velocity<LV>,
    pub angvel: AngularVelocity<AV>,
    pub inertia: Inertia<Local>,
    pub acc: Acceleration<LA>,
}

impl<
    LV: CoordinateSystem + Component,
    AV: CoordinateSystem + Component,
    LA: CoordinateSystem + Component,
> SimulationBundle<LV, AV, LA> {
    pub fn new(
        vel: Velocity<LV>,
        acc: Acceleration<LA>,
        angvel: AngularVelocity<AV>,
        inertia: Inertia<Local>,
    ) -> Self {
        Self {
            transform: Transform::default(),
            sim: Simulated,
            vel,
            angvel,
            inertia,
            acc,
        }
    }
    pub fn new_with_gravity(vel: Velocity<LV>, inertia: Inertia<Local>) -> Self {
        Self::new(vel, Acceleration::GRAVITY, AngularVelocity::ZERO, inertia)
    }
}

/// Marker that designates entites to be simulated
///
/// Remove to easily stop something from being simulated.
#[derive(Component, Debug)]
pub struct Simulated;
