//! Abstractions over number types.
//!
//! Some of code in this module is taken from the `num-traits` crate.

use std::fmt::Debug;
use std::iter::{Product, Sum};
use std::ops::*;

/// Conversion trait for conversions from incomplete data.
///
/// See the [`Vector::from_partial`][crate::Vector::from_partial] function.
pub trait FromPartial<T, U> {
    fn from_partial(_: U, fill: T) -> Self;
}

/// The trait for types implementing basic numeric operations.
pub trait Ops<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
{
}
impl<T, Rhs, Output> Ops<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
        + Rem<Rhs, Output = Output>
{
}

/// The trait for types implementing numeric assignment operators.
pub trait AssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{
}
impl<T, Rhs> AssignOps<Rhs> for T where
    T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{
}

/// A base trait for numeric types.
pub trait Num:
    Sized
    + Debug
    + Copy
    + PartialEq
    + Zero
    + One
    + Abs
    + Ops<Self> // Num + Num
    + AssignOps<Self> // Num += Num
    + for<'r> Ops<&'r Self> // Num + &Num
    + for<'r> AssignOps<&'r Self> // Num += &Num
    + Sum<Self>
    + Product<Self>
{}

impl<T> Num for T where
    T: Sized
        + Debug
        + Copy
        + PartialEq
        + Zero
        + One
        + Abs
        + Ops<Self> // Num + Num
        + AssignOps<Self> // Num += Num
        + for<'r> Ops<&'r Self> // Num + &Num
        + for<'r> AssignOps<&'r Self> // Num += &Num
        + Sum<Self>
        + Product<Self>
{
}

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
    ($($ty:ty)+) => ($(
        impl Abs for $ty {
            #[inline]
            fn abs(self) -> $ty {
                self.abs()
            }
        }
    )+)
}

macro_rules! impl_abs_self {
    ($($ty:ty)+) => ($(
        impl Abs for $ty {
            #[inline]
            fn abs(self) -> $ty {
                self
            }
        }
    )+)
}

impl_one!(1 usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);
impl_one!(1.0 f32 f64);

impl_zero!(0 usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);
impl_zero!(0.0 f32 f64);

impl_abs!(isize i8 i16 i32 i64 i128 f32 f64);
impl_abs_self!(usize u8 u16 u32 u64 u128);
