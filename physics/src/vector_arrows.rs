use bevy::{
    color::Color,
    ecs::{query::With, system::Query},
    gizmos::gizmos::Gizmos,
    math::Vec3,
    transform::components::Transform,
};

use crate::components::{self, acceleration::Accelerator, velocity::Velocity, Simulated};

pub fn velocity(
    query: Query<(&Transform, &Velocity), With<Simulated>>,
    mut gizmos: Gizmos,
) {
    for (trans, vel) in query.iter() {
        let (pos, vel) = (trans.translation, vel.0);

        // skip drawing if the velocity is 0
        if vel == Vec3::ZERO {
            continue;
        }

        gizmos.arrow(
            pos,       // from object center
            pos + vel, // to object center + acceleration
            Color::srgb(0.65, 0.0, 0.0),
        );
    }
}

pub fn acceleration(
    query: Query<(&Transform, &Accelerator), With<Simulated>>,
    mut gizmos: Gizmos,
) {
    for (trans, acc) in query.iter() {
        let (pos, acc) = (trans.translation, acc.0);

        // skip drawing if the acceleration is 0
        if acc == Vec3::ZERO {
            continue;
        }

        gizmos.arrow(
            pos,       // from object center
            pos + acc, // to object center + acceleration
            Color::srgb(0.0, 0.0, 0.65),
        );
    }
}
