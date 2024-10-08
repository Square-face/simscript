use bevy::app::{Plugin, PostUpdate, Update};
use bevy::ecs::query::With;
use bevy::ecs::system::{Query, Res};
use bevy::math::{Quat, Vec3};
use bevy::time::Time;
use bevy::transform::components::Transform;

use components::acceleration::Accelerator;
use components::force::Moment;
use components::inertia::Inertia;

pub mod components;
mod vector_arrows;

pub struct SimulatiorPlugin;

impl Plugin for SimulatiorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_simulated);
        app.add_systems(Update, update_simulated);
        app.add_systems(
            PostUpdate,
            (vector_arrows::velocity, vector_arrows::acceleration),
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
            &mut components::velocity::Velocity,
            &mut components::velocity::AngularVelocity,
            &Inertia,
            Option<&Accelerator>,
        ),
        With<components::Simulated>,
    >,
) {
    let delta = time.delta_seconds();
    let half_delta = delta / 2.0;

    for (mut trans, mut vel, mut angvel, inertia, acc) in accelerators.iter_mut() {
        let acc = acc.unwrap_or(&Accelerator::ZERO);

        let (torque, _force) = Moment::new(Vec3::Z, Vec3::new(0.0, 10.0, 0.0)).get_parts();
        let angacc = inertia.get_angular_acceleration(torque);

        // Accelerate and move
        vel.accelerate(acc, half_delta);
        angvel.0 += angacc * half_delta;

        trans.translation += vel.0 * delta;

        let delta_rot =
            Quat::from_vec4((angvel.0 * delta / 2.0).extend(trans.rotation.w * delta / 2.0));

        if delta_rot.w != 0.0 {
            trans.rotation = (trans.rotation + delta_rot.normalize() * trans.rotation).normalize();
        }

        angvel.0 += angacc * half_delta;
        vel.accelerate(acc, half_delta);
    }
}
