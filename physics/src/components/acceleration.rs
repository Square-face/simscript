use bevy::{ecs::component::Component, math::Vec3};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

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

// Addition implementations
impl Add<Accelerator> for Accelerator {
    type Output = Accelerator;

    #[inline]
    fn add(self, rhs: Accelerator) -> Self::Output {
        Accelerator(self.0 + rhs.0)
    }
}

impl Add<&Accelerator> for Accelerator {
    type Output = Accelerator;

    #[inline]
    fn add(self, rhs: &Accelerator) -> Self::Output {
        Accelerator(self.0 + rhs.0)
    }
}

impl Add<Accelerator> for &Accelerator {
    type Output = Accelerator;

    #[inline]
    fn add(self, rhs: Accelerator) -> Self::Output {
        Accelerator(self.0 + rhs.0)
    }
}

impl Add<&Accelerator> for &Accelerator {
    type Output = Accelerator;

    #[inline]
    fn add(self, rhs: &Accelerator) -> Self::Output {
        Accelerator(self.0 + rhs.0)
    }
}

impl AddAssign<Accelerator> for Accelerator {
    #[inline]
    fn add_assign(&mut self, rhs: Accelerator) {
        self.0 += rhs.0
    }
}

impl AddAssign<&Accelerator> for Accelerator {
    #[inline]
    fn add_assign(&mut self, rhs: &Accelerator) {
        self.0 += rhs.0
    }
}

// Subtraction implementations
impl Sub<Accelerator> for Accelerator {
    type Output = Accelerator;

    #[inline]
    fn sub(self, rhs: Accelerator) -> Self::Output {
        Accelerator(self.0 - rhs.0)
    }
}

impl Sub<&Accelerator> for Accelerator {
    type Output = Accelerator;

    #[inline]
    fn sub(self, rhs: &Accelerator) -> Self::Output {
        Accelerator(self.0 - rhs.0)
    }
}

impl Sub<Accelerator> for &Accelerator {
    type Output = Accelerator;

    #[inline]
    fn sub(self, rhs: Accelerator) -> Self::Output {
        Accelerator(self.0 - rhs.0)
    }
}

impl Sub<&Accelerator> for &Accelerator {
    type Output = Accelerator;

    #[inline]
    fn sub(self, rhs: &Accelerator) -> Self::Output {
        Accelerator(self.0 - rhs.0)
    }
}

impl SubAssign<Accelerator> for Accelerator {
    #[inline]
    fn sub_assign(&mut self, rhs: Accelerator) {
        self.0 -= rhs.0
    }
}

impl SubAssign<&Accelerator> for Accelerator {
    #[inline]
    fn sub_assign(&mut self, rhs: &Accelerator) {
        self.0 -= rhs.0
    }
}

// Multiplication implementations
impl Mul<Accelerator> for Accelerator {
    type Output = Accelerator;

    #[inline]
    fn mul(self, rhs: Accelerator) -> Self::Output {
        Accelerator(self.0 * rhs.0)
    }
}

impl Mul<&Accelerator> for Accelerator {
    type Output = Accelerator;

    #[inline]
    fn mul(self, rhs: &Accelerator) -> Self::Output {
        Accelerator(self.0 * rhs.0)
    }
}

impl Mul<Accelerator> for &Accelerator {
    type Output = Accelerator;

    #[inline]
    fn mul(self, rhs: Accelerator) -> Self::Output {
        Accelerator(self.0 * rhs.0)
    }
}

impl Mul<&Accelerator> for &Accelerator {
    type Output = Accelerator;

    #[inline]
    fn mul(self, rhs: &Accelerator) -> Self::Output {
        Accelerator(self.0 * rhs.0)
    }
}

impl MulAssign<Accelerator> for Accelerator {
    #[inline]
    fn mul_assign(&mut self, rhs: Accelerator) {
        self.0 *= rhs.0
    }
}

impl MulAssign<&Accelerator> for Accelerator {
    #[inline]
    fn mul_assign(&mut self, rhs: &Accelerator) {
        self.0 *= rhs.0
    }
}

// Division implementations
impl Div<Accelerator> for Accelerator {
    type Output = Accelerator;

    #[inline]
    fn div(self, rhs: Accelerator) -> Self::Output {
        Accelerator(self.0 / rhs.0)
    }
}

impl Div<&Accelerator> for Accelerator {
    type Output = Accelerator;

    #[inline]
    fn div(self, rhs: &Accelerator) -> Self::Output {
        Accelerator(self.0 / rhs.0)
    }
}

impl Div<Accelerator> for &Accelerator {
    type Output = Accelerator;

    #[inline]
    fn div(self, rhs: Accelerator) -> Self::Output {
        Accelerator(self.0 / rhs.0)
    }
}

impl Div<&Accelerator> for &Accelerator {
    type Output = Accelerator;

    #[inline]
    fn div(self, rhs: &Accelerator) -> Self::Output {
        Accelerator(self.0 / rhs.0)
    }
}

impl DivAssign<Accelerator> for Accelerator {
    #[inline]
    fn div_assign(&mut self, rhs: Accelerator) {
        self.0 /= rhs.0
    }
}

impl DivAssign<&Accelerator> for Accelerator {
    #[inline]
    fn div_assign(&mut self, rhs: &Accelerator) {
        self.0 /= rhs.0
    }
}

// Other
impl Neg for Accelerator {
    type Output = Accelerator;

    #[inline]
    fn neg(self) -> Self::Output {
        Accelerator(-self.0)
    }
}

impl PartialEq for Accelerator {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Accelerator {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let this = self.0.element_sum();
        let other = other.0.element_sum();

        this.partial_cmp(&other)
    }
}
