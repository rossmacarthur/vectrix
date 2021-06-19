//! Generic constructors.

use core::iter::FromIterator;

use crate::prelude::*;

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
    /// Create a new matrix using `T::default()` as an initializer.
    #[inline]
    fn default() -> Self {
        Self::repeat(T::default())
    }
}

impl<T, const M: usize, const N: usize> FromIterator<T> for Matrix<T, M, N>
where
    T: Copy + Default,
{
    /// Create a new matrix from an iterator. Elements will be filled in
    /// column-major order.
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        let mut matrix = Self::default();
        for i in 0..(M * N) {
            match iter.next() {
                Some(value) => matrix[i] = value,
                None => {
                    panic!(
                        "collect iterator of length {} into `Matrix<_, {}, {}>`",
                        i, M, N
                    );
                }
            }
        }
        matrix
    }
}
