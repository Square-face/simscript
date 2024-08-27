use bevy::app::{Plugin, Update};

pub mod simulation;

pub struct SimulatiorPlugin;

impl Plugin for SimulatiorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, simulation::update);
    }
}
