use std::f32::consts::PI;

use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res},
    },
    math::{Quat, Vec3, Vec3Swizzles},
    time::Time,
    transform::components::Transform,
};
use bevy::app::{Plugin, Update};

pub struct SimulatiorPlugin;

impl Plugin for SimulatiorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_simulated);
    }
}

#[derive(Component)]
pub struct Simulated;

#[derive(Component)]
pub struct Velocity(pub Vec3);

/// Applies a constant acceleration
#[derive(Component)]
pub struct Accelerator(pub Vec3);

/// Makes the entity rotate into the direction of travel
#[derive(Component)]
pub struct AlignToForward;

/// Updates objects with acceleration
#[allow(clippy::type_complexity)]
pub fn update_simulated(
    time: Res<Time>,
    mut accelerators: Query<
        (
            &mut Transform,
            &mut Velocity,
            Option<&Accelerator>,
            Option<&AlignToForward>,
        ),
        With<Simulated>,
    >,
) {
    for (mut trans, mut vel, acc, align) in accelerators.iter_mut() {
        let acc = acc.map_or(Vec3::ZERO, |a| a.0);
        let _align = align.is_some();

        // Rotate object into the direction of travel
        let vel_dir = Quat::from_vec4(vel.0.xyz().normalize_or(Vec3::Y).extend(0.0))
            .mul_quat(Quat::from_rotation_z(PI / 2.0));

        trans.rotation = vel_dir;

        // Accelerate and move
        vel.0 += acc * time.delta_seconds() * 0.5;
        trans.translation += vel.0 * time.delta_seconds();
        vel.0 += acc * time.delta_seconds() * 0.5;
    }
}
