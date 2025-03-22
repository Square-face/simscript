use bevy::{
    app::{FixedUpdate, Plugin, Update},
    color::Color,
    prelude::{Bundle, Component, Gizmos, Query, Res, Transform, Visibility},
    time::Time,
};
use simscript_physics::State;

#[derive(Component)]
pub struct SimState(pub State);

#[derive(Bundle)]
#[allow(dead_code)]
pub struct SimulationBundle {
    pub transform: Transform,
    pub visibility: Visibility,
    pub state: SimState,
}

impl SimulationBundle {
    pub fn new(state: State) -> Self {
        let transform = Transform::default();
        let visibility = Visibility::default();
        Self {
            transform,
            visibility,
            state: SimState(state),
        }
    }
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(FixedUpdate, step);
        app.add_systems(Update, arrows);
    }
}

fn arrows(mut gizmos: Gizmos, query: Query<&SimState>) {
    for state in query.iter() {
        let state = &state.0;

        let pos = state.transform.translation.0;
        let rot = state.transform.rotation.0;
        let vel = state.momentum / state.mass.rotated(rot);

        gizmos.arrow(
            (pos).as_vec3(),
            (pos + vel.linear.0).as_vec3(),
            Color::srgb(0., 1., 0.),
        );

        for panel in state.panels.iter() {
            let rotated = panel.rotated(&rot);
            let mom = panel.to_moment(&vel, &rot);
            let off = rotated.offset + pos;

            gizmos.arrow(
                (off).as_vec3(),
                (off + mom.force.0).as_vec3(),
                Color::srgb(1.0, 0., 0.),
            );
        }
    }
}

fn step(time: Res<Time>, mut query: Query<(&mut Transform, &mut SimState)>) {
    query.par_iter_mut().for_each(|(mut trans, mut state)| {
        state.0.runge_kutta_4(time.delta());

        trans.translation = state.0.transform.translation.0.as_vec3();
        trans.rotation = state.0.transform.rotation.0.as_quat();
    });
}
