use bevy::app::{Plugin, Update};
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::log::info;
use bevy::math::Quat;
use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    math::Vec3,
    time::Time,
    transform::components::Transform,
};
use force::Force;
use inertia::Inertia;

pub mod components;
pub mod force;
pub mod inertia;
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
            &mut components::AngularVelocity,
            &Inertia,
            Option<&components::Accelerator>,
        ),
        With<components::Simulated>,
    >,
) {
    for (mut trans, mut vel, mut angvel, inertia, acc) in accelerators.iter_mut() {
        let acc = acc.map_or(Vec3::ZERO, |a| a.0);
        let force = Force::new(Vec3::Z, Vec3::new(0.0, 10.0, 0.0));
        let angacc = inertia.get_angular_acceleration(force.get_torque());

        // Accelerate and move
        vel.0 += acc * time.delta_seconds() * 0.5;
        angvel.0 += angacc * time.delta_seconds() * 0.5;

        trans.translation += vel.0 * time.delta_seconds();
        let delta = time.delta_seconds();
        let delta_rot =
            Quat::from_vec4((angvel.0 * delta / 2.0).extend(trans.rotation.w * delta / 2.0))
                * trans.rotation;
        if delta_rot.w != 0.0 {
            trans.rotation = (trans.rotation + delta_rot).normalize();
        }

        angvel.0 += angacc * time.delta_seconds() * 0.5;
        vel.0 += acc * time.delta_seconds() * 0.5;
    }
}
