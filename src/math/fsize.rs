use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct FSize(f32);

impl FSize {
    pub const ZERO: FSize = FSize(0.0);
    pub const INFINITY: FSize = FSize(f32::INFINITY);

    pub fn new(size: f32) -> Self {
        Self::try_from(size).unwrap()
    }
}

impl TryFrom<f32> for FSize {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value.is_nan() {
            Err("SizeF32 cannot be NaN")
        } else if value.is_sign_negative() {
            Err("SizeF32 cannot be negative")
        } else {
            Ok(FSize(value))
        }
    }
}

impl Deref for FSize {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Eq for FSize {}
impl Ord for FSize {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

use std::ops::{Add, Sub, Mul, Div, Rem};

impl Add for FSize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}

impl Sub for FSize {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}

impl Mul for FSize {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

impl Div for FSize {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.0 / rhs.0)
    }
}

impl Rem for FSize {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::new(self.0 % rhs.0)
    }
}

impl Add<f32> for FSize {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

impl Sub<f32> for FSize {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::new(self.0 - rhs)
    }
}

impl Mul<f32> for FSize {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.0 * rhs)
    }
}

impl Div<f32> for FSize {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.0 / rhs)
    }
}

impl Rem<f32> for FSize {
    type Output = Self;

    fn rem(self, rhs: f32) -> Self::Output {
        Self::new(self.0 % rhs)
    }
}

impl Display for FSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}