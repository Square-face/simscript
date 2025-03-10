use bevy::{
    app::{Plugin, Startup, Update},
    input::common_conditions::input_just_pressed,
    prelude::{IntoSystemConfigs, KeyCode, ResMut},
    time::{Time, Virtual},
};

pub struct TimeControllPlugin;

impl Plugin for TimeControllPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, (start_paused,));
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

fn start_paused(mut time: ResMut<Time<Virtual>>) {
    time.set_relative_speed_f64(0.1);
    time.pause();
}
