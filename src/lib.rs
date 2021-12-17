//! This crate provides a stack-allocated, constant-size [`Matrix<T, M, N>`]
//! type implemented using const generics.
//!
//! ## 🚀 Getting started
//!
//! Add the following to your Cargo manifest.
//!
//! ```toml
//! [dependencies]
//! vectrix = "0.2"
//! ```
//!
//! `no_std` is also supported by disabling the default std feature.
//! ```toml
//! [dependencies]
//! vectrix = { version = "0.2", default-features = false, features = ["macro"] }
//! ```
//!
//! ## 🤸 Usage
//!
//! ### Types
//!
//! The base [`Matrix<T, M, N>`] type represents a matrix with `M` rows and `N`
//! columns. This type is a backed by an array of arrays. The data is stored in
//! column-major order. Some convenient aliases are provided for common
//! matrices, like vectors.
//!
//! - [`Matrix<T, M, N>`] → a generic matrix type with `M` rows and `N` columns.
//! - [`Vector<T, M>`] → a column vector with `M` rows.
//! - [`RowVector<T, N>`] → a row vector with `N` columns.
//!
//! ### Macros
//!
//! Macros are provided for easy construction of the provided types. These
//! macros will also work in `const` contexts.
//!
//! - The [`matrix!`] macro can be used to construct a new [`Matrix`] of any
//!   size.
//!   ```
//!   # use vectrix::*;
//!   #
//!   let m = matrix![
//!       1, 3, 5;
//!       2, 4, 6;
//!   ];
//!   ```
//!
//!   In the above example `matrix` is a `Matrix<_, 2, 3>` type, having 2 rows and
//!   3 columns.
//!
//! - The [`vector!`] and [`row_vector!`] macros can be used to to construct
//!   column and row vectors respectively.
//!   ```
//!   # use vectrix::*;
//!   #
//!   let v = vector![1, 3, 3, 7];
//!   //  ^ type `Vector<_, 4>`
//!   assert_eq!(v, matrix![1; 3; 3; 7]);
//!
//!   let v = row_vector![1, 3, 3, 7];
//!   //  ^^^^^^ type `RowVector<_, 4>`
//!   assert_eq!(v, matrix![1, 3, 3, 7]);
//!   ```
//!
//! ### Constructors
//!
//! Commonly used constructors are listed below.
//!
//! - [`::zero()`][`Matrix::zero()`] → constructs a new matrix filled with
//!   [`T::zero()`][`Zero::zero()`].
//! - [`::identity()`][`Matrix::identity()`] → constructs a new identity matrix.
//! - [`::repeat(..)`][`Matrix::repeat()`] → constructs a new matrix filled with
//!   the provided value.
//! - [`::repeat_with(..)`][`Matrix::repeat_with()`] → constructs a new matrix
//!   filled with values computed by the provided closure.
//! - [`::from_iter(..)`][`core::iter::FromIterator::from_iter`] → constructs a
//!   new matrix from an iterator.
//! - [`::new(..)`][`Matrix::new()`] → constructs a new vector using the
//!   provided components.
//!
//! ### Accessing elements
//!
//! Three types of element access are available.
//!
//! - `usize` indexing selects the nth element in the matrix as viewed in
//!    column-major order.
//!    ```
//!    # use vectrix::*;
//!    #
//!    let m = matrix![
//!        1, 2, 3;
//!        4, 5, 6;
//!    ];
//!    assert_eq!(m[1], 4);
//!    ```
//!
//! - `(usize, usize)` indexing selects the element at a particular row and
//!   column position.
//!   ```
//!   # use vectrix::*;
//!   #
//!   let m = matrix![
//!       1, 2, 3;
//!       4, 5, 6;
//!   ];
//!   assert_eq!(m[(1, 0)], 4);
//!   ```
//!
//! - Component accessors are available for small vectors using traditional
//!   names.
//!   ```
//!   # use vectrix::*;
//!   #
//!   let mut v = vector![1, 2, 3, 4, 0, 0];
//!   v.y = 3;
//!   v.w = 7;
//!   assert_eq!(v.x, 1);
//!   assert_eq!(v.y, 3);
//!   assert_eq!(v.z, 3);
//!   assert_eq!(v.w, 7);
//!   assert_eq!(v.a, 0);
//!   assert_eq!(v.b, 0);
//!   ```
//!
//! ### Accessing a row or column
//!
//! You can get a reference to particular row or column using the
//! [`.row()`][`Matrix::row`] or [`.column()`][`Matrix::column`] methods. You
//! can get a mutable reference using the `_mut` variants.
//!
//! ```
//! # use vectrix::*;
//! #
//! let mut m = matrix![
//!     1, 2, 3;
//!     4, 7, 6;
//! ];
//! let row = m.row_mut(1);
//! row[1] = 5;
//! assert_eq!(m.column(1), &[2, 5]);
//! ```
//!
//! ### Iteration
//!
//! Element-wise, column-major order iteration is provided using the following
//! methods.
//!
//! - [`.into_iter()`][`Matrix::into_iter()`] → consumes the matrix and returns
//!   an owned iterator over each element.
//! - [`.iter()`][`Matrix::iter()`] → returns an iterator over a reference to
//!   each element.
//! - [`.iter_mut()`][`Matrix::iter_mut()`] → returns an iterator over a mutable
//!   reference to each element.
//!
//! Iteration over rows and columns is provide using the following methods.
//!
//! - [`.iter_rows()`][`Matrix::iter_rows()`] → returns an iterator over a
//!   reference to each row.
//! - [`.iter_rows_mut()`][`Matrix::iter_rows_mut()`] → returns an iterator over
//!   mutable reference to each row.
//! - [`.iter_columns()`][`Matrix::iter_columns()`] → returns an iterator over a
//!   reference to each column.
//! - [`.iter_columns_mut()`][`Matrix::iter_columns_mut()`] → returns an
//!   iterator over a mutable reference to each column.
//!
//! ### Slice representation
//!
//! A slice view of the underlying data is provided using
//! [`.as_slice()`][`Matrix::as_slice`] and
//! [`.as_mut_slice()`][`Matrix::as_mut_slice`].
//! ```
//! # use vectrix::*;
//! #
//! let mut m = matrix![
//!     1, 3, 5;
//!     2, 3, 6;
//! ];
//! m.as_mut_slice()[3] = 4;
//! assert_eq!(m.as_slice(), &[1, 2, 3, 4, 5, 6]);
//! ```
//!
//! ### Debug
//!
//! The [`Debug`][`core::fmt::Debug`] implementation will print out vectors as
//! lists and matrices as a list of lists in column-major order.
//!
//! ```
//! # use vectrix::*;
//! #
//! let v = vector![1.1, 2.0];
//! let m = matrix![1, 2; 3, 4];
//! println!("vector: {:.2?}", v);
//! println!("matrix: {:?}", m);
//! ```
//!
//! This will output:
//!
//! ```text
//! vector: [1.10, 2.00]
//! matrix: [[1, 3], [2, 4]]
//! ```
//!
//! ### Display
//!
//! The [`Display`][`core::fmt::Display`] implementation will print out the
//! matrix in the traditional box bracket format. Precision is supported as well
//! as most of the other formatting traits like
//! [`LowerHex`][`core::fmt::LowerHex`].
//!
//! ```
//! # use vectrix::*;
//! #
//! let cv = vector![1.1, 2.0];
//! let rv = row_vector![1.1, 2.0];
//! let m = matrix![1, 2; 3, 4];
//! println!("column vector: {:.2}", cv);
//! println!("row vector: {:.1}", rv);
//! println!("matrix: {:b}", m);
//! ```
//!
//! This will output:
//!
//! ```text
//! column vector:
//!  ┌      ┐
//!  │ 1.10 │
//!  │ 2.00 │
//!  └      ┘
//!
//! row vector:
//!  ┌          ┐
//!  │ 1.1  2.0 │
//!  └          ┘
//!
//! matrix:
//!  ┌         ┐
//!  │  1   10 │
//!  │ 11  100 │
//!  └         ┘
//! ```
//!
//! ### Operations
//!
//! [`Matrix`] implements many built-in operators. With scalar operands almost
//! all operators are implemented and they simply apply the operation to each
//! element in the matrix. Unary operators will do the equivalent. In the
//! following example each element in the matrix is multiplied by 2.
//!
//! ```
//! # use vectrix::*;
//! #
//! let m = matrix![
//!     1, -3;
//!     3, -7;
//! ];
//! let exp = matrix![
//!     2, -6;
//!     6, -14;
//! ];
//! assert_eq!(m * 2, exp);
//! ```
//!
//! [`Matrix`] supports addition and subtraction with same size matrices for
//! element-wise addition and subtraction. In the following example a matrix
//! is added to itself.
//!
//! ```
//! # use vectrix::*;
//! #
//! let m = matrix![
//!     1, -3;
//!     3, -7;
//! ];
//! let exp = matrix![
//!     2, -6;
//!     6, -14;
//! ];
//! assert_eq!(m + m, exp);
//! ```

#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

#[cfg(feature = "std")]
extern crate std;

mod fmt;
mod index;
mod iter;
mod new;
mod ops;
mod traits;
mod vector;
mod view;

use core::iter::Sum;
use core::ops::*;
use core::slice;

#[doc(hidden)]
#[cfg(feature = "macro")]
pub use vectrix_macro as proc_macro;

pub use crate::index::MatrixIndex;
pub use crate::iter::{IntoIter, IterColumns, IterColumnsMut, IterRows, IterRowsMut};
pub use crate::traits::{Abs, One, Zero};
pub use crate::view::{Column, Row};

/// Represents a matrix with constant `M` rows and constant `N` columns.
///
/// The underlying data is represented as an array and is always stored in
/// column-major order.
///
/// See the [crate root][crate] for usage examples.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: [[T; M]; N],
}

/// A matrix with one row and `N` columns.
pub type RowVector<T, const N: usize> = Matrix<T, 1, N>;

/// A matrix with one column and `M` rows.
pub type Vector<T, const M: usize> = Matrix<T, M, 1>;

////////////////////////////////////////////////////////////////////////////////
// Matrix<T, M, N> methods
////////////////////////////////////////////////////////////////////////////////

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    /// Create a new matrix from an array of arrays in column-major order.
    #[doc(hidden)]
    #[inline]
    pub const fn from_column_major_order(data: [[T; M]; N]) -> Self {
        Self { data }
    }

    /// Returns a zero matrix.
    #[must_use]
    #[inline]
    pub fn zero() -> Self
    where
        T: Copy + Zero,
    {
        Self::repeat(T::zero())
    }

    /// Create a new matrix filled with the given element.
    #[must_use]
    #[inline]
    pub fn repeat(element: T) -> Self
    where
        T: Copy,
    {
        Self {
            data: [[element; M]; N],
        }
    }

    /// Create a new matrix filled with computed elements.
    ///
    /// Elements will be filled in column-major order.
    #[must_use]
    #[inline]
    pub fn repeat_with<F>(f: F) -> Self
    where
        F: FnMut() -> T,
    {
        // SAFETY: the iterator will yield forever.
        unsafe { new::collect_unchecked(core::iter::repeat_with(f)) }
    }

    /// Returns a raw pointer to the underlying data.
    #[inline]
    fn as_ptr(&self) -> *const T {
        self.data.as_ptr() as *const T
    }

    /// Returns an unsafe mutable pointer to the underlying data.
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr() as *mut T
    }

    /// Views the underlying data as a contiguous slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), M * N) }
    }

    /// Views the underlying data as a contiguous mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), M * N) }
    }

    /// Returns a reference to an element in the matrix or `None` if out of
    /// bounds.
    #[inline]
    pub fn get<I>(&self, i: I) -> Option<&I::Output>
    where
        I: MatrixIndex<Self>,
    {
        i.get(self)
    }

    /// Returns a mutable reference to an element in the matrix or `None` if out
    /// of bounds.
    #[inline]
    pub fn get_mut<I>(&mut self, i: I) -> Option<&mut I::Output>
    where
        I: MatrixIndex<Self>,
    {
        i.get_mut(self)
    }

    /// Returns a reference to an element in the matrix without doing any bounds
    /// checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is
    /// *[undefined behavior]* even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    pub unsafe fn get_unchecked<I>(&self, i: I) -> &I::Output
    where
        I: MatrixIndex<Self>,
    {
        unsafe { &*i.get_unchecked(self) }
    }

    /// Returns a mutable reference to an element in the matrix without doing
    /// any bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is
    /// *[undefined behavior]* even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    pub unsafe fn get_unchecked_mut<I>(&mut self, i: I) -> &mut I::Output
    where
        I: MatrixIndex<Self>,
    {
        unsafe { &mut *i.get_unchecked_mut(self) }
    }

    /// Returns a reference to the `i`-th row of this matrix.
    #[inline]
    pub fn row(&self, i: usize) -> &Row<T, M, N> {
        Row::new(&self.as_slice()[i..])
    }

    /// Returns a mutable reference to the `i`-th row of this matrix.
    #[inline]
    pub fn row_mut(&mut self, i: usize) -> &mut Row<T, M, N> {
        Row::new_mut(&mut self.as_mut_slice()[i..])
    }

    /// Returns a reference to the `i`-th column of this matrix.
    #[inline]
    pub fn column(&self, i: usize) -> &Column<T, M, N> {
        Column::new(&self.data[i])
    }

    /// Returns a mutable reference to the `i`-th column of this matrix.
    #[inline]
    pub fn column_mut(&mut self, i: usize) -> &mut Column<T, M, N> {
        Column::new_mut(&mut self.data[i])
    }

    /// Returns an iterator over the underlying data.
    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    /// Returns a mutable iterator over the underlying data.
    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    /// Returns an iterator over the rows in this matrix.
    #[inline]
    pub fn iter_rows(&self) -> IterRows<'_, T, M, N> {
        IterRows::new(self)
    }

    /// Returns a mutable iterator over the rows in this matrix.
    #[inline]
    pub fn iter_rows_mut(&mut self) -> IterRowsMut<'_, T, M, N> {
        IterRowsMut::new(self)
    }

    /// Returns an iterator over the columns in this matrix.
    #[inline]
    pub fn iter_columns(&self) -> IterColumns<'_, T, M, N> {
        IterColumns::new(self)
    }

    /// Returns a mutable iterator over the columns in this matrix.
    #[inline]
    pub fn iter_columns_mut(&mut self) -> IterColumnsMut<'_, T, M, N> {
        IterColumnsMut::new(self)
    }

    /// Returns a matrix of the same size as self, with function `f` applied to
    /// each element in column-major order.
    #[inline]
    pub fn map<F, U>(self, f: F) -> Matrix<U, M, N>
    where
        F: FnMut(T) -> U,
    {
        // SAFETY: the iterator has the exact number of elements required.
        unsafe { new::collect_unchecked(self.into_iter().map(f)) }
    }

    /// Returns the L1 norm of the matrix.
    ///
    /// Also known as *Manhattan Distance* or *Taxicab norm*. L1 Norm is the sum
    /// of the magnitudes of the vectors in a space.
    ///
    /// Note: if the matrix is a *row vector* this method might not do what you
    /// what you expect. For example:
    ///
    /// ```
    /// # use vectrix::matrix;
    /// #
    /// let row_vector = matrix![1, 2, 3];
    /// assert_eq!(row_vector.l1_norm(), 3);
    ///
    /// let column_vector = matrix![1; 2; 3];
    /// assert_eq!(column_vector.l1_norm(), 6);
    /// ```
    pub fn l1_norm(&self) -> T
    where
        T: Copy + Ord + Abs + Zero + Sum<T>,
    {
        (0..N)
            .map(|i| self.data[i].iter().copied().map(Abs::abs).sum())
            .max()
            .unwrap_or_else(Zero::zero)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix<T, N, N> methods
////////////////////////////////////////////////////////////////////////////////

impl<T, const N: usize> Matrix<T, N, N> {
    /// Returns an identity matrix.
    #[must_use]
    #[inline]
    pub fn identity() -> Self
    where
        T: Copy + One + Zero,
    {
        let mut matrix = Self::zero();
        for i in 0..N {
            matrix[(i, i)] = T::one();
        }
        matrix
    }

    /// Returns the diagonal of the matrix.
    pub fn diagonal(&self) -> Vector<T, N>
    where
        T: Copy + Zero,
    {
        let mut vector = Vector::zero();
        for i in 0..N {
            vector[i] = self[(i, i)];
        }
        vector
    }
}
