use bevy::{
    math::{DQuat, DVec3},
    prelude::Resource,
};
use clap::Parser;
use relative_path::{RelativePath, RelativePathBuf};
use serde::{Deserialize, Serialize};
use simscript_physics::{
    inertia_mass::{Inertia, InertiaMass, Mass},
    momentum::Momentum,
    transform::Transform,
    velocity::Velocity,
};
use std::{
    env::current_dir,
    fs::File,
    io::Read,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

/// Simulate simple Newtonian physics
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to config file for the initial simulation state
    #[arg(required = true)]
    pub config: RelativePathBuf,

    /// Override the starting timescale used by the simulation
    #[arg(long, short)]
    pub timescale: Option<f64>,

    /// Overrides the frequency the simulation will run at.
    ///
    /// Note that the frequency only affects "virtual" time.
    #[arg(long, short)]
    pub frequency: Option<f64>,
}

impl Cli {
    pub fn get_config(&self) -> Config {
        Config::serialize(&self.config)
    }

    pub fn override_config(&self, config: &mut Config) {
        if let Some(timescale) = self.timescale {
            config.settings.timescale = timescale;
        }
        if let Some(frequency) = self.frequency {
            config.settings.frequency = frequency;
        }
    }
}

impl Config {
    pub fn serialize(path: &RelativePath) -> Self {
        let current_dir = current_dir().expect("Failed to get current directory");
        let config_path = &path.to_path(current_dir);
        let config_dir = config_path
            .parent()
            .expect("Unable to get config parent directory");

        let mut file = File::open(config_path).expect("Failed to open config file");
        let size = file.metadata().map(|m| m.size()).unwrap_or(0) as usize;

        let mut buf = String::with_capacity(size);

        file.read_to_string(&mut buf)
            .expect("Failed to read config file");

        let mut config: Config = toml::from_str(&buf).expect("Failed to deserialize");

        config.fix_paths(config_dir);
        config
    }

    fn fix_paths(&mut self, base: &Path) {
        for entity in self.enteties.iter_mut() {
            let path_buf = &entity.sprite.path;

            entity.sprite.path = RelativePathBuf::from_path(path_buf)
                .expect("Invalid sprite path")
                .to_path(base);
        }
    }
}

impl InertiaShapes {
    pub const fn get_mass(&self) -> Mass {
        let mass = match self {
            InertiaShapes::CylinderX {
                radius: _,
                height: _,
                mass,
            } => mass,
            InertiaShapes::CylinderY {
                radius: _,
                height: _,
                mass,
            } => mass,
            InertiaShapes::CylinderZ {
                radius: _,
                height: _,
                mass,
            } => mass,
        };

        Mass::new(*mass)
    }
    pub const fn get_inertia(&self) -> Inertia {
        match self {
            InertiaShapes::CylinderX {
                radius,
                height,
                mass,
            } => Inertia::cylinder_x(*height, *radius, *mass),
            InertiaShapes::CylinderY {
                radius,
                height,
                mass,
            } => Inertia::cylinder_y(*height, *radius, *mass),
            InertiaShapes::CylinderZ {
                radius,
                height,
                mass,
            } => Inertia::cylinder_z(*height, *radius, *mass),
        }
    }
    pub fn to_inertiamass(&self) -> InertiaMass {
        let inertia = self.get_inertia();
        let mass = self.get_mass();

        InertiaMass::new(mass, inertia)
    }
}

impl TransformConfig {
    pub fn to_transform(&self) -> Transform {
        let rot = DQuat::from_euler(
            bevy::math::EulerRot::YZX,
            self.angular.y.to_radians(),
            self.angular.x.to_radians(),
            self.angular.z.to_radians(),
        );

        Transform::from_inner(self.linear, rot)
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

    #[serde(default)]
    pub transform: TransformConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    #[serde(default)]
    pub primary: bool,

    pub sprite: Sprite,
    pub inertia: InertiaShapes,

    #[serde(default)]
    pub momentum: Momentum,

    #[serde(default)]
    pub velocity: Velocity,

    #[serde(default)]
    pub transform: TransformConfig,

    #[serde(default)]
    pub panels: Vec<PanelConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct Config {
    pub settings: Settings,

    #[serde(default)]
    pub enteties: Vec<Entity>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TransformConfig {
    pub linear: DVec3,
    pub angular: DVec3,

    #[serde(default)]
    pub scale: DVec3,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PanelConfig {
    pub offset: DVec3,
    pub normal: DVec3,
    pub area: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InertiaShapes {
    CylinderX { radius: f64, height: f64, mass: f64 },
    CylinderY { radius: f64, height: f64, mass: f64 },
    CylinderZ { radius: f64, height: f64, mass: f64 },
}
