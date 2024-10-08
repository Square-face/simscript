use bevy::{
    app::{App, Startup},
    asset::AssetServer,
    color::palettes::css::{BLACK, WHITE},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    log::LogPlugin,
    math::Vec3,
    pbr::AmbientLight,
    prelude::PluginGroup,
    render::camera::ClearColor,
    scene::SceneBundle,
    transform::components::Transform,
    utils::default,
    window::{PresentMode, Window, WindowPlugin},
    DefaultPlugins,
};

use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin, InfiniteGridSettings};
use physics::components::{
    acceleration::Accelerator, inertia::Inertia, velocity::AngularVelocity, SimulationBundle,
    velocity::Velocity,
};
use ui::camera::{CameraPlugin, CameraTarget};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "debug,wgpu_core=warn,wgpu_hal=warn,simscript=debug".into(),
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
        .add_plugins(InfiniteGridPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(physics::SimulatiorPlugin)
        .add_systems(Startup, (spawn_tests,))
        .run();
}

fn spawn_tests(mut commands: Commands, ass: Res<AssetServer>) {
    let arrow = ass.load("arrow.glb#Scene0");

    commands
        .spawn((
            SimulationBundle::new(
                Velocity(Vec3::new(100.0, 100.0, 0.0)),
                Accelerator(Vec3::Y * -9.82),
                AngularVelocity(Vec3::ZERO),
                Inertia::cylinder_x(20.0, 0.5, 50.0),
            ),
            CameraTarget,
        ))
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: arrow.clone(),
                transform: Transform::from_scale(Vec3 {
                    x: -1.0,
                    y: 1.0,
                    z: 1.0,
                })
                .with_translation(Vec3::Y * 0.15),
                ..default()
            });
        });

    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 100.0,
    });

    commands.insert_resource(ClearColor(BLACK.into()));

    commands.spawn(InfiniteGridBundle {
        settings: InfiniteGridSettings {
            fadeout_distance: 1000.0,
            ..Default::default()
        },
        ..Default::default()
    });
}
