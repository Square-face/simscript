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
    prelude::{ChildBuild, PluginGroup, Transform, Visibility},
    render::camera::ClearColor,
    scene::SceneRoot,
    window::{PresentMode, Window, WindowPlugin},
    DefaultPlugins,
};

use entity::SimulationBundle;
use physics::{
    components::{
        acceleration::{Acceleration, AngularAcceleration},
        inertia::Inertia,
        velocity::{AngularVelocity, Velocity},
    },
    coordinate_systems::Global,
};
use simulation::simulation_step;
use ui::{
    camera::{CameraPlugin, CameraTarget},
    grid::GridPlugin,
};

mod entity;
mod simulation;

fn main() {
    let logging = LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,simscript=info".into(),
        level: bevy::log::Level::DEBUG,
        ..Default::default()
    };

    let window = WindowPlugin {
        primary_window: Some(Window {
            title: "SimScript".to_string(),
            name: Some("sq8".to_string()),
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        }),
        ..Default::default()
    };

    let default = DefaultPlugins.set(logging).set(window);

    App::new()
        .add_plugins(default)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(GridPlugin)
        .add_systems(Startup, (spawn_tests,))
        .add_plugins(simulation_step)
        .run();
}

fn spawn_tests(mut commands: Commands, ass: Res<AssetServer>) {
    let arrow = ass.load("arrow.glb#Scene0");

    commands
        .spawn((
            SimulationBundle {
                position: Transform::default(),
                visibility: Visibility::default(),
                velocity: Velocity::<Global>::new(Vec3::ONE * 10.),
                angvel: AngularVelocity::ZERO,
                acceleration: Acceleration::new(Vec3::NEG_Y * 9.82),
                angaccel: AngularAcceleration::ZERO,
                inertia: Inertia::cylinder_x(30., 5., 40.),
            },
            CameraTarget,
        ))
        .with_children(|parent| {
            parent.spawn(SceneRoot(arrow.clone()));
        });

    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 100.0,
    });

    commands.insert_resource(ClearColor(BLACK.into()));
}
