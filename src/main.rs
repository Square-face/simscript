use bevy::prelude::PluginGroup;
use bevy::transform::components::Transform;
use bevy::{
    app::{App, Startup},
    asset::AssetServer,
    ecs::system::{Commands, Res},
    log::LogPlugin,
    scene::SceneBundle,
    utils::default,
    DefaultPlugins,
};
use camera::{CameraPlugin, CameraTarget};

mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,simscript=debug".into(),
            level: bevy::log::Level::DEBUG,
            custom_layer: |_| None,
        }))
        .add_plugins(CameraPlugin)
        .add_systems(Startup, (spawn_cubes,))
        .run();
}

fn spawn_cubes(mut commands: Commands, ass: Res<AssetServer>) {
    let cube = ass.load("cube.glb#Scene0");
    commands.spawn((SceneBundle {
        scene: cube.clone(),
        ..default()
    },));
    commands.spawn((SceneBundle {
        scene: cube.clone(),
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        ..default()
    },));
    commands.spawn((SceneBundle {
        scene: cube.clone(),
        transform: Transform::from_xyz(3.0, 3.0, 0.0),
        ..default()
    }, CameraTarget));
}
