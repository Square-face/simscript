use bevy::{
    app::{FixedFirst, Plugin, Startup, Update},
    input::common_conditions::{input_just_pressed, input_pressed},
    prelude::{IntoSystemConfigs, KeyCode, ResMut, Resource},
    time::{Fixed, Time, Virtual},
};

pub struct TimeControllPlugin;

#[derive(Resource, Default)]
struct StepFlag(bool);

const HZ: f64 = 30000.;
const TIMESCALE: f64 = 1.;

impl Plugin for TimeControllPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Time::<Fixed>::from_hz(HZ));
        app.insert_resource(StepFlag(false));
        app.add_systems(Startup, (start_paused,));
        app.add_systems(
            Update,
            (toggle_pause,).run_if(input_just_pressed(KeyCode::Space)),
        );
        app.add_systems(
            Update,
            (step_once_start,).run_if(input_pressed(KeyCode::ArrowRight)),
        );
        app.add_systems(FixedFirst, step_once_end);
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
    time.set_relative_speed_f64(TIMESCALE);
    time.pause();
}

fn step_once_start(mut time: ResMut<Time<Virtual>>, mut flag: ResMut<StepFlag>) {
    time.set_relative_speed_f64(60. / HZ);
    time.unpause();
    flag.0 = true;
}

fn step_once_end(mut time: ResMut<Time<Virtual>>, mut flag: ResMut<StepFlag>) {
    if flag.0 {
        time.set_relative_speed_f64(TIMESCALE);
        time.pause();
        flag.0 = false;
    }
}
