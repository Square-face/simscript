use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use bevy::{ecs::component::Component, math::Vec3};

///
/// Applies a constant acceleration
///
/// Works similar to [Velocity] in that the acceleration is represented as a Vec3 in global
/// cordinates
#[derive(Component, Debug)]
pub struct Accelerator(pub Vec3);

impl Accelerator {
    /// [Accelerator] that doesn't accelerate in any direction
    pub const ZERO: Self = Self(Vec3::ZERO);

    /// [Accelerator] that mimics gravity (-9.82 m/s^2 in y velocity)
    pub const GRAVITY: Self = Self(Vec3::new(0.0, -9.82, 0.0));
}

impl Add for Accelerator {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Accelerator {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Accelerator {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Accelerator {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for Accelerator {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for Accelerator {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Div for Accelerator {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for Accelerator {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl Mul<f32> for Accelerator {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<f32> for Accelerator {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs
    }
}
