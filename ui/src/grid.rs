use bevy::{
    app::{App, Startup, Update}, color::Color, gizmos, math::Vec3, prelude::{Commands, Gizmos}
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridSettings};

pub fn GridPlugin(app: &mut App) {
    app.add_plugins(bevy_infinite_grid::InfiniteGridPlugin);
    app.add_systems(Startup, (create_grid,));
    app.add_systems(Update, cardinal);
}

fn cardinal(mut gizmos: Gizmos) {
    gizmos.arrow(Vec3::ZERO, Vec3::X, Color::srgb(1.0, 0.0, 0.0));
    gizmos.arrow(Vec3::ZERO, Vec3::Y, Color::srgb(0.0, 1.0, 0.0));
    gizmos.arrow(Vec3::ZERO, Vec3::Z, Color::srgb(0.0, 0.0, 1.0));
}

fn create_grid(mut commands: Commands) {
    commands.spawn(InfiniteGridBundle{
        settings: InfiniteGridSettings{
            fadeout_distance: 10000.,
            scale: 0.1,
            ..Default::default()
        },
        ..Default::default()
    });
}
