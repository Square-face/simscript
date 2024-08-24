use bevy::prelude::PluginGroup;
use bevy::{
    app::{App, Startup},
    asset::AssetServer,
    ecs::system::{Commands, Res},
    log::LogPlugin,
    scene::SceneBundle,
    utils::default,
    DefaultPlugins,
};
use camera::CameraPlugin;

mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,simscript=debug".into(),
            level: bevy::log::Level::DEBUG,
            custom_layer: |_| None,
        }))
        .add_plugins(CameraPlugin)
        .add_systems(Startup, (spawn_cube,))
        .run();
}

fn spawn_cube(mut commands: Commands, ass: Res<AssetServer>) {
    let cube = ass.load("cube.glb#Scene0");
    commands.spawn(SceneBundle {
        scene: cube,
        ..default()
    });
}
