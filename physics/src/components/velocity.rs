extern crate overload;
use overload::overload;
use std::{marker::PhantomData, ops};

use bevy::{
    ecs::component::Component,
    math::{Quat, Vec3},
};

use crate::cordinate_systems::{CoordinateSystem, Global, Local};

/// Stores the current translational Velocity
///
/// The velocity is represented as a Vec3 in global cordinates
#[derive(Component, Debug)]
pub struct Velocity<S: CoordinateSystem>(pub Vec3, PhantomData<S>);

/// Stores the current angular Velocity
///
/// The velocity is represented as a Vec3 in global cordinates
#[derive(Component, Debug)]
pub struct AngularVelocity(pub Vec3);

impl<S: CoordinateSystem> Velocity<S> {
    pub fn new(vel: Vec3) -> Velocity<S> {
        Velocity(vel, PhantomData)
    }
}

impl Velocity<Global> {
    /// [Velocity] of zero in every direction
    pub const ZERO: Self = Self(Vec3::ZERO, PhantomData);

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

impl AngularVelocity {
    /// [AngularVelocity] of zero in every direction
    pub const ZERO: Self = Self(Vec3::ZERO);
}

impl Velocity<Global> {
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

overload!((a: ?Velocity<Local>) + (b: ?Velocity<Local>) -> Velocity<Local> { Velocity(a.0 + b.0, PhantomData) });
overload!((a: ?Velocity<Local>) - (b: ?Velocity<Local>) -> Velocity<Local> { Velocity(a.0 - b.0, PhantomData) });
overload!((a: ?Velocity<Local>) * (b: ?Velocity<Local>) -> Velocity<Local> { Velocity(a.0 * b.0, PhantomData) });
overload!((a: ?Velocity<Local>) / (b: ?Velocity<Local>) -> Velocity<Local> { Velocity(a.0 / b.0, PhantomData) });
overload!((a: ?Velocity<Local>) % (b: ?Velocity<Local>) -> Velocity<Local> { Velocity(a.0 % b.0, PhantomData) });

overload!((a: &mut Velocity<Local>) += (b: ?Velocity<Local>) { a.0 += b.0 });
overload!((a: &mut Velocity<Local>) -= (b: ?Velocity<Local>) { a.0 -= b.0 });
overload!((a: &mut Velocity<Local>) *= (b: ?Velocity<Local>) { a.0 *= b.0 });
overload!((a: &mut Velocity<Local>) /= (b: ?Velocity<Local>) { a.0 /= b.0 });
overload!((a: &mut Velocity<Local>) %= (b: ?Velocity<Local>) { a.0 %= b.0 });

overload!(-(a: ?Velocity<Local>) -> Velocity<Local> { Velocity(-a.0, PhantomData) });

overload!((a: ?Velocity<Global>) + (b: ?Velocity<Global>) -> Velocity<Global> { Velocity(a.0 + b.0, PhantomData) });
overload!((a: ?Velocity<Global>) - (b: ?Velocity<Global>) -> Velocity<Global> { Velocity(a.0 - b.0, PhantomData) });
overload!((a: ?Velocity<Global>) * (b: ?Velocity<Global>) -> Velocity<Global> { Velocity(a.0 * b.0, PhantomData) });
overload!((a: ?Velocity<Global>) / (b: ?Velocity<Global>) -> Velocity<Global> { Velocity(a.0 / b.0, PhantomData) });
overload!((a: ?Velocity<Global>) % (b: ?Velocity<Global>) -> Velocity<Global> { Velocity(a.0 % b.0, PhantomData) });

overload!((a: &mut Velocity<Global>) += (b: ?Velocity<Global>) { a.0 += b.0 });
overload!((a: &mut Velocity<Global>) -= (b: ?Velocity<Global>) { a.0 -= b.0 });
overload!((a: &mut Velocity<Global>) *= (b: ?Velocity<Global>) { a.0 *= b.0 });
overload!((a: &mut Velocity<Global>) /= (b: ?Velocity<Global>) { a.0 /= b.0 });
overload!((a: &mut Velocity<Global>) %= (b: ?Velocity<Global>) { a.0 %= b.0 });

overload!(-(a: ?Velocity<Global>) -> Velocity<Global> { Velocity(-a.0, PhantomData) });

overload!((a: ?AngularVelocity) + (b: ?AngularVelocity) -> AngularVelocity { AngularVelocity(a.0 + b.0) });
overload!((a: ?AngularVelocity) - (b: ?AngularVelocity) -> AngularVelocity { AngularVelocity(a.0 - b.0) });
overload!((a: ?AngularVelocity) * (b: ?AngularVelocity) -> AngularVelocity { AngularVelocity(a.0 * b.0) });
overload!((a: ?AngularVelocity) / (b: ?AngularVelocity) -> AngularVelocity { AngularVelocity(a.0 / b.0) });
overload!((a: ?AngularVelocity) % (b: ?AngularVelocity) -> AngularVelocity { AngularVelocity(a.0 % b.0) });

overload!((a: &mut AngularVelocity) += (b: ?AngularVelocity) { a.0 += b.0 });
overload!((a: &mut AngularVelocity) -= (b: ?AngularVelocity) { a.0 -= b.0 });
overload!((a: &mut AngularVelocity) *= (b: ?AngularVelocity) { a.0 *= b.0 });
overload!((a: &mut AngularVelocity) /= (b: ?AngularVelocity) { a.0 /= b.0 });
overload!((a: &mut AngularVelocity) %= (b: ?AngularVelocity) { a.0 %= b.0 });

overload!(-(a: ?AngularVelocity) -> AngularVelocity { AngularVelocity(-a.0) });

#[cfg(test)]
mod linear_velocity {
    use std::f32::consts::PI;

    use bevy::math::{Quat, Vec3};
    use float_cmp::assert_approx_eq;

    use crate::components::Velocity;

    #[test]
    fn to_direction() {
        let x = Velocity::new(Vec3::X).to_direction().to_array();
        let y = Velocity::new(Vec3::Y).to_direction().to_array();
        let z = Velocity::new(Vec3::Z).to_direction().to_array();

        let ang45 = Velocity::new(Vec3 {
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
        let x = Velocity::new(Vec3::X);
        let y = Velocity::new(Vec3::Y);
        let z = Velocity::new(Vec3::Z);

        assert_approx_eq!(f32, x.pitch(), 0.0);
        assert_approx_eq!(f32, y.pitch(), PI / 2.0);
        assert_approx_eq!(f32, z.pitch(), 0.0);
    }

    #[test]
    fn yaw() {
        let x = Velocity::new(Vec3::X);
        let y = Velocity::new(Vec3::Y);
        let z = Velocity::new(Vec3::Z);

        let nx = Velocity::new(Vec3::NEG_X);
        let ny = Velocity::new(Vec3::NEG_Y);
        let nz = Velocity::new(Vec3::NEG_Z);

        assert_approx_eq!(f32, x.yaw(), 0.0);
        assert_approx_eq!(f32, y.yaw(), 0.0);
        assert_approx_eq!(f32, z.yaw(), -PI / 2.0);

        assert_approx_eq!(f32, nx.yaw(), -PI);
        assert_approx_eq!(f32, ny.yaw(), 0.0);
        assert_approx_eq!(f32, nz.yaw(), PI / 2.0);
    }
}
