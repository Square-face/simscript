use std::path::PathBuf;

use clap::Parser;

/// Simulate simple Newtonian physics
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to config file for the initial simulation state
    #[arg(required = true)]
    pub config: PathBuf,
}

impl Args {
    pub fn get() -> Self {
        Args::parse()
    }
}
