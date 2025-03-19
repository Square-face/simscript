use bevy::{
    app::{App, Startup},
    asset::AssetServer,
    color::palettes::css::{BLACK, WHITE},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    log::LogPlugin,
    math::{DQuat, DVec3, Vec3},
    pbr::AmbientLight,
    prelude::{ChildBuild, PluginGroup, Transform},
    render::camera::ClearColor,
    scene::SceneRoot,
    window::{PresentMode, Window, WindowPlugin},
    DefaultPlugins,
};
use entity::{SimulationBundle, SimulationPlugin};
use simscript_physics::{
    inertia_mass::{Inertia, InertiaMass, Mass},
    momentum::{AngMom, LinMom, Momentum},
    panels::Panel,
};
use std::f64::consts::{PI, TAU};
use ui::{
    camera::{CameraPlugin, CameraTarget},
    grid::grid_plugin,
    time::TimeControllPlugin,
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
        .add_plugins(grid_plugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(TimeControllPlugin)
        .add_systems(Startup, (spawn_tests,))
        .run();
}

fn panels() -> Vec<Panel> {
    let normals: Vec<DVec3> = (0..3)
        .map(|i| DQuat::from_rotation_x(TAU / 3. * i as f64).mul_vec3(DVec3::Y))
        .collect();

    let back = DVec3::NEG_X * 0.71 / 2.;
    fn rot_90(vec: DVec3) -> DVec3 {
        DQuat::from_rotation_x(PI / 2.).mul_vec3(vec)
    }

    vec![
        Panel::new(back + normals[0] * 0.6 / 100., rot_90(normals[0]), 0.001),
        Panel::new(back + normals[1] * 0.6 / 100., rot_90(normals[1]), 0.001),
        Panel::new(back + normals[2] * 0.6 / 100., rot_90(normals[2]), 0.001),
    ]
}

fn spawn_tests(mut commands: Commands, ass: Res<AssetServer>) {
    let arrow = ass.load("arrow.glb#Scene0");

    let mass = InertiaMass::new(
        Mass::new(0.023),
        Inertia::cylinder_x(0.71, 0.3 / 100., 0.023),
    );
    let mom = Momentum::new(LinMom::Z * 0.023 * 050., AngMom::X * 0.0001);

    let state = simscript_physics::StateBuilder::new()
        .mass(mass)
        .momentum(mom)
        .panels(panels())
        .build();

    commands
        .spawn((SimulationBundle::new(state), CameraTarget))
        .with_children(|parent| {
            parent.spawn((
                SceneRoot(arrow.clone()),
                Transform::from_xyz(0., 0.14 / 14., 0.)
                    .with_scale(Vec3::new(-1., 1., 1.) * (0.71 / 14.)),
            ));
        });

    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 100.0,
    });

    commands.insert_resource(ClearColor(BLACK.into()));
}
