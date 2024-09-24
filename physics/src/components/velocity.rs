use bevy::{ecs::component::Component, math::{Quat, Vec3}};

use super::acceleration::Accelerator;


/// Stores the current translational Velocity
///
/// The velocity is represented as a Vec3 in global cordinates
#[derive(Component, Debug)]
pub struct Velocity(pub Vec3);

/// Stores the current angular Velocity
///
/// The velocity is represented as a Vec3 in global cordinates
#[derive(Component, Debug)]
pub struct AngularVelocity(pub Vec3);


impl Velocity {
    /// Accelerates this velocity based on a time duration
    ///
    /// Remember to do half the acceleration before applying transformation and half
    /// after
    ///
    /// ```rust
    /// # use physics::components::acceleration::Accelerator;
    /// # use physics::components::velocity::Velocity;
    /// # use bevy::math::Vec3;
    ///
    /// let mut v = Velocity(Vec3::ZERO);
    /// let a = Accelerator(Vec3::ONE);
    /// v.accelerate(&a, 5.0);
    ///
    /// assert_eq!(v.0, Vec3::ONE * 5.0);
    ///
    /// ```
    pub fn accelerate(&mut self, acc: &Accelerator, delta: f32) {
        self.0 += acc.0 * delta
    }

    /// Returns a Quat representing the orientation of the vector.
    ///
    /// ```
    /// # use physics::components::velocity::Velocity;
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

impl Velocity {
    /// Computes the angle from the horizontal plane to the velocity vector
    fn pitch(&self) -> f32 {
        let vec = self.0;
        let fdist = (vec.x.powi(2) + vec.z.powi(2)).sqrt();
        (vec.y / fdist).atan()
    }

    /// Computes the horizontal angle from the x axis to the velocity vector
    fn yaw(&self) -> f32 {
        let vec = self.0;
        -vec.z.atan2(vec.x)
    }
}

#[cfg(test)]
mod linear_velocity {
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
