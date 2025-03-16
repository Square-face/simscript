use bevy::{
    app::{FixedUpdate, Plugin, Update},
    color::Color,
    prelude::{Bundle, Component, Gizmos, IntoSystemConfigs, Query, Res, Transform, Visibility},
    time::Time,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use simscript_physics::{momentum::Momentum, panels::Panel, State};

#[derive(Component)]
pub struct SimState(pub State);

#[derive(Component)]
pub struct Panels(pub Vec<Panel>);

#[derive(Bundle)]
#[allow(dead_code)]
pub struct SimulationBundle {
    pub transform: Transform,
    pub visibility: Visibility,
    pub state: SimState,
    pub panels: Panels,
}

impl SimulationBundle {
    pub fn new(state: State, panels: Vec<Panel>) -> Self {
        let transform = Transform::default();
        let visibility = Visibility::default();
        Self {
            transform,
            visibility,
            state: SimState(state),
            panels: Panels(panels),
        }
    }
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(FixedUpdate, (moments, step, moments).chain());
        app.add_systems(Update, arrows);
    }
}

fn arrows(mut gizmos: Gizmos, query: Query<(&SimState, &Panels)>) {
    for (state, panels) in query.iter() {
        let state = &state.0;
        for panel in &panels.0 {
            let pos = state.transform.translation.0;
            let mom = panel.to_moment(state);

            gizmos.arrow(
                (pos + mom.0).as_vec3(),
                (pos + mom.0 + mom.1).as_vec3(),
                Color::srgb(1.0, 0., 0.),
            );
        }
    }
}

fn moments(time: Res<Time>, mut query: Query<(&mut SimState, &Panels)>) {
    let delta = time.delta() / 2;
    for (mut state, panels) in query.iter_mut() {
        let state = &mut state.0;

        let momentum: Momentum = panels
            .0
            .iter()
            .map(|panel| panel.to_moment(state) * delta)
            .fold(Momentum::ZERO, |acc, e| acc + e);

        state.momentum += momentum;
    }
}

fn step(time: Res<Time>, mut query: Query<(&mut Transform, &mut SimState)>) {
    query.par_iter_mut().for_each(|(mut trans, mut state)| {
        state.0.step_movement(time.delta());

        trans.translation = state.0.transform.translation.0.as_vec3();
        trans.rotation = state.0.transform.rotation.0.as_quat();
    });
}
