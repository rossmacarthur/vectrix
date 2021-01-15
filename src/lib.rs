//! This crate provides a stack-allocated, constant-size [`Matrix<T, M, N>`]
//! type.
//!
//! # Constructors
//!
//! ### `matrix!` macro
//!
//! Simply use the [`matrix!`] macro to construct a new [`Matrix`] of any size.
//!
//! ```
//! # use vectrix::{matrix, Matrix};
//! #
//! let m = matrix![
//!     1, 3, 5;
//!     2, 4, 6;
//! ];
//! ```
//!
//! In the above example `m` is a `Matrix<_, 2, 3>` type, having 2 rows and 3
//! columns. The `matrix!` macro will also work in `const` contexts.
//!
//! ### Component accessors
//!
//! Component accessors are available for small vectors using commonly
//! recognized names.
//! ```
//! # use vectrix::matrix;
//! #
//! let mut vector = matrix![1, 2, 3, 4];
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
//! [`.as_slice()`][`Matrix::as_slice`] and
//! [`.as_mut_slice()`][`Matrix::as_mut_slice`].
//! ```
//! # use vectrix::matrix;
//! #
//! let mut m = matrix![
//!     1, 3, 5;
//!     2, 3, 6;
//! ];
//! m.as_mut_slice()[3] = 4;
//! assert_eq!(m.as_slice(), &[1, 2, 3, 4, 5, 6]);
//! ```

#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod comps;
mod iter;
mod ops;
mod prelude;
pub mod traits;

use core::slice;

#[doc(hidden)]
pub use vectrix_macro as proc_macro;

pub use crate::iter::IntoIter;
use crate::prelude::*;

/// Represents a matrix with constant `M` rows and constant `N` columns.
///
/// The underlying data is represented as an array and is always stored in
/// column-major order.
///
/// See the [crate root][crate] for usage examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: [[T; M]; N],
}

////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////

/// A macro for composing matrices.
///
/// This macro allows one to write such a matrix in the natural order. For
/// example:
///
/// ```rust
/// # use vectrix::matrix;
/// #
/// let m = matrix![
///     1.0, 4.0;
///     2.0, 5.0;
///     3.0, 6.0;
/// ];
/// ```
///
/// corresponds to the following matrix with three rows and two columns:
///
/// ```text
/// ┌            ┐
/// │  1.0  4.0  │
/// │  2.0  5.0  │
/// │  3.0  6.0  │
/// └            ┘
/// ```
///
/// Which is stored as an array of arrays in column-major order.
///
/// ```text
/// Matrix { data: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]] }
/// ```
#[cfg(feature = "macro")]
#[macro_export]
macro_rules! matrix {
    ($($data:tt)*) => {
        $crate::Matrix::from_column_major_order($crate::proc_macro::matrix!($($data)*))
    };
}

impl<T, const M: usize, const N: usize> Default for Matrix<T, M, N>
where
    T: Copy + Default,
{
    /// Creates a new matrix using `T::default()` as an initializer.
    #[inline]
    fn default() -> Self {
        Self {
            data: [[T::default(); M]; N],
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// General methods
////////////////////////////////////////////////////////////////////////////////

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    /// Create a new matrix from an array of arrays in column-major order.
    #[doc(hidden)]
    pub const fn from_column_major_order(data: [[T; M]; N]) -> Self {
        Self { data }
    }

    /// Returns a zero vector.
    #[inline]
    #[must_use]
    pub fn zero() -> Self
    where
        T: Copy + Zero,
    {
        Self {
            data: [[T::zero(); M]; N],
        }
    }

    /// Views the underlying data as a contiguous slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        let ptr = self.data.as_ptr() as *const T;
        unsafe { slice::from_raw_parts(ptr, M * N) }
    }

    /// Views the underlying data as a contiguous mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let ptr = self.data.as_mut_ptr() as *mut T;
        unsafe { slice::from_raw_parts_mut(ptr, M * N) }
    }

    /// Returns an iterator over the underlying data.
    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    /// Returns an iterator over the data that allows modifying each value.
    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    /// Returns a matrix of the same size as self, with function `f` applied to
    /// each element in order.
    #[inline]
    fn map<F, U>(self, mut f: F) -> Matrix<U, M, N>
    where
        T: Copy,
        U: Copy + Default,
        F: FnMut(T) -> U,
    {
        let mut vector = Matrix::default();
        for i in 0..(M * N) {
            vector[i] = f(self[i]);
        }
        vector
    }

    /// Returns the absolute value of the matrix.
    #[inline]
    pub fn abs(self) -> Self
    where
        T: Copy + Default + Abs,
    {
        self.map(Abs::abs)
    }
}
