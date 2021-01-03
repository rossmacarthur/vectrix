//! Abstractions over number types.
//!
//! Some of code in this module is taken from the `num-traits` crate.

pub(crate) use core::fmt::Debug;
pub(crate) use core::iter::Sum;
pub(crate) use core::ops::*;

/// Conversion trait for conversions from incomplete data.
///
/// See the [`Vector::from_partial`][crate::Vector::from_partial] function.
pub trait FromPartial<T, U> {
    fn from_partial(_: U, fill: T) -> Self;
}

/// A base trait for numeric types.
///
// FIXME: once rust-lang/rust#61956 is fixed this can probably be removed.
pub trait Base: Copy + Default {}
impl<T> Base for T where T: Copy + Default {}

/// Defines the absolute value for a type.
pub trait Abs {
    /// Returns the absolute value of this type.
    fn abs(self) -> Self;
}

/// Defines a multiplicative identity element for a type.
pub trait One {
    /// Returns the multiplicative identity element of this type.
    fn one() -> Self;
}

/// Defines a additive identity element for a type.
pub trait Zero {
    /// Returns the additive identity element of this type.
    fn zero() -> Self;
}

macro_rules! impl_one {
    ($one:literal $($ty:ty)+) => ($(
        impl One for $ty {
            #[inline]
            fn one() -> $ty {
                $one
            }
        }
    )+)
}

macro_rules! impl_zero {
    ($zero:literal $($ty:ty)+) => ($(
        impl Zero for $ty {
            #[inline]
            fn zero() -> $ty {
                $zero
            }
        }
    )+)
}

macro_rules! impl_abs {
    ($($ty:ident)+) => ($(
        impl Abs for $ty {
            #[inline]
            fn abs(self) -> $ty {
                $ty::abs(self)
            }
        }
    )+)
}

macro_rules! impl_abs_self {
    ($($ty:ident)+) => ($(
        impl Abs for $ty {
            #[inline]
            fn abs(self) -> $ty {
                self
            }
        }
    )+)
}

impl_one! { 1 usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
impl_one! { 1.0 f32 f64 }

impl_zero! { 0 usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
impl_zero! { 0.0 f32 f64 }

impl_abs_self! { usize u8 u16 u32 u64 u128 }
impl_abs! { isize i8 i16 i32 i64 i128 }
#[cfg(feature = "std")]
impl_abs! { f32 f64 }
