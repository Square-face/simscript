use std::{path::Path, str::FromStr};

use bevy::{
    app::{App, Startup},
    asset::{AssetPath, AssetServer, Handle},
    color::palettes::css::{BLACK, WHITE},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::system::Commands,
    log::LogPlugin,
    pbr::AmbientLight,
    prelude::{BuildChildren, PluginGroup, Res, Transform},
    render::camera::ClearColor,
    scene::{Scene, SceneRoot},
    window::{PresentMode, Window, WindowPlugin},
    DefaultPlugins,
};
use cli::{Config, Sprite};
use entity::{SimulationBundle, SimulationPlugin};
use simscript_physics::{panels::Panel, StateBuilder};
use ui::{
    camera::{CameraPlugin, CameraTarget},
    grid::grid_plugin,
    time::TimeControllPlugin,
};

mod cli;
mod entity;

fn main() {
    let config = cli::Args::get_config();
    dbg!(&config);

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
        .add_systems(Startup, (setup_environment, spawn_config))
        .insert_resource(config)
        .run();
}

fn load_dynamic_asset(sprite: &Sprite, ass: &Res<AssetServer>) -> Handle<Scene> {
    let label = String::from_str(&sprite.label).unwrap();
    let path = String::from_str(sprite.path.to_str().unwrap()).unwrap();

    let label: &'static str = Box::leak(label.into_boxed_str());
    let path: &'static str = Box::leak(path.into_boxed_str());

    let path = Path::new(path);
    let asset_path = AssetPath::from_path(path).with_label(label);
    ass.load(asset_path)
}

fn spawn_config(mut commands: Commands, ass: Res<AssetServer>, config: Res<Config>) {
    for entity in config.enteties.iter() {
        let panels: Vec<Panel> = entity
            .panels
            .iter()
            .map(|panel| Panel::new(panel.offset, panel.normal.normalize(), panel.area))
            .collect();

        let state = StateBuilder::new()
            .mass(entity.inertia.to_inertiamass())
            .momentum(entity.momentum)
            .transform(entity.transform.to_transform())
            .panels(panels)
            .build();

        let state_transform = entity.sprite.transform.to_transform();

        if entity.primary {
            commands.spawn((SimulationBundle::new(state), CameraTarget))
        } else {
            commands.spawn(SimulationBundle::new(state))
        }
        .with_child((
            Transform::from_translation(state_transform.translation.0.as_vec3())
                .with_rotation(state_transform.rotation.0.as_quat()),
            SceneRoot(load_dynamic_asset(&entity.sprite, &ass)),
        ));
    }
}

fn setup_environment(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 100.0,
    });

    commands.insert_resource(ClearColor(BLACK.into()));
}
