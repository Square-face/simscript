use bevy::{
    app::{App, Startup},
    asset::AssetServer,
    color::palettes::css::{BLACK, WHITE},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    log::LogPlugin,
    pbr::AmbientLight,
    prelude::{ChildBuild, PluginGroup, Transform},
    render::camera::ClearColor,
    scene::SceneRoot,
    window::{PresentMode, Window, WindowPlugin},
    DefaultPlugins,
};

use entity::SimulationBundle;
use simscript_physics::{
    inertia_mass::{Inertia, InnertiaMass, Mass},
    State,
};
use ui::{
    camera::{CameraPlugin, CameraTarget},
    grid::GridPlugin,
};

mod entity;

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
        .run();
}

fn spawn_tests(mut commands: Commands, ass: Res<AssetServer>) {
    let arrow = ass.load("arrow.glb#Scene0");
    let state = State::new_zeroed(InnertiaMass::new(
        Mass::new(80.),
        Inertia::cylinder_x(5., 0.8, 80.),
    ));

    commands
        .spawn((SimulationBundle::new(state), CameraTarget))
        .with_children(|parent| {
            parent.spawn((SceneRoot(arrow.clone()), Transform::from_xyz(0., 0.14, 0.)));
        });

    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 100.0,
    });

    commands.insert_resource(ClearColor(BLACK.into()));
}
