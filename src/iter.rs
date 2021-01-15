use core::fmt;
use core::iter::*;
use core::ops::Range;

use crate::prelude::*;

/// An iterator that moves out of a matrix.
///
/// This `struct` is created by the `.into_iter()` method on [`Matrix`]
/// (provided by the [`IntoIterator`] trait).
///
/// # Examples
///
/// ```
/// # use vectrix::{matrix, IntoIter};
/// #
/// let m = matrix![
///     1, 3, 5;
///     2, 4, 6;
/// ];
/// let iter: IntoIter<_, 2, 3> = m.into_iter();
/// ```
#[derive(Clone)]
pub struct IntoIter<T, const M: usize, const N: usize> {
    matrix: Matrix<T, M, N>,
    alive: Range<usize>,
}

impl<T, const M: usize, const N: usize> fmt::Debug for IntoIter<T, M, N>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.as_slice()).finish()
    }
}

impl<T, const M: usize, const N: usize> IntoIter<T, M, N> {
    /// Creates a new iterator over the given matrix.
    fn new(matrix: Matrix<T, M, N>) -> Self {
        Self {
            matrix,
            alive: 0..(M * N),
        }
    }

    fn as_slice(&self) -> &[T] {
        let slice = self.matrix.as_slice();
        unsafe { slice.get_unchecked(self.alive.clone()) }
    }
}

impl<T, const M: usize, const N: usize> Iterator for IntoIter<T, M, N>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Get the next index from the front.
        self.alive.next().map(|idx| {
            let slice = self.matrix.as_slice();
            *unsafe { slice.get_unchecked(idx) }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<T, const M: usize, const N: usize> DoubleEndedIterator for IntoIter<T, M, N>
where
    T: Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        // Get the next index from the back.
        self.alive.next_back().map(|idx| {
            let slice = self.matrix.as_slice();
            *unsafe { slice.get_unchecked(idx) }
        })
    }
}

impl<T, const M: usize, const N: usize> ExactSizeIterator for IntoIter<T, M, N>
where
    T: Copy,
{
    fn len(&self) -> usize {
        // Will never underflow due to the invariant `alive.start <= alive.end`.
        self.alive.end - self.alive.start
    }
}

impl<T, const M: usize, const N: usize> FusedIterator for IntoIter<T, M, N> where T: Copy {}

impl<T, const M: usize, const N: usize> IntoIterator for Matrix<T, M, N>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = IntoIter<T, M, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<T, const M: usize, const N: usize> Sum<Matrix<T, M, N>> for Matrix<T, M, N>
where
    Self: Add<Output = Self>,
    T: Copy + Zero,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Matrix::zero(), Add::add)
    }
}
