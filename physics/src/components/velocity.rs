extern crate overload;
use overload::overload;
use std::{marker::PhantomData, ops};

use bevy::{
    ecs::component::Component,
    math::{Quat, Vec3},
};

use crate::cordinate_systems::{CoordinateConvert, CoordinateSystem, Global, Local};

/// Stores the current translational Velocity
///
/// The velocity is represented as a Vec3 in global cordinates
#[derive(Component, Debug)]
pub struct Velocity<S: CoordinateSystem>(pub Vec3, PhantomData<S>);

/// Stores the current angular Velocity
///
/// The velocity is represented as a Vec3 in global cordinates
#[derive(Component, Debug)]
pub struct AngularVelocity<S: CoordinateSystem>(pub Vec3, PhantomData<S>);

impl<S: CoordinateSystem> Velocity<S> {
    /// [Velocity] of zero in every direction
    pub const ZERO: Self = Self::new(Vec3::ZERO);

    #[inline]
    #[must_use]
    pub const fn new(vel: Vec3) -> Velocity<S> {
        Velocity(vel, PhantomData)
    }
}

impl Velocity<Global> {
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
    #[inline]
    #[must_use]
    pub fn to_direction(&self) -> Quat {
        Quat::from_euler(bevy::math::EulerRot::YXZ, self.yaw(), 0.0, self.pitch())
    }
}

impl<S: CoordinateSystem> AngularVelocity<S> {
    /// [AngularVelocity] of zero in every direction
    pub const ZERO: Self = Self::new(Vec3::ZERO);

    #[inline]
    #[must_use]
    pub const fn new(vel: Vec3) -> AngularVelocity<S> {
        AngularVelocity(vel, PhantomData)
    }
}

impl Velocity<Global> {
    /// Computes the angle from the horizontal plane to the velocity vector
    #[must_use]
    fn pitch(&self) -> f32 {
        let vec = self.0;
        let fdist = (vec.x.powi(2) + vec.z.powi(2)).sqrt();
        (vec.y / fdist).atan()
    }

    /// Computes the horizontal angle from the x axis to the velocity vector
    #[must_use]
    fn yaw(&self) -> f32 {
        let vec = self.0;
        -vec.z.atan2(vec.x)
    }
}

impl<S: CoordinateSystem> CoordinateConvert for Velocity<S> {
    type Global = Velocity<Global>;

    type Local = Velocity<Local>;

    #[inline]
    fn to_global(self, rot: Quat) -> Self::Global {
        Velocity::<Global>::new(S::vec3_to_global(self.0, rot))
    }

    #[inline]
    fn to_local(self, rot: Quat) -> Self::Local {
        Velocity::<Local>::new(S::vec3_to_local(self.0, rot))
    }
}

impl<S: CoordinateSystem> CoordinateConvert for AngularVelocity<S> {
    type Global = AngularVelocity<Global>;

    type Local = AngularVelocity<Local>;

    #[inline]
    fn to_global(self, rot: Quat) -> Self::Global {
        AngularVelocity::<Global>::new(S::vec3_to_global(self.0, rot))
    }

    #[inline]
    fn to_local(self, rot: Quat) -> Self::Local {
        AngularVelocity::<Local>::new(S::vec3_to_local(self.0, rot))
    }
}

// ==== Local Coordinate System ====
// impl x for Velocity<Local>
overload!((a: ?Velocity<Local>) + (b: ?Velocity<Local>) -> Velocity<Local> { Velocity::new(a.0 + b.0) });
overload!((a: ?Velocity<Local>) - (b: ?Velocity<Local>) -> Velocity<Local> { Velocity::new(a.0 - b.0) });
overload!((a: ?Velocity<Local>) * (b: ?Velocity<Local>) -> Velocity<Local> { Velocity::new(a.0 * b.0) });
overload!((a: ?Velocity<Local>) / (b: ?Velocity<Local>) -> Velocity<Local> { Velocity::new(a.0 / b.0) });

overload!((a: ?Velocity<Local>) * (b: f32) -> Velocity<Local> { Velocity::new(a.0 * b) });
overload!((a: ?Velocity<Local>) / (b: f32) -> Velocity<Local> { Velocity::new(a.0 / b) });

overload!(- (a: &mut Velocity<Local>) -> Velocity<Local> { Velocity::new(- a.0) });

// impl xAssign for Velocity<Local>
overload!((a: &mut Velocity<Local>) += (b: ?Velocity<Local>) { a.0 += b.0 });
overload!((a: &mut Velocity<Local>) -= (b: ?Velocity<Local>) { a.0 -= b.0 });
overload!((a: &mut Velocity<Local>) *= (b: ?Velocity<Local>) { a.0 *= b.0 });
overload!((a: &mut Velocity<Local>) /= (b: ?Velocity<Local>) { a.0 /= b.0 });

overload!((a: &mut Velocity<Local>) *= (b: f32) {a.0 *= b});
overload!((a: &mut Velocity<Local>) /= (b: f32) {a.0 /= b});

// ==== Global Coordinate System ====
// impl x for Velocity<Global>
overload!((a: ?Velocity<Global>) + (b: ?Velocity<Global>) -> Velocity<Global> { Velocity::new(a.0 + b.0) });
overload!((a: ?Velocity<Global>) - (b: ?Velocity<Global>) -> Velocity<Global> { Velocity::new(a.0 - b.0) });
overload!((a: ?Velocity<Global>) * (b: ?Velocity<Global>) -> Velocity<Global> { Velocity::new(a.0 * b.0) });
overload!((a: ?Velocity<Global>) / (b: ?Velocity<Global>) -> Velocity<Global> { Velocity::new(a.0 / b.0) });

overload!((a: ?Velocity<Global>) * (b: f32) -> Velocity<Global> { Velocity::new(a.0 * b) });
overload!((a: ?Velocity<Global>) / (b: f32) -> Velocity<Global> { Velocity::new(a.0 / b) });

overload!(- (a: &mut Velocity<Global>) -> Velocity<Global> { Velocity::new(- a.0) });

// impl xAssign for Velocity<Global>
overload!((a: &mut Velocity<Global>) += (b: ?Velocity<Global>) { a.0 += b.0 });
overload!((a: &mut Velocity<Global>) -= (b: ?Velocity<Global>) { a.0 -= b.0 });
overload!((a: &mut Velocity<Global>) *= (b: ?Velocity<Global>) { a.0 *= b.0 });
overload!((a: &mut Velocity<Global>) /= (b: ?Velocity<Global>) { a.0 /= b.0 });

overload!((a: &mut Velocity<Global>) *= (b: f32) {a.0 *= b});
overload!((a: &mut Velocity<Global>) /= (b: f32) {a.0 /= b});

// ==== Local Coordinate System ====
// impl x for AngularVelocity<Local>
overload!((a: ?AngularVelocity<Local>) + (b: ?AngularVelocity<Local>) -> AngularVelocity<Local> { AngularVelocity::new(a.0 + b.0) });
overload!((a: ?AngularVelocity<Local>) - (b: ?AngularVelocity<Local>) -> AngularVelocity<Local> { AngularVelocity::new(a.0 - b.0) });
overload!((a: ?AngularVelocity<Local>) * (b: ?AngularVelocity<Local>) -> AngularVelocity<Local> { AngularVelocity::new(a.0 * b.0) });
overload!((a: ?AngularVelocity<Local>) / (b: ?AngularVelocity<Local>) -> AngularVelocity<Local> { AngularVelocity::new(a.0 / b.0) });

overload!((a: ?AngularVelocity<Local>) * (b: f32) -> AngularVelocity<Local> { AngularVelocity::new(a.0 * b) });
overload!((a: ?AngularVelocity<Local>) / (b: f32) -> AngularVelocity<Local> { AngularVelocity::new(a.0 / b) });

overload!(- (a: &mut AngularVelocity<Local>) -> AngularVelocity<Local> { AngularVelocity::new(- a.0) });

// impl xAssign for AngularVelocity<Local>
overload!((a: &mut AngularVelocity<Local>) += (b: ?AngularVelocity<Local>) { a.0 += b.0 });
overload!((a: &mut AngularVelocity<Local>) -= (b: ?AngularVelocity<Local>) { a.0 -= b.0 });
overload!((a: &mut AngularVelocity<Local>) *= (b: ?AngularVelocity<Local>) { a.0 *= b.0 });
overload!((a: &mut AngularVelocity<Local>) /= (b: ?AngularVelocity<Local>) { a.0 /= b.0 });

overload!((a: &mut AngularVelocity<Local>) *= (b: f32) {a.0 *= b});
overload!((a: &mut AngularVelocity<Local>) /= (b: f32) {a.0 /= b});

// ==== Global Coordinate System ====
// impl x for AngularVelocity<Global>
overload!((a: ?AngularVelocity<Global>) + (b: ?AngularVelocity<Global>) -> AngularVelocity<Global> { AngularVelocity::new(a.0 + b.0) });
overload!((a: ?AngularVelocity<Global>) - (b: ?AngularVelocity<Global>) -> AngularVelocity<Global> { AngularVelocity::new(a.0 - b.0) });
overload!((a: ?AngularVelocity<Global>) * (b: ?AngularVelocity<Global>) -> AngularVelocity<Global> { AngularVelocity::new(a.0 * b.0) });
overload!((a: ?AngularVelocity<Global>) / (b: ?AngularVelocity<Global>) -> AngularVelocity<Global> { AngularVelocity::new(a.0 / b.0) });

overload!((a: ?AngularVelocity<Global>) * (b: f32) -> AngularVelocity<Global> { AngularVelocity::new(a.0 * b) });
overload!((a: ?AngularVelocity<Global>) / (b: f32) -> AngularVelocity<Global> { AngularVelocity::new(a.0 / b) });

overload!(- (a: &mut AngularVelocity<Global>) -> AngularVelocity<Global> { AngularVelocity::new(- a.0) });

// impl xAssign for AngularVelocity<Global>
overload!((a: &mut AngularVelocity<Global>) += (b: ?AngularVelocity<Global>) { a.0 += b.0 });
overload!((a: &mut AngularVelocity<Global>) -= (b: ?AngularVelocity<Global>) { a.0 -= b.0 });
overload!((a: &mut AngularVelocity<Global>) *= (b: ?AngularVelocity<Global>) { a.0 *= b.0 });
overload!((a: &mut AngularVelocity<Global>) /= (b: ?AngularVelocity<Global>) { a.0 /= b.0 });

overload!((a: &mut AngularVelocity<Global>) *= (b: f32) {a.0 *= b});
overload!((a: &mut AngularVelocity<Global>) /= (b: f32) {a.0 /= b});

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
