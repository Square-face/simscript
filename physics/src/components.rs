use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::{Quat, Vec3},
    prelude::SpatialBundle,
};

#[derive(Bundle)]
pub struct SimulationBundle {
    pub spatial: SpatialBundle,
    pub sim: Simulated,
    pub vel: Velocity,
    pub acc: Accelerator,
}

impl SimulationBundle {
    pub fn new(vel: Velocity, acc: Accelerator) -> Self {
        Self {
            spatial: SpatialBundle::default(),
            sim: Simulated,
            vel,
            acc,
        }
    }
    pub fn new_with_gravity(vel: Velocity) -> Self {
        Self {
            spatial: SpatialBundle::default(),
            sim: Simulated,
            vel,
            acc: Accelerator::gravity(),
        }
    }
}

#[derive(Component, Debug)]
pub struct Inertia(pub Vec3);

/// Marker that designates entites to be simulated
///
/// Remove to easily stop something from being simulated.
#[derive(Component, Debug)]
pub struct Simulated;

/// Stores the current translational Velocity
///
/// The velocity is represented as a Vec3 in global cordinates
#[derive(Component, Debug)]
pub struct Velocity(pub Vec3);

/// Applies a constant acceleration
///
/// Works similar to [Velocity] in that the acceleration is represented as a Vec3 in global
/// cordinates
#[derive(Component, Debug)]
pub struct Accelerator(pub Vec3);

impl Accelerator {
    /// Returns a accelerator that simulates gravity.
    /// i.e a negative Y acceleration of 9.82 m/s^2
    ///
    /// ```
    /// # use bevy::prelude::Vec3;
    /// # use physics::components::Accelerator;
    /// let acc = Accelerator::gravity();
    ///
    /// assert_eq!(acc.0, Vec3{x:0.0, y:-9.82, z:0.0});
    /// ```
    pub fn gravity() -> Self {
        Self(Vec3::NEG_Y * 9.82)
    }
}

impl Velocity {
    /// Computes the angle from the horizontal plane to the velocity vector
    ///
    /// ```
    /// # use physics::components::Velocity;
    /// # use bevy::prelude::Vec3;
    /// let vel = Velocity(Vec3{x:1.0, y:1.0, z:0.0});
    ///
    /// assert_eq!(vel.pitch(), 45f32.to_radians());
    /// ```
    pub fn pitch(&self) -> f32 {
        let vec = self.0;
        let fdist = (vec.x.powi(2) + vec.z.powi(2)).sqrt();
        (vec.y / fdist).atan()
    }

    /// Computes the horizontal angle from the x axis to the velocity vector
    ///
    /// ```
    /// # use physics::components::Velocity;
    /// # use bevy::prelude::Vec3;
    /// let vel = Velocity(Vec3{x:1.0, y:0.0, z:-1.0});
    ///
    /// assert_eq!(vel.yaw(), 45f32.to_radians());
    /// ```
    pub fn yaw(&self) -> f32 {
        let vec = self.0;
        -vec.z.atan2(vec.x)
    }

    /// Returns a Quat representing the orientation of the vector.
    ///
    /// ```
    /// # use physics::components::Velocity;
    /// # use std::f32::consts::PI;
    /// # use bevy::math::Vec3;
    /// # use bevy::math::Quat;
    /// let vel = Velocity(Vec3{x:1.0, y:0.0, z:1.0});
    ///
    /// assert_eq!(
    ///     vel.to_direction(),
    ///     Quat::from_rotation_y(-PI/4.0)
    /// );
    /// ```
    pub fn to_direction(&self) -> Quat {
        Quat::from_euler(bevy::math::EulerRot::YXZ, self.yaw(), 0.0, self.pitch())
    }
}

#[cfg(test)]
mod velocity {
    use std::f32::consts::PI;

    use bevy::math::{Quat, Vec3};
    use float_cmp::assert_approx_eq;

    use crate::components::Velocity;

    #[test]
    fn to_direction() {
        let x = Velocity(Vec3::X).to_direction().to_array();
        let y = Velocity(Vec3::Y).to_direction().to_array();
        let z = Velocity(Vec3::Z).to_direction().to_array();

        let ang45 = Velocity(Vec3 {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        })
        .to_direction()
        .to_array();

        assert_approx_eq!(&[f32], &x, &Quat::default().to_array());
        assert_approx_eq!(&[f32], &y, &Quat::from_rotation_z(PI / 2.0).to_array());
        assert_approx_eq!(&[f32], &z, &Quat::from_rotation_y(-PI / 2.0).to_array());
        assert_approx_eq!(&[f32], &ang45, &Quat::from_rotation_z(PI / 4.0).to_array());
    }

    #[test]
    fn pitch() {
        let x = Velocity(Vec3::X);
        let y = Velocity(Vec3::Y);
        let z = Velocity(Vec3::Z);

        assert_approx_eq!(f32, x.pitch(), 0.0);
        assert_approx_eq!(f32, y.pitch(), PI / 2.0);
        assert_approx_eq!(f32, z.pitch(), 0.0);
    }

    #[test]
    fn yaw() {
        let x = Velocity(Vec3::X);
        let y = Velocity(Vec3::Y);
        let z = Velocity(Vec3::Z);

        let nx = Velocity(Vec3::NEG_X);
        let ny = Velocity(Vec3::NEG_Y);
        let nz = Velocity(Vec3::NEG_Z);

        assert_approx_eq!(f32, x.yaw(), 0.0);
        assert_approx_eq!(f32, y.yaw(), 0.0);
        assert_approx_eq!(f32, z.yaw(), -PI / 2.0);

        assert_approx_eq!(f32, nx.yaw(), -PI);
        assert_approx_eq!(f32, ny.yaw(), 0.0);
        assert_approx_eq!(f32, nz.yaw(), PI / 2.0);
    }
}
