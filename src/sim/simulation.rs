use bevy::{
    ecs::{component::Component, query::With, system::{Query, Res}}, math::Vec3, time::Time, transform::components::Transform
};

#[derive(Component)]
pub struct Simulated;

#[derive(Component)]
pub struct Velocity(pub Vec3);

pub fn update(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Simulated>>) {
    for (mut trans, vel) in query.iter_mut() {
        trans.translation += vel.0*time.delta_seconds();
    }
}
