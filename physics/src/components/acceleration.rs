extern crate overload;
use overload::overload;
use std::ops;

use bevy::{ecs::component::Component, math::Vec3};

/// Represents a linear acceleration
///
/// Works similar to [Velocity] in that the acceleration is represented as a Vec3 in global
/// cordinates
#[derive(Component, Debug, PartialEq)]
pub struct Acceleration(pub Vec3);

impl Acceleration {
    /// [Accelerator] that doesn't accelerate in any direction
    pub const ZERO: Self = Self(Vec3::ZERO);

    /// [Accelerator] that mimics gravity (-9.82 m/s^2 in y velocity)
    pub const GRAVITY: Self = Self(Vec3::new(0.0, -9.82, 0.0));
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

overload!((a: ?Acceleration) + (b: ?Acceleration) -> Acceleration { Acceleration ( a.0 + b.0 )});
overload!((a: ?Acceleration) - (b: ?Acceleration) -> Acceleration { Acceleration ( a.0 - b.0 )});
overload!((a: ?Acceleration) * (b: ?Acceleration) -> Acceleration { Acceleration ( a.0 * b.0 )});
overload!((a: ?Acceleration) / (b: ?Acceleration) -> Acceleration { Acceleration ( a.0 / b.0 )});

overload!((a: &mut Acceleration) += (b: ?Acceleration) { a.0 += b.0; });
overload!((a: &mut Acceleration) -= (b: ?Acceleration) { a.0 -= b.0; });
overload!((a: &mut Acceleration) *= (b: ?Acceleration) { a.0 *= b.0; });
overload!((a: &mut Acceleration) /= (b: ?Acceleration) { a.0 /= b.0; });

overload!(- (a: ?Acceleration) -> Acceleration { Acceleration ( -a.0 )});
