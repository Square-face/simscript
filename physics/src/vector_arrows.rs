use bevy::{
    color::Color,
    ecs::{query::With, system::Query},
    gizmos::gizmos::Gizmos,
    transform::components::Transform,
};

pub fn velocity(
    query: Query<(&Transform, &crate::Velocity), With<crate::Simulated>>,
    mut gizmos: Gizmos,
) {
    for (trans, vel) in query.iter() {
        gizmos.arrow(
            trans.translation, // from object center
            trans.translation + vel.0, // to object center + acceleration
            Color::srgb(0.65, 0.0, 0.0),
        );
    }
}

pub fn acceleration(
    query: Query<(&Transform, &crate::Accelerator), With<crate::Simulated>>,
    mut gizmos: Gizmos,
) {
    for (trans, acc) in query.iter() {
        gizmos.arrow(
            trans.translation, // from object center
            trans.translation + acc.0, // to object center + acceleration
            Color::srgb(0.0, 0.0, 0.65),
        );
    }
}
