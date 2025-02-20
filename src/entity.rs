use bevy::{
    app::{FixedUpdate, Plugin}, prelude::{Component, Query, Res, Transform, Visibility}, time::Time
};
use simscript_physics::State;

#[derive(Component)]
#[allow(dead_code)]
pub struct SimulationBundle { pub transform: Transform, pub visibility: Visibility, pub state: State }

impl SimulationBundle {
    pub fn new(state: State) -> Self {
        let transform = Transform::default();
        let visibility = Visibility::default();
        Self { transform, visibility, state }
    }
}

pub struct SimulationPlugin();

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(FixedUpdate, step);
    }
}

fn step(time: Res<Time>, mut query: Query<&mut SimulationBundle>) {
    for mut bundle in query.iter_mut() {
        bundle.state.step_movement(time.elapsed());

        bundle.transform.translation = bundle.state.transform.translation.0.as_vec3();
    }
}
