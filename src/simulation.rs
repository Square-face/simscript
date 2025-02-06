use bevy::{
    app::{App, FixedUpdate},
    prelude::IntoSystemConfigs, time::{Fixed, Time, Virtual},
};

const TIMESCALE: f32 = 1.;

pub fn simulation_step(app: &mut App) {
    #[cfg(debug_assertions)]
    app.init_resource::<timer::PhysicsStart>();
    app.world_mut().resource_mut::<Time<Virtual>>().set_relative_speed(1.0);
    app.insert_resource(Time::<Fixed>::from_hz(1000.0));
    app.add_systems(
        FixedUpdate,
        (
            #[cfg(debug_assertions)]
            timer::start,
            (acceleration::linear, acceleration::angular),
            (velocity::linear, velocity::angular),
            (translation::linear, translation::angular),
            (velocity::linear, velocity::angular),
            #[cfg(debug_assertions)]
            timer::end,
        )
            .chain(),
    );
}

mod translation {
    use super::TIMESCALE;
    use bevy::{
        math::Quat,
        prelude::{Query, Res, Transform},
        time::Time,
    };
    use physics::{
        components::velocity::{AngularVelocity, Velocity},
        coordinate_systems::Global,
    };

    pub fn linear(time: Res<Time>, mut query: Query<(&Velocity<Global>, &mut Transform)>) {
        for (vel, mut trans) in &mut query {
            trans.translation += vel.0 * time.delta_secs() * TIMESCALE;
        }
    }

    pub fn angular(time: Res<Time>, mut query: Query<(&AngularVelocity<Global>, &mut Transform)>) {
        for (angvel, mut trans) in &mut query {
            let delta = time.delta_secs() * TIMESCALE;

            let delta_rot = Quat::from_vec4(
                (angvel.0 * delta / 2.0).extend(trans.rotation.w * delta / 2.0),
            );

            if delta_rot.w != 0.0 {
                trans.rotation =
                    (trans.rotation + delta_rot.normalize() * trans.rotation).normalize();
            }
        }
    }
}

mod velocity {
    use super::TIMESCALE;
    use bevy::{
        prelude::{Query, Res},
        time::Time,
    };
    use physics::{
        components::{
            acceleration::{Acceleration, AngularAcceleration},
            velocity::{AngularVelocity, Velocity},
        },
        coordinate_systems::Global,
    };

    pub fn linear(
        time: Res<Time>,
        mut query: Query<(&Acceleration<Global>, &mut Velocity<Global>)>,
    ) {
        for (acc, mut vel) in &mut query {
            let delta = acc * 0.5 * TIMESCALE * time.delta_secs();
            *vel += delta;
        }
    }

    pub fn angular(
        time: Res<Time>,
        mut query: Query<(&AngularAcceleration<Global>, &mut AngularVelocity<Global>)>,
    ) {
        for (acc, mut vel) in &mut query {
            let delta = acc * 0.5 * TIMESCALE * time.delta_secs();
            *vel += delta;
        }
    }
}

mod acceleration {
    use bevy::prelude::{Query, Transform};
    use physics::{
        components::{
            acceleration::{Acceleration, AngularAcceleration},
            force::Moment,
            inertia::Inertia,
        },
        coordinate_systems::{CoordinateConvert, Global, Local},
    };

    #[allow(clippy::type_complexity)]
    pub fn linear(
        mut query: Query<(
            &Moment<Global>,
            &Inertia<Local>,
            &Transform,
            &mut Acceleration<Global>,
        )>,
    ) {
        for (mom, inert, trans, mut acc) in &mut query {
            let force = mom.get_force();
            let rot = trans.rotation;
            *acc = inert
                .get_linear_acceleration(&force.to_local(rot))
                .to_global(rot);
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn angular(
        mut query: Query<(
            &Moment<Global>,
            &Inertia<Local>,
            &Transform,
            &mut AngularAcceleration<Global>,
        )>,
    ) {
        for (mom, inert, trans, mut acc) in &mut query {
            let torque = mom.get_torque();
            let rot = trans.rotation;
            *acc = inert
                .get_angular_acceleration(&torque.to_local(rot))
                .to_global(rot);
        }
    }
}

mod timer {
    use bevy::{
        log::debug,
        prelude::{Res, ResMut, Resource},
    };
    use chrono::TimeDelta;
    use std::time::Instant;

    #[derive(Debug, Resource)]
    pub struct PhysicsStart(Instant);

    impl Default for PhysicsStart {
        fn default() -> Self {
            Self(Instant::now())
        }
    }

    pub fn start(mut last: ResMut<PhysicsStart>) {
        last.0 = Instant::now();
    }

    pub fn end(last: Res<PhysicsStart>) {
        let duration = TimeDelta::from_std(Instant::now() - last.0).expect("Physics took to long");

        #[cfg(debug_assertions)]
        debug!(
            "Physics calculations took {}µs",
            duration.num_microseconds().expect("no microseconds")
        );
    }
}
