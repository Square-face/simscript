use clap::Parser;

/// Simulate simple Newtonian physics
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {}

impl Args {
    pub fn get() -> Self {
        Args::parse()
    }
}
