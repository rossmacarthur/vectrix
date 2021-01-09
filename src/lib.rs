//! This crate provides a stack-allocated, constant-size, *n*-dimensional
//! [`Vector<T, N>`] type.
//!
//! # Constructors
//!
//! ### `vector!` macro
//!
//! Simply use the [`vector!`] macro to construct a new [`Vector`] of any size.
//!
//! ```
//! # use vectrs::vector;
//! #
//! let v = vector![1, 3, 3, 7];
//! //  ^ constructs a `Vector<_, 4>`.
//! ```
//!
//! ### Directly using `new()` or `From`
//!
//! A [`Vector`] is backed by an array. You can construct a [`Vector`] from an
//! array or tuple. In both of these cases the size of the `Vector` can be
//! easily inferred by the Rust type system.
//!
//! From an array:
//! ```
//! # use vectrs::Vector;
//! #
//! let v = Vector::new([1, 2, 3, 4]);
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
//! assert_eq!(v, Vector::new([1, 2, 0]));
//!
//! let v = Vector::<_, 5>::from_partial_with((3, 2, 1), 1);
//! assert_eq!(v, Vector::new([3, 2, 1, 1, 1]));      // ^ fill value
//! ```
//!
//! # Accessing and mutating data
//!
//! ### Indexing
//!
//! Data can be accessed and mutated using indexing.
//! ```
//! # use vectrs::vector;
//! #
//! let mut vector = vector![1, 2, 3, 4];
//! vector[1] = 3;
//! vector[3] = 7;
//! assert_eq!(vector[0], 1);
//! assert_eq!(vector[1], 3);
//! assert_eq!(vector[2], 3);
//! assert_eq!(vector[3], 7);
//! ```
//!
//! ### Component accessors
//!
//! Component accessors are available for small vectors using commonly
//! recognized names.
//! ```
//! # use vectrs::vector;
//! #
//! let mut vector = vector![1, 2, 3, 4];
//! vector.y = 3;
//! vector.w = 7;
//! assert_eq!(vector.x, 1);
//! assert_eq!(vector.y, 3);
//! assert_eq!(vector.z, 3);
//! assert_eq!(vector.w, 7);
//! ```
//!
//! ### Slice representation
//!
//! A slice view of the underlying data is provided using
//! [`.as_slice()`][Vector::as_slice] and
//! [`.as_mut_slice()`][Vector::as_mut_slice].
//! ```
//! # use vectrs::vector;
//! #
//! let mut vector = vector![1, 3, 3, 4];
//! vector.as_mut_slice()[3] = 7;
//! assert_eq!(vector.as_slice(), &[1, 3, 3, 7]);
//! ```

#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod comps;
mod ops;
mod prelude;
pub mod traits;

use core::fmt;
use core::iter;
use core::slice;

use crate::prelude::*;

/// Represents a constant-size, *n*-dimensional vector.
///
/// See the [crate root][crate] for usage examples.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct Vector<T, const N: usize> {
    arr: [T; N],
}

////////////////////////////////////////////////////////////////////////////////
// Formatting
////////////////////////////////////////////////////////////////////////////////

impl<T: Debug, const N: usize> Debug for Vector<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.arr, f)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////

impl<T: Base, const N: usize> Default for Vector<T, N> {
    #[inline]
    fn default() -> Self {
        let arr = [T::default(); N];
        Self { arr }
    }
}
// `From` implementations

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    #[inline]
    fn from(arr: [T; N]) -> Self {
        Self { arr }
    }
}

impl<'a, T: Base, const N: usize> From<&'a [T]> for Vector<T, N> {
    #[inline]
    fn from(slice: &'a [T]) -> Self {
        slice.iter().copied().collect()
    }
}

#[cfg(feature = "std")]
impl<T: Base, const N: usize> From<Vec<T>> for Vector<T, N> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        vec.into_iter().collect()
    }
}

// `FromPartial` implementations

impl<T: Base, const M: usize, const N: usize> FromPartial<T, [T; M]> for Vector<T, N> {
    #[inline]
    fn from_partial(arr: [T; M], fill: T) -> Self {
        arr.iter().copied().chain(iter::repeat(fill)).collect()
    }
}

impl<T: Base, const M: usize, const N: usize> FromPartial<T, Vector<T, M>> for Vector<T, N> {
    #[inline]
    fn from_partial(vector: Vector<T, M>, fill: T) -> Self {
        vector.into_iter().chain(iter::repeat(fill)).collect()
    }
}

impl<'a, T: Base, const N: usize> FromPartial<T, &'a [T]> for Vector<T, N> {
    #[inline]
    fn from_partial(slice: &'a [T], fill: T) -> Self {
        slice.iter().copied().chain(iter::repeat(fill)).collect()
    }
}

#[cfg(feature = "std")]
impl<T: Base, const N: usize> FromPartial<T, Vec<T>> for Vector<T, N> {
    #[inline]
    fn from_partial(vec: Vec<T>, fill: T) -> Self {
        vec.into_iter().chain(iter::repeat(fill)).collect()
    }
}

// `From` and `FromPartial` implementations for tuples

macro_rules! impl_from_tuple {
    ($({ $N:literal: ($($n:ident: $T:ident,)+) },)+) => {$(
        impl<T: Base> From<($($T,)+)> for Vector<T, $N> {
            #[inline]
            fn from(($($n,)+): ($($T,)+)) -> Self {
                Self::from([$($n,)+])
            }
        }

        impl<T: Base, const N: usize> FromPartial<T, ($($T,)+)> for Vector<T, N> {
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
pub struct IntoIter<T, const N: usize> {
    left: usize,
    right: usize,
    vector: Vector<T, N>,
}

impl<T, const N: usize> IntoIter<T, N> {
    #[inline]
    fn new(vector: Vector<T, N>) -> Self {
        Self {
            left: 0,
            right: vector.arr.len(),
            vector,
        }
    }
}

impl<T: Base, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.left == self.right {
            None
        } else {
            let next = unsafe { self.vector.arr.get_unchecked(self.left) };
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

impl<T: Base, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.left == self.right {
            None
        } else {
            self.right -= 1;
            let next = unsafe { self.vector.arr.get_unchecked(self.right) };
            Some(*next)
        }
    }
}

impl<T: Base, const N: usize> ExactSizeIterator for IntoIter<T, N> {}

impl<T: Base, const N: usize> iter::FusedIterator for IntoIter<T, N> {}

impl<T: Base, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<T: Base, const N: usize> iter::FromIterator<T> for Vector<T, N> {
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

impl<T: Base, const N: usize> iter::Sum<Vector<T, N>> for Vector<T, N>
where
    Self: Add<Output = Self>,
    T: Zero,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Vector::zero(), Add::add)
    }
}

////////////////////////////////////////////////////////////////////////////////
// General methods
////////////////////////////////////////////////////////////////////////////////

/// Construct a new [`Vector`] of any size.
#[macro_export]
macro_rules! vector {
    ( $elem:expr; $n:expr ) => {
        $crate::Vector::new([$elem; $n])
    };
    ( $($elem:expr),* $(,)? ) => {
        $crate::Vector::new([$($elem),*])
    };
}

impl<T, const N: usize> Vector<T, N> {
    /// Create a new vector.
    pub const fn new(arr: [T; N]) -> Self {
        Self { arr }
    }
}

impl<T: Base, const N: usize> Vector<T, N> {
    /// Returns a zero vector.
    #[inline]
    pub fn zero() -> Self
    where
        T: Zero,
    {
        let arr = [T::zero(); N];
        Self { arr }
    }

    /// Create a vector from various types, filling with zeroes as needed.
    pub fn from_partial<U>(partial: U) -> Self
    where
        Self: FromPartial<T, U>,
        T: Zero,
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
        &self.arr
    }

    /// Views the underlying vector representation as a mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.arr
    }

    /// Returns an iterator over the slice.
    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.arr.iter()
    }

    /// Returns an iterator that allows modifying each value.
    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.arr.iter_mut()
    }

    /// Consumes this vector and returns the underlying array.
    #[inline]
    pub fn into_array(self) -> [T; N] {
        self.arr
    }

    /// Returns a vector of the same size as self, with function `f` applied to
    /// each element in order.
    #[inline]
    pub fn map<F, U: Base>(self, mut f: F) -> Vector<U, N>
    where
        F: FnMut(T) -> U,
    {
        let mut vector = Vector::default();
        for i in 0..N {
            vector[i] = f(self[i]);
        }
        vector
    }

    /// Returns the absolute value of the vector.
    #[inline]
    pub fn abs(self) -> Self
    where
        T: Abs,
    {
        self.map(|n| n.abs())
    }

    /// Returns the reduced row echelon form of the vector.
    ///
    /// This is the same as dividing each element by the greatest common divisor
    /// of all the elements.
    #[inline]
    pub fn reduced(self) -> Self
    where
        T: PartialEq<T> + Div<Output = T> + Rem<Output = T> + Zero + Abs,
    {
        if self == Self::zero() {
            self
        } else {
            let div = fold_first(self.into_iter(), gcd).unwrap();
            self.into_iter().map(|n| n / div).collect()
        }
    }

    /// Calculates the dot-product between `self` and `other`.
    #[inline]
    pub fn dot(&self, other: &Self) -> T
    where
        T: Mul<Output = T> + Sum<T>,
    {
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
    pub fn l1_norm(&self) -> T
    where
        T: Abs + Sum<T>,
    {
        self.abs().into_iter().sum()
    }
}

/// Like rust-lang/rust#57563 but reimplemented so we can support stable Rust.
#[inline]
fn fold_first<I, T, F>(mut iter: I, f: F) -> Option<T>
where
    I: Iterator<Item = T>,
    F: FnMut(T, T) -> T,
{
    let first = iter.next()?;
    Some(iter.fold(first, f))
}

/// Returns the greatest common divisor of two numbers.
fn gcd<T>(mut y: T, mut x: T) -> T
where
    T: Copy + PartialEq<T> + Rem<Output = T> + Zero + Abs,
{
    while x != T::zero() {
        let tmp = x;
        x = y % tmp;
        y = tmp;
    }
    y.abs()
}
