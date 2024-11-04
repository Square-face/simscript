extern crate overload;
use overload::overload;
use std::ops;

use bevy::{ecs::component::Component, math::Vec3};

use super::velocity::{AngularVelocity, Velocity};

/// Represents a linear acceleration
///
/// Ment to be used together with [Velocity]
///
/// Values are in `m/s^2`
#[derive(Component, Debug, PartialEq)]
pub struct Acceleration(pub Vec3);

/// Represents a rotational acceleration
///
/// Meant to be used together with [AngularVelocity]
///
/// Values are in `rad/s^2`
#[derive(Component, Debug, PartialEq)]
pub struct AngularAcceleration(pub Vec3);

impl Acceleration {
    /// [Accelerator] that doesn't accelerate in any direction
    pub const ZERO: Self = Self(Vec3::ZERO);

    /// [Accelerator] that mimics gravity (-9.82 m/s^2 in y velocity)
    pub const GRAVITY: Self = Self(Vec3::new(0.0, -9.82, 0.0));
}

impl AngularAcceleration {
    /// [Accelerator] that doesn't accelerate in any direction
    pub const ZERO: Self = Self(Vec3::ZERO);
}

// Other
impl PartialOrd for Acceleration {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let this = self.0.element_sum();
        let other = other.0.element_sum();

        this.partial_cmp(&other)
    }
}
overload!(- (a: ?Acceleration) -> Acceleration { Acceleration ( -a.0 )});
overload!(- (a: ?AngularAcceleration) -> AngularAcceleration { AngularAcceleration ( -a.0 )});

// Acceleration, Acceleration

overload!((a: ?Acceleration) + (b: ?Acceleration) -> Acceleration { Acceleration ( a.0 + b.0 )});
overload!((a: ?Acceleration) - (b: ?Acceleration) -> Acceleration { Acceleration ( a.0 - b.0 )});
overload!((a: ?Acceleration) * (b: ?Acceleration) -> Acceleration { Acceleration ( a.0 * b.0 )});
overload!((a: ?Acceleration) / (b: ?Acceleration) -> Acceleration { Acceleration ( a.0 / b.0 )});

overload!((a: &mut Acceleration) += (b: ?Acceleration) { a.0 += b.0; });
overload!((a: &mut Acceleration) -= (b: ?Acceleration) { a.0 -= b.0; });
overload!((a: &mut Acceleration) *= (b: ?Acceleration) { a.0 *= b.0; });
overload!((a: &mut Acceleration) /= (b: ?Acceleration) { a.0 /= b.0; });

// Acceleration, f32

overload!((a: ?Acceleration) * (b: f32) -> Velocity { Velocity ( a.0 * b )});
overload!((a: ?Acceleration) / (b: f32) -> Velocity { Velocity ( a.0 / b )});

overload!((a: &mut Acceleration) *= (b: f32) { a.0 *= b; });
overload!((a: &mut Acceleration) /= (b: f32) { a.0 /= b; });

// Angular Acceleration, Angular Acceleration

overload!((a: ?AngularAcceleration) + (b: ?AngularAcceleration) -> AngularAcceleration { AngularAcceleration ( a.0 + b.0 )});
overload!((a: ?AngularAcceleration) - (b: ?AngularAcceleration) -> AngularAcceleration { AngularAcceleration ( a.0 - b.0 )});
overload!((a: ?AngularAcceleration) * (b: ?AngularAcceleration) -> AngularAcceleration { AngularAcceleration ( a.0 * b.0 )});
overload!((a: ?AngularAcceleration) / (b: ?AngularAcceleration) -> AngularAcceleration { AngularAcceleration ( a.0 / b.0 )});

overload!((a: &mut AngularAcceleration) += (b: ?AngularAcceleration) { a.0 += b.0; });
overload!((a: &mut AngularAcceleration) -= (b: ?AngularAcceleration) { a.0 -= b.0; });
overload!((a: &mut AngularAcceleration) *= (b: ?AngularAcceleration) { a.0 *= b.0; });
overload!((a: &mut AngularAcceleration) /= (b: ?AngularAcceleration) { a.0 /= b.0; });

// AngularAcceleration, f32

overload!((a: ?AngularAcceleration) * (b: f32) -> AngularVelocity { AngularVelocity ( a.0 * b )});
overload!((a: ?AngularAcceleration) / (b: f32) -> AngularVelocity { AngularVelocity ( a.0 / b )});
