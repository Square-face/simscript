use bevy::{
    app::{App, Startup},
    asset::AssetServer,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{
        component::Component,
        system::{Commands, Res},
    },
    math::Vec3,
    scene::SceneBundle,
    transform::components::Transform,
    utils::default,
    DefaultPlugins,
};

#[derive(Component)]
struct PrimaryCameraMarker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, spawn_cube))
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

fn spawn_cube(mut commands: Commands, ass: Res<AssetServer>) {
    let cube = ass.load("cube.glb#Scene0");
    commands.spawn(SceneBundle {
        scene: cube,
        ..default()
    });
}
