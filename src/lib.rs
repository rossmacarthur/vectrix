//! This crate provides a stack-allocated, constant-size, *n*-dimensional
//! [`Vector<T, N>`] type.
//!
//! # Constructors
//!
//! ### Directly using `From`
//!
//! A [`Vector`] is backed by an array. The simplest way to construct a
//! [`Vector`] is to create it directly from an array or tuple. In both of these
//! cases the size of the `Vector` can be easily inferred by the Rust type
//! system.
//!
//! From an array:
//! ```
//! # use vectrs::Vector;
//! #
//! let v = Vector::from([1, 2, 3, 4]);
//! //  ^ Rust automatically infers that the type is `Vector<_, 4>`.
//! ```
//!
//! From a tuple:
//! ```
//! # use vectrs::Vector;
//! #
//! // ... 1 to 12 element tuples are supported
//! let v = Vector::from((1, 2, 3));
//! //  ^ Rust automatically infers that the type is `Vector<_, 3>`.
//! ```
//!
//! ### Collecting from an iterator
//!
//! The other common method of constructing a [`Vector`] is to use the
//! `.collect()` method on an iterator. When collecting from an iterator,
//! `.collect()` will panic if there are not enough elements to fill the
//! [`Vector`]. If there are extra elements they will be ignored.
//! ```
//! # use vectrs::Vector;
//! #
//! let heap = vec![1, 2, 3, 4, 5];
//! let stack: Vector<_, 5> = heap.into_iter().collect();
//! //         ^^^^^^^^^^^^ the type needs to be provided in this case
//! ```
//!
//! ### Using `from_partial{_with}`
//!
//! It is common that you do not have enough elements to fill the [`Vector`]. So
//! the [`.from_partial()`][Vector::from_partial] and
//! [`.from_partial_with()`][Vector::from_partial_with] methods are provided.
//! These can be used to construct a [`Vector`] and fill the remaining space
//! with zeroes or a fill value.
//!
//! ```
//! # use vectrs::Vector;
//! #
//! let v = Vector::<_, 3>::from_partial((1, 2));
//! assert_eq!(v, Vector::from([1, 2, 0]));
//!
//! let v = Vector::<_, 5>::from_partial_with((3, 2, 1), 1);
//! assert_eq!(v, Vector::from([3, 2, 1, 1, 1]));     // ^ fill value
//! ```
//!
//! # Accessing and mutating data
//!
//! ### Slice representation
//!
//! A slice view of the underlying data is provided using `Deref` or
//! [`.as_slice()`][Vector::as_slice]. This means all slice methods are
//! available including indexing.
//! ```
//! # use vectrs::Vector;
//! #
//! let vector = Vector::from([1, 3, 3, 7]);
//! assert_eq!(vector[3], 7);
//! ```
//!
//! A mutable slice view of the underlying data is provide using `DerefMut` or
//! [`.as_mut_slice()`][Vector::as_mut_slice]. This means you can mutate data
//! using slice indexing.
//! ```
//! # use vectrs::Vector;
//! #
//! let mut vector = Vector::from([1, 3, 3, 7]);
//! vector[0] = 2;
//! assert_eq!(vector, Vector::from([2, 3, 3, 7]));
//! ```
//!
//! ### Component accessor methods
//!
//! Component accessor methods are available for small vectors using commonly
//! recognized names.
//! ```
//! # use vectrs::Vector;
//! #
//! let vector = Vector::from([1, 3, 3, 7]);
//! assert_eq!(vector.x(), 1);
//! assert_eq!(vector.y(), 3);
//! assert_eq!(vector.z(), 3);
//! assert_eq!(vector.w(), 7);
//! ```
//!
//! Additionally, you can get mutable access using the `*_mut` versions.
//! ```
//! # use vectrs::Vector;
//! #
//! let mut vector = Vector::from([1, 3, 3, 7]);
//! *vector.y_mut() = 2;
//! *vector.w_mut() = 4;
//! assert_eq!(vector, Vector::from([1, 2, 3, 4]));
//! ```

#![feature(const_fn)]
#![feature(iterator_fold_self)]
#![feature(min_const_generics)]

mod comps;
pub mod traits;

use std::fmt;
use std::iter;
use std::ops;

use crate::traits::*;

/// Represents a constant-size, *n*-dimensional vector.
///
/// See the [crate root][crate] for usage examples.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vector<T, const N: usize> {
    arr: [T; N],
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for Vector<T, N> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.arr, f)
    }
}

impl<T, const N: usize> ops::Deref for Vector<T, N> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.arr
    }
}

impl<T, const N: usize> ops::DerefMut for Vector<T, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.arr
    }
}

////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////

impl<T: Num, const N: usize> Default for Vector<T, N> {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    #[inline]
    fn from(arr: [T; N]) -> Self {
        Self { arr }
    }
}

impl<T: Num, const M: usize, const N: usize> FromPartial<T, [T; M]> for Vector<T, N> {
    #[inline]
    fn from_partial(arr: [T; M], fill: T) -> Self {
        arr.iter().copied().chain(iter::repeat(fill)).collect()
    }
}

impl<'a, T: Num, const N: usize> From<&'a [T]> for Vector<T, N> {
    #[inline]
    fn from(slice: &'a [T]) -> Self {
        slice.iter().copied().collect()
    }
}

impl<'a, T: Num, const N: usize> FromPartial<T, &'a [T]> for Vector<T, N> {
    #[inline]
    fn from_partial(slice: &'a [T], fill: T) -> Self {
        slice.iter().copied().chain(iter::repeat(fill)).collect()
    }
}

impl<T: Num, const N: usize> From<Vec<T>> for Vector<T, N> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        vec.into_iter().collect()
    }
}

impl<T: Num, const N: usize> FromPartial<T, Vec<T>> for Vector<T, N> {
    #[inline]
    fn from_partial(vec: Vec<T>, fill: T) -> Self {
        vec.into_iter().chain(iter::repeat(fill)).collect()
    }
}

macro_rules! impl_from_tuple {
    ($({ $N:literal: ($($n:ident: $T:ident,)+) },)+) => {$(
        impl<T> From<($($T,)+)> for Vector<T, $N> {
            #[inline]
            fn from(($($n,)+): ($($T,)+)) -> Self {
                Self::from([$($n,)+])
            }
        }

        impl<T: Num, const N: usize> FromPartial<T, ($($T,)+)> for Vector<T, N> {
            #[inline]
            fn from_partial(($($n,)+): ($($T,)+), fill: T) -> Self {
                FromPartial::from_partial([$($n,)+], fill)
            }
        }
    )+}
}

impl_from_tuple! {
    {  1: (x: T,) },
    {  2: (x: T, y: T,) },
    {  3: (x: T, y: T, z: T,) },
    {  4: (x: T, y: T, z: T, w: T,) },
    {  5: (x: T, y: T, z: T, w: T, a: T,) },
    {  6: (x: T, y: T, z: T, w: T, a: T, b: T,) },
    {  7: (x: T, y: T, z: T, w: T, a: T, b: T, c: T,) },
    {  8: (x: T, y: T, z: T, w: T, a: T, b: T, c: T, d: T,) },
    {  9: (x: T, y: T, z: T, w: T, a: T, b: T, c: T, d: T, e: T,) },
    { 10: (x: T, y: T, z: T, w: T, a: T, b: T, c: T, d: T, e: T, f: T,) },
    { 11: (x: T, y: T, z: T, w: T, a: T, b: T, c: T, d: T, e: T, f: T, g: T,) },
    { 12: (x: T, y: T, z: T, w: T, a: T, b: T, c: T, d: T, e: T, f: T, g: T, h: T,) },
}

////////////////////////////////////////////////////////////////////////////////
// Operators
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_add {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl<T: Num, const N: usize> ops::Add<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn add(self, other: $rhs) -> Self::Output {
                let mut vector = Vector::default();
                for i in 0..N {
                    vector[i] = self[i] + other[i];
                }
                vector
            }
        }
    };
}

macro_rules! impl_sub {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl<T: Num, const N: usize> ops::Sub<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn sub(self, other: $rhs) -> Self::Output {
                let mut vector = Vector::default();
                for i in 0..N {
                    vector[i] = self[i] - other[i];
                }
                vector
            }
        }
    };
}

macro_rules! impl_mul {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl<'a, T: Num, const N: usize> ops::Mul<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn mul(self, other: $rhs) -> Self::Output {
                let mut vector = Vector::default();
                for i in 0..N {
                    vector[i] = self[i] * other;
                }
                vector
            }
        }
    };
}

macro_rules! impl_add_assign {
    ($self:ty, $other:ty) => {
        impl<T: Num, const N: usize> ops::AddAssign<$other> for $self {
            #[inline]
            fn add_assign(&mut self, other: $other) {
                for i in 0..N {
                    self[i] += other[i]
                }
            }
        }
    };
}

macro_rules! impl_sub_assign {
    ($self:ty, $other:ty) => {
        impl<T: Num, const N: usize> ops::SubAssign<$other> for $self {
            #[inline]
            fn sub_assign(&mut self, other: $other) {
                for i in 0..N {
                    self[i] -= other[i]
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
/// This `struct` is created by the `.into_iter()` method on [`Vector`]
/// (provided by the [`IntoIterator`] trait).
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
pub struct IntoIter<T: Num, const N: usize> {
    vector: Vector<T, N>,
    left: usize,
    right: usize,
}

impl<T: Num, const N: usize> IntoIter<T, N> {
    #[inline]
    fn new(vector: Vector<T, N>) -> Self {
        Self {
            vector,
            left: 0,
            right: vector.len(),
        }
    }
}

impl<T: Num, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.left == self.right {
            None
        } else {
            let next = unsafe { self.vector.get_unchecked(self.left) };
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

impl<T: Num, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.left == self.right {
            None
        } else {
            self.right -= 1;
            let next = unsafe { self.vector.get_unchecked(self.right) };
            Some(*next)
        }
    }
}

impl<T: Num, const N: usize> ExactSizeIterator for IntoIter<T, N> {}

impl<T: Num, const N: usize> iter::FusedIterator for IntoIter<T, N> {}

impl<T: Num, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<T: Num, const N: usize> iter::FromIterator<T> for Vector<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut vector = Vector::default();
        for i in 0..N {
            match iter.next() {
                Some(value) => vector[i] = value,
                None => {
                    panic!("collect iterator of length {} into `Vector<_, {}>`", i, N);
                }
            }
        }
        vector
    }
}

////////////////////////////////////////////////////////////////////////////////
// General methods
////////////////////////////////////////////////////////////////////////////////

impl<T: Num, const N: usize> Vector<T, N> {
    /// Create a new vector.
    pub const fn new(arr: [T; N]) -> Self {
        Self { arr }
    }

    /// Returns a zero vector.
    #[inline]
    pub fn zero() -> Self {
        let arr = [T::zero(); N];
        Self { arr }
    }

    /// Create a vector from various types, filling with zeroes as needed.
    pub fn from_partial<U>(partial: U) -> Self
    where
        Self: FromPartial<T, U>,
    {
        FromPartial::from_partial(partial, T::zero())
    }

    /// Create a vector from various types, filling with the given value as needed.
    pub fn from_partial_with<U>(partial: U, fill: T) -> Self
    where
        Self: FromPartial<T, U>,
    {
        FromPartial::from_partial(partial, fill)
    }

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

    /// Consumes this vector and returns the underlying array.
    #[inline]
    pub fn into_array(self) -> [T; N] {
        self.arr
    }

    /// Returns the absolute value of the vector.
    #[inline]
    pub fn abs(mut self) -> Self {
        for n in self.iter_mut() {
            *n = n.abs();
        }
        self
    }

    /// Returns the reduced row echelon form of the vector.
    ///
    /// This is the same as dividing each element by the greatest common divisor
    /// of all the elements.
    #[inline]
    pub fn reduced(self) -> Self {
        if self == Self::zero() {
            self
        } else {
            let div = self.into_iter().fold_first(gcd).unwrap();
            self.into_iter().map(|n| n / div).collect()
        }
    }

    /// Calculates the dot-product between `self` and `other`.
    #[inline]
    pub fn dot(&self, other: &Self) -> T {
        self.into_iter()
            .zip(other.into_iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    /// Returns the L1 norm of the vector.
    ///
    /// Also known as *Manhattan Distance* or *Taxicab norm*. L1 Norm is the sum
    /// of the magnitudes of the vectors in a space.
    #[inline]
    pub fn l1_norm(&self) -> T {
        self.abs().into_iter().sum()
    }
}

/// Returns the greatest common divisor of two numbers.
fn gcd<T: Num>(mut y: T, mut x: T) -> T {
    while x != T::zero() {
        let tmp = x;
        x = y % tmp;
        y = tmp;
    }
    y.abs()
}
