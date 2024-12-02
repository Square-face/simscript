extern crate overload;
use overload::overload;
use std::{marker::PhantomData, ops};

use bevy::{ecs::component::Component, math::Vec3};

use crate::cordinate_systems::{CoordinateSystem, Global, Local};

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
    /// [Accelerator] that doesn't accelerate in any direction
    pub const ZERO: Self = Self::new(Vec3::ZERO);

    /// [Accelerator] that mimics gravity (-9.82 m/s^2 in y velocity)
    pub const GRAVITY: Self = Self::new(Vec3::new(0.0, -9.82, 0.0));

    pub const fn new(acc: Vec3) -> Acceleration<S> {
        Acceleration(acc, PhantomData)
    }
}

impl<S: CoordinateSystem> AngularAcceleration<S> {
    /// [Accelerator] that doesn't accelerate in any direction
    pub const ZERO: Self = Self::new(Vec3::ZERO);

    pub const fn new(acc: Vec3) -> AngularAcceleration<S> {
        AngularAcceleration(acc, PhantomData)
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

// ===== Acceleration =====
// === Local Coordinate system ===

overload!((a: ?Acceleration<Local>) + (b: ?Acceleration<Local>) -> Acceleration<Local> { Acceleration::new( a.0 + b.0 )});
overload!((a: ?Acceleration<Local>) - (b: ?Acceleration<Local>) -> Acceleration<Local> { Acceleration::new( a.0 - b.0 )});
overload!((a: ?Acceleration<Local>) * (b: ?Acceleration<Local>) -> Acceleration<Local> { Acceleration::new( a.0 * b.0 )});
overload!((a: ?Acceleration<Local>) / (b: ?Acceleration<Local>) -> Acceleration<Local> { Acceleration::new( a.0 / b.0 )});

overload!((a: &mut Acceleration<Local>) += (b: ?Acceleration<Local>) { a.0 += b.0; });
overload!((a: &mut Acceleration<Local>) -= (b: ?Acceleration<Local>) { a.0 -= b.0; });
overload!((a: &mut Acceleration<Local>) *= (b: ?Acceleration<Local>) { a.0 *= b.0; });
overload!((a: &mut Acceleration<Local>) /= (b: ?Acceleration<Local>) { a.0 /= b.0; });

overload!(- (a: ?Acceleration<Local>) -> Acceleration<Local> { Acceleration::new( -a.0 )});

overload!((a: ?Acceleration<Local>) * (b: f32) -> Velocity<Local> { Velocity::new( a.0 * b )});
overload!((a: ?Acceleration<Local>) / (b: f32) -> Velocity<Local> { Velocity::new( a.0 / b )});

overload!((a: &mut Acceleration<Local>) *= (b: f32) { a.0 *= b; });
overload!((a: &mut Acceleration<Local>) /= (b: f32) { a.0 /= b; });

// === Global Coordinate system ===

overload!((a: ?Acceleration<Global>) + (b: ?Acceleration<Global>) -> Acceleration<Global> { Acceleration::new( a.0 + b.0 )});
overload!((a: ?Acceleration<Global>) - (b: ?Acceleration<Global>) -> Acceleration<Global> { Acceleration::new( a.0 - b.0 )});
overload!((a: ?Acceleration<Global>) * (b: ?Acceleration<Global>) -> Acceleration<Global> { Acceleration::new( a.0 * b.0 )});
overload!((a: ?Acceleration<Global>) / (b: ?Acceleration<Global>) -> Acceleration<Global> { Acceleration::new( a.0 / b.0 )});

overload!((a: &mut Acceleration<Global>) += (b: ?Acceleration<Global>) { a.0 += b.0; });
overload!((a: &mut Acceleration<Global>) -= (b: ?Acceleration<Global>) { a.0 -= b.0; });
overload!((a: &mut Acceleration<Global>) *= (b: ?Acceleration<Global>) { a.0 *= b.0; });
overload!((a: &mut Acceleration<Global>) /= (b: ?Acceleration<Global>) { a.0 /= b.0; });

overload!(- (a: ?Acceleration<Global>) -> Acceleration<Global> { Acceleration::new( -a.0 )});

overload!((a: ?Acceleration<Global>) * (b: f32) -> Velocity<Global> { Velocity::new( a.0 * b )});
overload!((a: ?Acceleration<Global>) / (b: f32) -> Velocity<Global> { Velocity::new( a.0 / b )});

overload!((a: &mut Acceleration<Global>) *= (b: f32) { a.0 *= b; });
overload!((a: &mut Acceleration<Global>) /= (b: f32) { a.0 /= b; });

// ===== Angular Acceleration =====
// === Local Coordinate system ===

overload!((a: ?AngularAcceleration<Local>) + (b: ?AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new( a.0 + b.0 )});
overload!((a: ?AngularAcceleration<Local>) - (b: ?AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new( a.0 - b.0 )});
overload!((a: ?AngularAcceleration<Local>) * (b: ?AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new( a.0 * b.0 )});
overload!((a: ?AngularAcceleration<Local>) / (b: ?AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new( a.0 / b.0 )});

overload!((a: &mut AngularAcceleration<Local>) += (b: ?AngularAcceleration<Local>) { a.0 += b.0; });
overload!((a: &mut AngularAcceleration<Local>) -= (b: ?AngularAcceleration<Local>) { a.0 -= b.0; });
overload!((a: &mut AngularAcceleration<Local>) *= (b: ?AngularAcceleration<Local>) { a.0 *= b.0; });
overload!((a: &mut AngularAcceleration<Local>) /= (b: ?AngularAcceleration<Local>) { a.0 /= b.0; });

overload!((a: ?AngularAcceleration<Local>) * (b: f32) -> AngularVelocity<Local> { AngularVelocity::new( a.0 * b )});
overload!((a: ?AngularAcceleration<Local>) / (b: f32) -> AngularVelocity<Local> { AngularVelocity::new( a.0 / b )});

overload!((a: &mut AngularAcceleration<Local>) *= (b: f32) { a.0 *= b; });
overload!((a: &mut AngularAcceleration<Local>) /= (b: f32) { a.0 /= b; });

overload!(- (a: ?AngularAcceleration<Local>) -> AngularAcceleration<Local> { AngularAcceleration::new( -a.0 )});

// === Global Coordinate system ===

overload!((a: ?AngularAcceleration<Global>) + (b: ?AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new( a.0 + b.0 )});
overload!((a: ?AngularAcceleration<Global>) - (b: ?AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new( a.0 - b.0 )});
overload!((a: ?AngularAcceleration<Global>) * (b: ?AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new( a.0 * b.0 )});
overload!((a: ?AngularAcceleration<Global>) / (b: ?AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new( a.0 / b.0 )});

overload!((a: &mut AngularAcceleration<Global>) += (b: ?AngularAcceleration<Global>) { a.0 += b.0; });
overload!((a: &mut AngularAcceleration<Global>) -= (b: ?AngularAcceleration<Global>) { a.0 -= b.0; });
overload!((a: &mut AngularAcceleration<Global>) *= (b: ?AngularAcceleration<Global>) { a.0 *= b.0; });
overload!((a: &mut AngularAcceleration<Global>) /= (b: ?AngularAcceleration<Global>) { a.0 /= b.0; });

overload!((a: ?AngularAcceleration<Global>) * (b: f32) -> AngularVelocity<Global> { AngularVelocity::new( a.0 * b )});
overload!((a: ?AngularAcceleration<Global>) / (b: f32) -> AngularVelocity<Global> { AngularVelocity::new( a.0 / b )});

overload!((a: &mut AngularAcceleration<Global>) *= (b: f32) { a.0 *= b; });
overload!((a: &mut AngularAcceleration<Global>) /= (b: f32) { a.0 /= b; });

overload!(- (a: ?AngularAcceleration<Global>) -> AngularAcceleration<Global> { AngularAcceleration::new( -a.0 )});

