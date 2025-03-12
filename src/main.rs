use std::f64::consts::{PI, TAU};

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
    inertia_mass::{Inertia, InnertiaMass, Mass}, momentum::{AngMom, LinMom}, panels::Panel, transform::{Rotation, Translation}, State
};
use ui::{
    camera::{CameraPlugin, CameraTarget},
    grid::GridPlugin,
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
        //.add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(TimeControllPlugin)
        .add_systems(Startup, (spawn_tests,))
        .run();
}

fn spawn_tests(mut commands: Commands, ass: Res<AssetServer>) {
    let arrow = ass.load("arrow.glb#Scene0");
    let state = State::new(
        InnertiaMass::new(Mass::new(80.), Inertia::cylinder_x(14., 0.2, 80.)),
        simscript_physics::transform::Transform::new(Translation::ZERO, Rotation::ZERO),
        simscript_physics::momentum::Momentum::new(LinMom::new(DVec3::NEG_X * 1000.), AngMom::new(DVec3::Y * 10.)),
    );

    let normals: Vec<DVec3> = (0..3).map(|i| DQuat::from_rotation_x(TAU/3. * i as f64).mul_vec3(DVec3::Y)).collect();
    dbg!(&normals);

    let back = DVec3::NEG_X * 7.3;
    fn rot_90(vec: DVec3) -> DVec3 {
        DQuat::from_rotation_x(PI/2.).mul_vec3(vec)
    }


    let panels = vec![
        Panel::new(back + normals[0]*0.2, rot_90(normals[0]), 0.5),
        Panel::new(back + normals[1]*0.2, rot_90(normals[1]), 0.5),
        Panel::new(back + normals[2]*0.2, rot_90(normals[2]), 0.5),
    ];

    commands
        .spawn((SimulationBundle::new(state, panels), CameraTarget))
        .with_children(|parent| {
            parent.spawn((SceneRoot(arrow.clone()), Transform::from_xyz(0., 0.14, 0.).with_scale(Vec3::ONE.with_x(-1.))));
        });

    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 100.0,
    });

    commands.insert_resource(ClearColor(BLACK.into()));
}
