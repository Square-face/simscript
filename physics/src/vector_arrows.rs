use bevy::{
    color::Color,
    ecs::{query::With, system::Query},
    gizmos::gizmos::Gizmos,
    transform::components::Transform,
};

use crate::components::{self, Simulated};

pub fn velocity(
    query: Query<(&Transform, &components::Velocity), With<Simulated>>,
    mut gizmos: Gizmos,
) {
    for (trans, vel) in query.iter() {
        gizmos.arrow(
            trans.translation,         // from object center
            trans.translation + vel.0, // to object center + acceleration
            Color::srgb(0.65, 0.0, 0.0),
        );
    }
}

pub fn acceleration(
    query: Query<(&Transform, &components::Accelerator), With<Simulated>>,
    mut gizmos: Gizmos,
) {
    for (trans, acc) in query.iter() {
        gizmos.arrow(
            trans.translation,         // from object center
            trans.translation + acc.0, // to object center + acceleration
            Color::srgb(0.0, 0.0, 0.65),
        );
    }
}
