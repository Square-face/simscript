use bevy::{
    app::{Plugin, Update},
    input::common_conditions::input_just_pressed,
    prelude::{IntoSystemConfigs, KeyCode, ResMut},
    time::{Time, Virtual},
};


pub struct TimeControllPlugin;

impl Plugin for TimeControllPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (toggle_pause,).run_if(input_just_pressed(KeyCode::Space)),
        );
    }
}

fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}
