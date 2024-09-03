use bevy::app::{Plugin, Update};
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    math::Vec3,
    time::Time,
    transform::components::Transform,
};

pub mod components;
mod vector_arrows;

pub struct SimulatiorPlugin;

impl Plugin for SimulatiorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_simulated);
        app.add_systems(
            Update,
            (vector_arrows::velocity, vector_arrows::acceleration).after(update_simulated),
        );
    }
}

/// Updates objects with acceleration
#[allow(clippy::type_complexity)]
pub fn update_simulated(
    time: Res<Time>,
    mut accelerators: Query<
        (
            &mut Transform,
            &mut components::Velocity,
            Option<&components::Accelerator>,
        ),
        With<components::Simulated>,
    >,
) {
    for (mut trans, mut vel, acc) in accelerators.iter_mut() {
        let acc = acc.map_or(Vec3::ZERO, |a| a.0);

        // Accelerate and move
        vel.0 += acc * time.delta_seconds() * 0.5;

        trans.translation += vel.0 * time.delta_seconds();
        trans.rotation = vel.to_direction();

        vel.0 += acc * time.delta_seconds() * 0.5;
    }
}
