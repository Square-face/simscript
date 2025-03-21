use std::{path::Path, str::FromStr};

use bevy::{
    app::{App, Startup},
    asset::{AssetPath, AssetServer},
    color::palettes::css::{BLACK, WHITE},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::system::Commands,
    log::LogPlugin,
    pbr::AmbientLight,
    prelude::{PluginGroup, Res},
    render::camera::ClearColor,
    scene::SceneRoot,
    window::{PresentMode, Window, WindowPlugin},
    DefaultPlugins,
};
use cli::Config;
use entity::SimulationPlugin;
use ui::{
    camera::{CameraPlugin, CameraTarget},
    grid::grid_plugin,
    time::TimeControllPlugin,
};

mod cli;
mod entity;

fn main() {
    let config = cli::Args::get_config();

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
        .add_systems(Startup, (setup_environment, spaw_config))
        .insert_resource(config)
        .run();
}

fn spaw_config(mut commands: Commands, ass: Res<AssetServer>, config: Res<Config>) {
    for entity in config.enteties.iter() {
        let label = String::from_str(&entity.sprite.label).unwrap();
        let path = String::from_str(entity.sprite.path.to_str().unwrap()).unwrap();

        let label: &'static str = Box::leak(label.into_boxed_str());
        let path: &'static str = Box::leak(path.into_boxed_str());

        let path = Path::new(path);
        let asset_path = AssetPath::from_path(path).with_label(label);

        let cube = ass.load(asset_path);

        if entity.primary {
            commands.spawn((SceneRoot(cube), CameraTarget));
        } else {
            commands.spawn(SceneRoot(cube));
        }
    }
}

fn setup_environment(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 100.0,
    });

    commands.insert_resource(ClearColor(BLACK.into()));
}
