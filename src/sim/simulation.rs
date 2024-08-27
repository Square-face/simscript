use bevy::{
    ecs::{component::Component, system::{Query, Res}}, math::Vec3, time::Time, transform::components::Transform
};

#[derive(Component)]
pub struct Simulated {
    pub vel: Velocity,
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

pub fn update(time: Res<Time>, mut query: Query<(&mut Transform, &Simulated)>) {
    for (mut trans, sim) in query.iter_mut() {
        trans.translation += sim.vel.0*time.delta_seconds();
    }
}
