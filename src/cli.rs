use bevy::prelude::Resource;
use clap::Parser;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use std::{
    env::current_dir,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

/// Simulate simple Newtonian physics
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to config file for the initial simulation state
    #[arg(required = true)]
    pub config: RelativePathBuf,
}

impl Args {
    pub fn get_config() -> Config {
        let args = Args::parse();
        let path = args
            .config
            .to_path(current_dir().expect("Failed to get current directory"));
        Config::serialize(&path)
    }
}

impl Config {
    pub fn serialize(path: &Path) -> Self {
        let mut file = File::open(path).expect("Failed to open config file");
        let mut buf = String::new();

        file.read_to_string(&mut buf)
            .expect("Failed to read config file");

        let base = path.parent().unwrap();
        let mut config: Config = toml::from_str(&buf).expect("Failed to deserialize");

        for entity in config.enteties.iter_mut() {
            entity.sprite.path = RelativePathBuf::from_path(entity.sprite.path.clone())
                .expect("Invalid sprite path")
                .to_path(base);
        }

        config
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub frequency: f64,

    #[serde(default)]
    pub timescale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprite {
    pub path: PathBuf,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    #[serde(default)]
    pub primary: bool,
    pub sprite: Sprite,
}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct Config {
    pub settings: Settings,

    #[serde(default)]
    pub enteties: Vec<Entity>,
}
