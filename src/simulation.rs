use std::any::TypeId;

use bevy::{
    app::{App, FixedUpdate},
    prelude::{IntoSystemConfigs, Query, Res, Transform},
    time::Time,
};
use physics::{
    components::{acceleration::Acceleration, velocity::Velocity},
    coordinate_systems::{CoordinateConvert, CoordinateSystem, Global, Local},
};

const TIMESCALE: f32 = 1.0;

pub fn simulation_step(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            (
                velocity::<Local, Local>,
                velocity::<Local, Global>,
                velocity::<Global, Local>,
                velocity::<Global, Global>,
            ),
            (translation::<Local>, translation::<Global>),
            (
                velocity::<Local, Local>,
                velocity::<Local, Global>,
                velocity::<Global, Local>,
                velocity::<Global, Global>,
            ),
        )
            .chain(),
    );
}

pub fn velocity<A: CoordinateSystem, V: CoordinateSystem>(
    time: Res<Time>,
    mut query: Query<(&Acceleration<A>, &mut Velocity<V>, &Transform)>,
) {
    for (acc, mut vel, trans) in &mut query {
        let rot = trans.rotation;

        let acc = if TypeId::of::<V>() == TypeId::of::<Local>() {
            acc.to_local(rot).0
        } else {
            acc.to_global(rot).0
        };

        let delta = acc * 0.5f32 * TIMESCALE * time.delta_secs();
        vel.0 += delta;
    }
}

pub fn translation<S: CoordinateSystem>(
    time: Res<Time>,
    mut query: Query<(&Velocity<S>, &mut Transform)>,
) {
    for (vel, mut trans) in &mut query {
        let rot = trans.rotation;
        let vel = vel.to_global(rot).0;
        trans.translation += vel * time.delta_secs() * TIMESCALE;
    }
}
