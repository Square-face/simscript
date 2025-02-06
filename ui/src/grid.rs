use bevy::{
    app::{App, Startup},
    prelude::Commands,
};
use bevy_infinite_grid::InfiniteGridBundle;

pub fn GridPlugin(app: &mut App) {
    app.add_plugins(bevy_infinite_grid::InfiniteGridPlugin);
    app.add_systems(Startup, (create_grid,));
}

fn create_grid(mut commands: Commands) {
    commands.spawn(InfiniteGridBundle::default());
}
