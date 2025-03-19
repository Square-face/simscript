use std::{fs::File, io::Read, path::PathBuf};

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use simscript_physics::State;

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct Settings {}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct Sprite {}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct Entity {
    pub camera_target: bool,
    pub state: State,
    pub sprite: Sprite,
}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct Config {
    pub settings: Settings,
    pub entities: Vec<Entity>,
}

impl Config {
    pub fn serialize(path: PathBuf) -> Self {
        let mut file = File::open(path).expect("failed to open config file");
        let mut buf = String::new();

        file.read_to_string(&mut buf)
            .expect("Failed to read config file");

        toml::from_str(&buf).expect("failed to deserialize")
    }
}
