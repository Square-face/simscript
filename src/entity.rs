use bevy::prelude::{Bundle, Transform, Visibility};
use physics::{
    components::{
        acceleration::{Acceleration, AngularAcceleration},
        inertia::Inertia,
        velocity::{AngularVelocity, Velocity},
    },
    coordinate_systems::{Global, Local},
};

#[derive(Bundle)]
pub struct SimulationBundle {
    pub position: Transform,
    pub visibility: Visibility,
    pub velocity: Velocity<Global>,
    pub angvel: AngularVelocity<Global>,
    pub acceleration: Acceleration<Global>,
    pub angaccel: AngularAcceleration<Global>,
    pub inertia: Inertia<Local>,
}
