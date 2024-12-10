extern crate overload;
use overload::overload;
use std::{marker::PhantomData, ops};

use bevy::{ecs::component::Component, math::Vec3};

use crate::cordinate_systems::{CoordinateConvert, CoordinateSystem, Global, Local};

use super::velocity::{AngularVelocity, Velocity};

/// Represents a linear acceleration
///
/// Ment to be used together with [Velocity]
///
/// Values are in `m/s^2`
#[derive(Component, Debug, PartialEq)]
pub struct Acceleration<S: CoordinateSystem>(pub Vec3, PhantomData<S>);

/// Represents a rotational acceleration
///
/// Meant to be used together with [AngularVelocity]
///
/// Values are in `rad/s^2`
#[derive(Component, Debug, PartialEq)]
pub struct AngularAcceleration<S: CoordinateSystem>(pub Vec3, PhantomData<S>);

impl<S: CoordinateSystem> Acceleration<S> {
    /// [Acceleration] that doesn't accelerate in any direction
    pub const ZERO: Self = Self::new(Vec3::ZERO);

    /// [Acceleration] that mimics gravity (-9.82 m/s^2 in y velocity)
    pub const GRAVITY: Self = Self::new(Vec3::new(0.0, -9.82, 0.0));

    pub const fn new(acc: Vec3) -> Acceleration<S> {
        Acceleration(acc, PhantomData)
    }
}

impl<S: CoordinateSystem> AngularAcceleration<S> {
    /// [Acceleration] that doesn't accelerate in any direction
    pub const ZERO: Self = Self::new(Vec3::ZERO);

    pub const fn new(acc: Vec3) -> AngularAcceleration<S> {
        AngularAcceleration(acc, PhantomData)
    }
}

impl<S: CoordinateSystem> CoordinateConvert for Acceleration<S> {
    type Global = Acceleration<Global>;

    type Local = Acceleration<Local>;

    fn to_global(self, rot: bevy::prelude::Quat) -> Self::Global {
        Acceleration::<Global>::new(S::vec3_to_global(self.0, rot))
    }

    fn to_local(self, rot: bevy::prelude::Quat) -> Self::Local {
        Acceleration::<Local>::new(S::vec3_to_local(self.0, rot))
    }
}

impl<S: CoordinateSystem> CoordinateConvert for AngularAcceleration<S> {
    type Global = AngularAcceleration<Global>;

    type Local = AngularAcceleration<Local>;

    fn to_global(self, rot: bevy::prelude::Quat) -> Self::Global {
        AngularAcceleration::<Global>::new(S::vec3_to_global(self.0, rot))
    }

    fn to_local(self, rot: bevy::prelude::Quat) -> Self::Local {
        AngularAcceleration::<Local>::new(S::vec3_to_global(self.0, rot))
    }
}

// Other
impl<S: CoordinateSystem + PartialOrd> PartialOrd for Acceleration<S> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let this = self.0.element_sum();
        let other = other.0.element_sum();

        this.partial_cmp(&other)
    }
}

// ==== Local Coordinate System ====
// impl x for Acceleration<Local>
overload!((a: ?Acceleration<Local>) + (b: ?Acceleration<Local>) -> Acceleration<Local> { Acceleration::new(a.0 + b.0) });
overload!((a: ?Acceleration<Local>) - (b: ?Acceleration<Local>) -> Acceleration<Local> { Acceleration::new(a.0 - b.0) });
overload!((a: ?Acceleration<Local>) * (b: ?Acceleration<Local>) -> Acceleration<Local> { Acceleration::new(a.0 * b.0) });
overload!((a: ?Acceleration<Local>) / (b: ?Acceleration<Local>) -> Acceleration<Local> { Acceleration::new(a.0 / b.0) });

overload!((a: ?Acceleration<Local>) * (b: f32) -> Velocity<Local> { Velocity::new(a.0 * b) });
overload!((a: ?Acceleration<Local>) / (b: f32) -> Acceleration<Local> { Acceleration::new(a.0 / b) });

overload!(- (a: &mut Acceleration<Local>) -> Acceleration<Local> { Acceleration::new(- a.0) });

// impl xAssign for Acceleration<Local>
overload!((a: &mut Acceleration<Local>) += (b: ?Acceleration<Local>) { a.0 += b.0 });
overload!((a: &mut Acceleration<Local>) -= (b: ?Acceleration<Local>) { a.0 -= b.0 });
overload!((a: &mut Acceleration<Local>) *= (b: ?Acceleration<Local>) { a.0 *= b.0 });
overload!((a: &mut Acceleration<Local>) /= (b: ?Acceleration<Local>) { a.0 /= b.0 });

overload!((a: &mut Acceleration<Local>) *= (b: f32) {a.0 *= b});
overload!((a: &mut Acceleration<Local>) /= (b: f32) {a.0 /= b});

// ==== Global Coordinate System ====
// impl x for Acceleration<Global>
overload!((a: ?Acceleration<Global>) + (b: ?Acceleration<Global>) -> Acceleration<Global> { Acceleration::new(a.0 + b.0) });
overload!((a: ?Acceleration<Global>) - (b: ?Acceleration<Global>) -> Acceleration<Global> { Acceleration::new(a.0 - b.0) });
overload!((a: ?Acceleration<Global>) * (b: ?Acceleration<Global>) -> Acceleration<Global> { Acceleration::new(a.0 * b.0) });
overload!((a: ?Acceleration<Global>) / (b: ?Acceleration<Global>) -> Acceleration<Global> { Acceleration::new(a.0 / b.0) });

overload!((a: ?Acceleration<Global>) * (b: f32) -> Velocity<Global> { Velocity::new(a.0 * b) });
overload!((a: ?Acceleration<Global>) / (b: f32) -> Acceleration<Global> { Acceleration::new(a.0 / b) });

overload!(- (a: &mut Acceleration<Global>) -> Acceleration<Global> { Acceleration::new(- a.0) });

// impl xAssign for Acceleration<Global>
overload!((a: &mut Acceleration<Global>) += (b: ?Acceleration<Global>) { a.0 += b.0 });
overload!((a: &mut Acceleration<Global>) -= (b: ?Acceleration<Global>) { a.0 -= b.0 });
overload!((a: &mut Acceleration<Global>) *= (b: ?Acceleration<Global>) { a.0 *= b.0 });
overload!((a: &mut Acceleration<Global>) /= (b: ?Acceleration<Global>) { a.0 /= b.0 });

overload!((a: &mut Acceleration<Global>) *= (b: f32) {a.0 *= b});
overload!((a: &mut Acceleration<Global>) /= (b: f32) {a.0 /= b});

// ==== Local Coordinate System ====
// impl x for AngularAcceleration<Local>
overload!((a: ?AngularAcceleration<Local>) + (b: ?AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new(a.0 + b.0) });
overload!((a: ?AngularAcceleration<Local>) - (b: ?AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new(a.0 - b.0) });
overload!((a: ?AngularAcceleration<Local>) * (b: ?AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new(a.0 * b.0) });
overload!((a: ?AngularAcceleration<Local>) / (b: ?AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new(a.0 / b.0) });

overload!((a: ?AngularAcceleration<Local>) * (b: f32) -> AngularVelocity<Local> { AngularVelocity::new(a.0 * b) });
overload!((a: ?AngularAcceleration<Local>) / (b: f32) -> AngularAcceleration<Local> { AngularAcceleration::new(a.0 / b) });

overload!(- (a: &mut AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new(- a.0) });

// impl xAssign for AngularAcceleration<Local>
overload!((a: &mut AngularAcceleration<Local>) += (b: ?AngularAcceleration<Local>) { a.0 += b.0 });
overload!((a: &mut AngularAcceleration<Local>) -= (b: ?AngularAcceleration<Local>) { a.0 -= b.0 });
overload!((a: &mut AngularAcceleration<Local>) *= (b: ?AngularAcceleration<Local>) { a.0 *= b.0 });
overload!((a: &mut AngularAcceleration<Local>) /= (b: ?AngularAcceleration<Local>) { a.0 /= b.0 });

overload!((a: &mut AngularAcceleration<Local>) *= (b: f32) {a.0 *= b});
overload!((a: &mut AngularAcceleration<Local>) /= (b: f32) {a.0 /= b});

// ==== Global Coordinate System ====
// impl x for AngularAcceleration<Global>
overload!((a: ?AngularAcceleration<Global>) + (b: ?AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new(a.0 + b.0) });
overload!((a: ?AngularAcceleration<Global>) - (b: ?AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new(a.0 - b.0) });
overload!((a: ?AngularAcceleration<Global>) * (b: ?AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new(a.0 * b.0) });
overload!((a: ?AngularAcceleration<Global>) / (b: ?AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new(a.0 / b.0) });

overload!((a: ?AngularAcceleration<Global>) * (b: f32) -> AngularVelocity<Global> { AngularVelocity::new(a.0 * b) });
overload!((a: ?AngularAcceleration<Global>) / (b: f32) -> AngularAcceleration<Global> { AngularAcceleration::new(a.0 / b) });

overload!(- (a: &mut AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new(- a.0) });

// impl xAssign for AngularAcceleration<Global>
overload!((a: &mut AngularAcceleration<Global>) += (b: ?AngularAcceleration<Global>) { a.0 += b.0 });
overload!((a: &mut AngularAcceleration<Global>) -= (b: ?AngularAcceleration<Global>) { a.0 -= b.0 });
overload!((a: &mut AngularAcceleration<Global>) *= (b: ?AngularAcceleration<Global>) { a.0 *= b.0 });
overload!((a: &mut AngularAcceleration<Global>) /= (b: ?AngularAcceleration<Global>) { a.0 /= b.0 });

overload!((a: &mut AngularAcceleration<Global>) *= (b: f32) {a.0 *= b});
overload!((a: &mut AngularAcceleration<Global>) /= (b: f32) {a.0 /= b});
