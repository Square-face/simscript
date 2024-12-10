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
    prelude::{ChildBuild, PluginGroup},
    render::camera::ClearColor,
    scene::SceneRoot,
    window::{PresentMode, Window, WindowPlugin},
    DefaultPlugins,
};

use physics::{
    components::{
        acceleration::Acceleration,
        inertia::Inertia,
        velocity::{AngularVelocity, Velocity},
        SimulationBundle,
    },
    cordinate_systems::Global,
};
use ui::camera::{CameraPlugin, CameraTarget};

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
        .add_plugins(physics::SimulatiorPlugin)
        .add_systems(Startup, (spawn_tests,))
        .run();
}

fn spawn_tests(mut commands: Commands, ass: Res<AssetServer>) {
    let arrow = ass.load("arrow.glb#Scene0");

    commands
        .spawn((
            SimulationBundle::new(
                Velocity::<Global>::new(Vec3::new(100.0, 100.0, 0.0)),
                Acceleration::<Global>::GRAVITY,
                AngularVelocity::<Global>::new(Vec3::ZERO),
                Inertia::cylinder_x(20.0, 0.5, 50.0),
            ),
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
