use std::f32::consts::PI;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{Quat, Vec3};
use bevy::prelude::PluginGroup;
use bevy::transform::components::Transform;
use bevy::window::{PresentMode, Window, WindowPlugin};
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
use sim::simulation::{Simulated, Velocity};
use sim::SimulatiorPlugin;

mod camera;
mod keybinds;
mod sim;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,simscript=debug".into(),
                    level: bevy::log::Level::DEBUG,
                    ..Default::default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "SimScript".to_string(),
                        name: Some("sq8".to_string()),
                        present_mode: PresentMode::AutoVsync,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SimulatiorPlugin)
        .add_systems(Startup, (spawn_tests,))
        .run();
}

fn spawn_tests(mut commands: Commands, ass: Res<AssetServer>) {
    let cube = ass.load("cube.glb#Scene0");
    let arrow = ass.load("arrow.glb#Scene0");

    commands.spawn((SceneBundle {
        scene: cube.clone(),
        ..default()
    },));

    commands.spawn((SceneBundle {
        scene: arrow.clone(),
        transform: Transform::from_rotation(Quat::from_rotation_z(PI)),
        ..default()
    },Simulated{
        vel: Velocity(Vec3::X)
    },CameraTarget));
}
