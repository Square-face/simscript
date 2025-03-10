use bevy::{
    app::{FixedUpdate, Plugin, Update},
    color::Color,
    prelude::{Bundle, Component, Gizmos, IntoSystemConfigs, Query, Res, Transform, Visibility},
    time::Time,
};
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
        app.add_systems(FixedUpdate, (moments, step).chain());
        app.add_systems(Update, arrows);
    }
}

fn arrows(mut gizmos: Gizmos, query: Query<(&SimState, &Panels)>) {
    for (state, panels) in query.iter() {
        let state = &state.0;
        for panel in &panels.0 {
            let pos = state.transform.translation.0;
            let rot = state.transform.rotation.0;
            let off = rot.mul_vec3(panel.offset);
            let norm = rot.mul_vec3(panel.normal);

            let vel = state.momentum / state.mass;
            let tip_vel = panel.tip_velocity(&rot, &vel);
            let rel_vel = rot.inverse().mul_vec3(tip_vel.0);
            let mom = panel.to_moment(state);

            gizmos.arrow(
                (pos + off).as_vec3(),
                (pos + off + norm).as_vec3(),
                Color::srgb(0.0, 0.0, 1.0),
            );
            gizmos.arrow(
                (pos + mom.offset).as_vec3(),
                (pos + mom.offset + mom.force).as_vec3(),
                Color::srgb(1.0, 0., 0.),
            );
            gizmos.arrow(
                (pos + off).as_vec3(),
                (pos + off + rel_vel).as_vec3(),
                Color::srgb(0.0, 1., 0.),
            );
        }
    }
}

fn moments(time: Res<Time>, mut query: Query<(&mut SimState, &Panels)>) {
    for (mut state, panels) in query.iter_mut() {
        let state = &mut state.0;

        let momentum: Momentum = panels
            .0
            .iter()
            .map(|panel| dbg!(dbg!(panel.to_moment(state)) * time.delta()))
            .reduce(|acc, e| acc + e)
            .unwrap_or(Momentum::ZERO);

        state.momentum += momentum;
    }
}

fn step(time: Res<Time>, mut query: Query<(&mut Transform, &mut SimState)>) {
    for (mut trans, mut state) in query.iter_mut() {
        state.0.step_movement(time.delta());

        trans.translation = state.0.transform.translation.0.as_vec3();
        trans.rotation = state.0.transform.rotation.0.as_quat();
    }
}
