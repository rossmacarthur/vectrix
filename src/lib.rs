#![feature(min_const_generics)]

mod comps;
mod prelude;
mod traits;

use std::fmt;
use std::iter;
use std::ops;

use crate::prelude::*;

/// Represents a constant size n-dimensional vector.
///
/// # Examples
///
/// Accessors are provided for small vectors.
///
/// ```
/// # use vectrs::Vector;
/// #
/// let vector = Vector::from((1, 2, 3, 4));
/// assert_eq!(vector.x(), 1);
/// assert_eq!(vector.y(), 2);
/// assert_eq!(vector.z(), 3);
/// ```
///
/// Data is represented internally using an array. `Vector<T, N>` implements
/// `Deref<Target = [T]>` so all slice methods are available.
///
/// ```
/// # use vectrs::Vector;
/// #
/// let vector: Vector<u8, 4> = Default::default();
/// // this uses the `[T]::len()` implementation.
/// assert_eq!(vector.len(), 4);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vector<T, const N: usize> {
    inner: [T; N],
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for Vector<T, N> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Vector").field(&self.inner).finish()
    }
}

impl<T, const N: usize> ops::Deref for Vector<T, N> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, const N: usize> ops::DerefMut for Vector<T, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Copy + Zero,
{
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    #[inline]
    fn from(arr: [T; N]) -> Self {
        Self { inner: arr }
    }
}

macro_rules! impl_from_tuple {
    ($tuple:ty, $N:literal, $($n:ident),+) => {
        impl<T> From<$tuple> for Vector<T, $N> {
            #[inline]
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
// Operators
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_add {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl<T, const N: usize> ops::Add<$rhs> for $lhs
        where
            T: Copy + Zero + ops::Add<Output = T>,
        {
            type Output = $output;

            #[inline]
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

            #[inline]
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

            #[inline]
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
            #[inline]
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
            #[inline]
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
// Iterators
////////////////////////////////////////////////////////////////////////////////

/// An iterator that moves out of a vector.
///
/// This `struct` is created by the `into_iter` method on [`Vector`] (provided
/// by the [`IntoIterator`] trait).
///
/// # Examples
///
/// ```
/// # use vectrs::{IntoIter, Vector};
/// #
/// let v = Vector::from([0, 1, 2]);
/// let iter: IntoIter<_, 3> = v.into_iter();
/// ```
#[derive(Debug)]
pub struct IntoIter<T, const N: usize> {
    vector: Vector<T, N>,
    left: usize,
    right: usize,
}

impl<T: Copy, const N: usize> IntoIter<T, N> {
    #[inline]
    fn new(vector: Vector<T, N>) -> Self {
        Self {
            vector,
            left: 0,
            right: vector.inner.len(),
        }
    }
}

impl<T: Copy, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.left == self.right {
            None
        } else {
            let next = unsafe { self.vector.inner.get_unchecked(self.left) };
            self.left += 1;
            Some(*next)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.right - self.left;
        (remaining, Some(remaining))
    }

    #[inline]
    fn count(self) -> usize {
        self.right - self.left
    }
}

impl<T: Copy, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.left == self.right {
            None
        } else {
            self.right -= 1;
            let next = unsafe { self.vector.inner.get_unchecked(self.right) };
            Some(*next)
        }
    }
}

impl<T: Copy, const N: usize> ExactSizeIterator for IntoIter<T, N> {}

impl<T: Copy, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

////////////////////////////////////////////////////////////////////////////////
// General methods
////////////////////////////////////////////////////////////////////////////////

impl<T: Copy + Zero, const N: usize> Vector<T, N> {
    /// Returns a zero vector.
    #[inline]
    pub fn zero() -> Self {
        let inner = [Zero::zero(); N];
        Self { inner }
    }
}

impl<T, const N: usize> Vector<T, N> {
    /// Views the underlying vector representation as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self
    }

    /// Views the underlying vector representation as a mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Copy + ops::Mul<Output = T> + iter::Sum<T>,
{
    /// Calculates the dot-product between `self` and `other`.
    #[inline]
    pub fn dot(&self, other: &Self) -> T {
        self.into_iter()
            .zip(other.into_iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Copy + Abs,
{
    /// Returns the absolute value of the vector.
    #[inline]
    pub fn abs(mut self) -> Self {
        for n in self.iter_mut() {
            *n = n.abs();
        }
        self
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Copy + iter::Sum<T> + Abs,
{
    /// Returns the L1 norm of the vector.
    ///
    /// Also known as *Manhattan Distance* or *Taxicab norm*. L1 Norm is the sum
    /// of the magnitudes of the vectors in a space.
    #[inline]
    pub fn l1_norm(&self) -> T {
        self.into_iter().map(|n| n.abs()).sum()
    }
}
