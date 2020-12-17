#![feature(min_const_generics)]

mod comps;
mod prelude;
mod traits;

use std::fmt;
use std::iter;
use std::ops;
use std::slice::Iter;

use crate::prelude::*;

/// Represents an n-dimensional vector.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vector<T, const N: usize> {
    inner: [T; N],
}

impl<T, const N: usize> fmt::Debug for Vector<T, N>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Vector").field(&self.inner).finish()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Copy + Zero,
{
    fn default() -> Self {
        let inner = [Zero::zero(); N];
        Self { inner }
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(arr: [T; N]) -> Self {
        Self { inner: arr }
    }
}

macro_rules! impl_from_tuple {
    ($tuple:ty, $N:literal, $($n:ident),+) => {
        impl<T> From<$tuple> for Vector<T, $N> {
            fn from(($($n,)+): $tuple) -> Self {
                Self::from([$($n,)+])
            }
        }
    };
}

impl_from_tuple!((T, T), 2, x, y);
impl_from_tuple!((T, T, T), 3, x, y, z);
impl_from_tuple!((T, T, T, T), 4, x, y, z, w);

////////////////////////////////////////////////////////////////////////////////
// Operator overloading
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_add {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl<T, const N: usize> ops::Add<$rhs> for $lhs
        where
            T: Copy + Zero + ops::Add<Output = T>,
        {
            type Output = $output;

            fn add(self, other: $rhs) -> Self::Output {
                let mut vector = Vector::default();
                for i in 0..N {
                    vector.inner[i] = self.inner[i] + other.inner[i];
                }
                vector
            }
        }
    };
}

macro_rules! impl_sub {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl<T, const N: usize> ops::Sub<$rhs> for $lhs
        where
            T: Copy + Zero + ops::Sub<Output = T>,
        {
            type Output = $output;

            fn sub(self, other: $rhs) -> Self::Output {
                let mut vector = Vector::default();
                for i in 0..N {
                    vector.inner[i] = self.inner[i] - other.inner[i];
                }
                vector
            }
        }
    };
}

macro_rules! impl_mul {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl<'a, T, const N: usize> ops::Mul<$rhs> for $lhs
        where
            T: Copy + Zero + ops::Mul<$rhs, Output = T>,
        {
            type Output = $output;

            fn mul(self, other: $rhs) -> Self::Output {
                let mut vector = Vector::default();
                for i in 0..N {
                    vector.inner[i] = self.inner[i] * other;
                }
                vector
            }
        }
    };
}

macro_rules! impl_add_assign {
    ($self:ty, $other:ty) => {
        impl<T, const N: usize> ops::AddAssign<$other> for $self
        where
            T: Copy + ops::AddAssign,
        {
            fn add_assign(&mut self, other: $other) {
                for i in 0..N {
                    self.inner[i] += other.inner[i]
                }
            }
        }
    };
}

macro_rules! impl_sub_assign {
    ($self:ty, $other:ty) => {
        impl<T, const N: usize> ops::SubAssign<$other> for $self
        where
            T: Copy + ops::SubAssign,
        {
            fn sub_assign(&mut self, other: $other) {
                for i in 0..N {
                    self.inner[i] -= other.inner[i]
                }
            }
        }
    };
}

impl_add!(Vector<T, N>, Vector<T, N>, Vector<T, N>);
impl_add!(Vector<T, N>, &Vector<T, N>, Vector<T, N>);
impl_add!(&Vector<T, N>, &Vector<T, N>, Vector<T, N>);

impl_sub!(Vector<T, N>, Vector<T, N>, Vector<T, N>);
impl_sub!(Vector<T, N>, &Vector<T, N>, Vector<T, N>);
impl_sub!(&Vector<T, N>, &Vector<T, N>, Vector<T, N>);

impl_mul!(Vector<T, N>, T, Vector<T, N>);
impl_mul!(Vector<T, N>, &'a T, Vector<T, N>);
impl_mul!(&Vector<T, N>, T, Vector<T, N>);
impl_mul!(&Vector<T, N>, &'a T, Vector<T, N>);

impl_add_assign!(Vector<T, N>, Vector<T, N>);
impl_add_assign!(Vector<T, N>, &Vector<T, N>);

impl_sub_assign!(Vector<T, N>, Vector<T, N>);
impl_sub_assign!(Vector<T, N>, &Vector<T, N>);

////////////////////////////////////////////////////////////////////////////////
// General methods
////////////////////////////////////////////////////////////////////////////////

impl<T, const N: usize> Vector<T, N> {
    /// Views the underlying vector representation as a slice.
    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Copy,
{
    /// An iterator visiting each component in dimension order.
    pub fn iter(&self) -> Iter<T> {
        self.inner.iter()
    }

    fn iter_copied(&self) -> iter::Copied<Iter<T>> {
        self.iter().copied()
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Copy + ops::Mul<Output = T> + iter::Sum<T>,
{
    /// Calculates the dot-product between `self` and `other`.
    pub fn dot(&self, other: &Self) -> T {
        self.iter_copied()
            .zip(other.iter_copied())
            .map(|(a, b)| a * b)
            .sum()
    }
}
