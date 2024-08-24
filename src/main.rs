use bevy::{
    app::{App, Startup},
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{component::Component, system::Commands},
    math::Vec3,
    transform::components::Transform,
    utils::default,
    DefaultPlugins,
};

#[derive(Component)]
struct PrimaryCameraMarker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera,))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PrimaryCameraMarker,
    ));
}
