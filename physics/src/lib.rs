//! Library for handling physics calculations in SimScript
//!
//! This library provides structures for relevant physical quantities, such as
//! forces, acceleration, time, and more. The goal is to define a struct for
//! each of these quantities. Currently, only a few are implemented, and there
//! is no comprehensive list of all the quantities that need to be added.
//!
//! ## Local and Global Typestate
//!
//! Many quantities also include a typestate to differentiate between local and
//! global coordinate spaces. 
//! - **Local space**: Refers to a coordinate system that rotates with the object it is associated with.
//! - **Global space**: Refers to a fixed, world-aligned coordinate system.
//!
//! The purpose of this design is to simplify distinguishing between these two
//! types of coordinate systems without requiring runtime checks, such as those
//! involving enums. Additionally, we leverage Rust's type system to enforce
//! correctness at compile time.
//!
//! ## Operations on Quantities
//!
//! This library provides trait implementations that allow you to perform
//! arithmetic operations on physical quantities, producing the correct
//! resulting quantity. For example:
//! - Multiplying `Acceleration` with `Time` yields a `Velocity`.
//!
//! These operations are designed to align with the rules of physics and ensure
//! correctness at the type level. By encoding these relationships into the type
//! system, the library helps prevent invalid operations and provides intuitive
//! support for working with physical calculations.
use bevy::app::{Plugin, PostUpdate, Update};
use bevy::ecs::query::With;
use bevy::ecs::system::{Query, Res};
use bevy::math::{Quat, Vec3};
use bevy::time::Time;
use bevy::transform::components::Transform;

use components::acceleration::Acceleration;
use components::force::Moment;
use components::inertia::Inertia;
use coordinate_systems::Global;

pub mod components;
mod vector_arrows;
pub mod coordinate_systems;

pub struct SimulatiorPlugin;

impl Plugin for SimulatiorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_simulated);
        app.add_systems(Update, update_simulated);
        app.add_systems(
            PostUpdate,
            (vector_arrows::velocity::<Global>, vector_arrows::acceleration::<Global>),
        );
    }
}

/// Updates objects with acceleration
#[allow(clippy::type_complexity)]
pub fn update_simulated(
    time: Res<Time>,
    mut accelerators: Query<
        (
            &mut Transform,
            &mut components::velocity::Velocity<Global>,
            &mut components::velocity::AngularVelocity<Global>,
            &Inertia<Global>,
            Option<&Acceleration<Global>>,
        ),
        With<components::Simulated>,
    >,
) {
    let delta = time.delta_secs();
    let half_delta = delta / 2.0;

    for (mut trans, mut vel, mut angvel, inertia, _acc) in accelerators.iter_mut() {
        let (torque, force) = Moment::new(Vec3::Y, Vec3::new(0.0, 0.0, 0.0)).get_parts();
        let acc = inertia.get_linear_acceleration(&force);
        let angacc = inertia.get_angular_acceleration(&torque);

        // Accelerate and move
        *vel += &acc * half_delta;
        *angvel += &angacc * half_delta;

        trans.translation += vel.0 * delta;

        let delta_rot =
            Quat::from_vec4((angvel.0 * delta / 2.0).extend(trans.rotation.w * delta / 2.0));

        if delta_rot.w != 0.0 {
            trans.rotation = (trans.rotation + delta_rot.normalize() * trans.rotation).normalize();
        }

        *vel += &acc * half_delta;
        *angvel += &angacc * half_delta;
    }
}
