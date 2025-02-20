use bevy::{
    app::{FixedUpdate, Plugin}, prelude::{Bundle, Component, Query, Res, Transform, Visibility}, time::Time
};
use simscript_physics::State;

#[derive(Component)]
pub struct SimState(pub State);

#[derive(Bundle)]
#[allow(dead_code)]
pub struct SimulationBundle { pub transform: Transform, pub visibility: Visibility, pub state: SimState }

impl SimulationBundle {
    pub fn new(state: State) -> Self {
        let transform = Transform::default();
        let visibility = Visibility::default();
        Self { transform, visibility, state: SimState(state) }
    }
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(FixedUpdate, step);
    }
}

fn step(time: Res<Time>, mut query: Query<(&mut Transform, &mut SimState)>) {
    for (mut trans, mut state) in query.iter_mut() {
        state.0.step_movement(time.elapsed());

        dbg!(state.0, &trans);

        trans.translation = state.0.transform.translation.0.as_vec3();
        trans.rotation = state.0.transform.rotation.0.as_quat();
    }
}
