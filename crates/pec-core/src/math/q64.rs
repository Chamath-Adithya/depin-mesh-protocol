//! Signed Q64.64 fixed-point number implementation for `PEC-MATH-01`.
//!
//! Value = raw * 2^(-64)

use core::ops::{Add, Sub};

/// Signed 128-bit fixed-point number with 64 integer bits and 64 fractional bits.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Q64(pub i128);

impl Q64 {
    /// Maximum representable value (~9.223372036854775807e18).
    pub const MAX: Self = Self(i128::MAX);

    /// Minimum representable value (~ -9.223372036854775808e18).
    pub const MIN: Self = Self(i128::MIN);

    /// Fixed-point constant 0.0.
    pub const ZERO: Self = Self(0);

    /// Fixed-point constant 1.0 (2^64).
    pub const ONE: Self = Self(1i128 << 64);

    /// Creates a Q64 fixed-point number from an integer value.
    #[inline]
    pub const fn from_int(val: i64) -> Self {
        Self((val as i128) << 64)
    }

    /// Performs saturating addition.
    #[inline]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self(self.0.saturating_add(rhs.0))
    }

    /// Performs saturating subtraction.
    #[inline]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }

    /// Returns the raw underlying i128 representation.
    #[inline]
    pub const fn to_raw(self) -> i128 {
        self.0
    }
}

impl Add for Q64 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl Sub for Q64 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saturating_add_identity_and_basic() {
        let one = Q64::ONE;
        let two = one.saturating_add(one);
        assert_eq!(two.0, 2i128 << 64);
        assert_eq!(two.saturating_sub(one), one);
    }

    #[test]
    fn test_saturating_add_positive_overflow() {
        let max = Q64::MAX;
        let one = Q64::ONE;
        assert_eq!(max.saturating_add(one), Q64::MAX);
        assert_eq!(max.saturating_add(Q64(1)), Q64::MAX);
    }

    #[test]
    fn test_saturating_sub_negative_underflow() {
        let min = Q64::MIN;
        let one = Q64::ONE;
        assert_eq!(min.saturating_sub(one), Q64::MIN);
        assert_eq!(min.saturating_sub(Q64(1)), Q64::MIN);
    }

    #[test]
    fn test_operator_overloads() {
        let one = Q64::ONE;
        let three = Q64::from_int(3);
        assert_eq!((one + one + one), three);
        assert_eq!((three - one), Q64::from_int(2));
    }
}
