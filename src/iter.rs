use core::fmt;
use core::iter::{FusedIterator, Sum};
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ops::{Add, Range};
use core::ptr;

use crate::new;
use crate::{Column, Matrix, Row, Zero};

////////////////////////////////////////////////////////////////////////////////
// Element iteration
////////////////////////////////////////////////////////////////////////////////

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
pub struct IntoIter<T, const M: usize, const N: usize> {
    matrix: Matrix<MaybeUninit<T>, M, N>,
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
            // SAFETY: we know that `T` is the same size as `MaybeUninit<T>`.
            matrix: unsafe { new::transmute_unchecked(matrix) },
            alive: 0..(M * N),
        }
    }

    /// Returns the `i`-th element in the underlying matrix.
    ///
    /// # Safety
    ///
    /// The caller must make sure that `i` is only fetched once and that `i` is
    /// in the range `alive.start <= alive.end`.
    #[inline]
    unsafe fn get_unchecked(&self, i: usize) -> T {
        let ptr = unsafe { self.matrix.get_unchecked(i) }.as_ptr();
        unsafe { ptr::read(ptr) }
    }

    /// Returns a slice of the remaining initialized elements.
    #[inline]
    fn as_slice(&self) -> &[T] {
        let slice = &self.matrix.as_slice()[self.alive.clone()];
        let ptr = slice as *const [MaybeUninit<T>] as *const [T];
        // SAFETY: `alive` keeps track of the elements that are initialized.
        unsafe { &*ptr }
    }

    /// Returns a mutable slice of the remaining initialized elements.
    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        let slice = &mut self.matrix.as_mut_slice()[self.alive.clone()];
        let ptr = slice as *mut [MaybeUninit<T>] as *mut [T];
        // SAFETY: `alive` keeps track of the elements that are initialized.
        unsafe { &mut *ptr }
    }
}

impl<T, const M: usize, const N: usize> Iterator for IntoIter<T, M, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Get the next index from the front.
        self.alive.next().map(|i| {
            // SAFETY: `i` is an index into the former "alive" region of the
            // array. This is the only time `i` will be yielded .
            unsafe { self.get_unchecked(i) }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.alive.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.alive.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<T, const M: usize, const N: usize> DoubleEndedIterator for IntoIter<T, M, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // Get the next index from the back.
        self.alive.next_back().map(|i| {
            // SAFETY: `i` is an index into the former "alive" region of the
            // array. This is the only time `i` will be yielded .
            unsafe { self.get_unchecked(i) }
        })
    }
}

impl<T, const M: usize, const N: usize> ExactSizeIterator for IntoIter<T, M, N> {
    fn len(&self) -> usize {
        self.alive.len()
    }
}

impl<T, const M: usize, const N: usize> FusedIterator for IntoIter<T, M, N> {}

impl<T, const M: usize, const N: usize> IntoIterator for Matrix<T, M, N> {
    type Item = T;
    type IntoIter = IntoIter<T, M, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<T, const M: usize, const N: usize> Clone for IntoIter<T, M, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        // Note, we don't really need to match the exact same alive range, so
        // we can just clone into offset 0 regardless of where `self` is.
        let mut new = Self {
            matrix: Matrix::uninit(),
            alive: 0..0,
        };
        // Clone the alive elements only.
        for (src, dst) in self.as_slice().iter().zip(new.matrix.as_mut_slice()) {
            // Write a clone into the new array, then update its alive range.
            // If cloning panics, we'll correctly drop the previous items.
            *dst = MaybeUninit::new(src.clone());
            new.alive.end += 1;
        }
        new
    }
}

impl<T, const M: usize, const N: usize> Drop for IntoIter<T, M, N> {
    fn drop(&mut self) {
        let slice = self.as_mut_slice();
        // SAFETY: `slice` contains only initialized elements.
        unsafe { ptr::drop_in_place(slice) }
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

////////////////////////////////////////////////////////////////////////////////
// Immutable row iteration
////////////////////////////////////////////////////////////////////////////////

/// An iterator over the rows in a matrix.
pub struct IterRows<'a, T, const M: usize, const N: usize> {
    matrix: &'a Matrix<T, M, N>,
    alive: Range<usize>,
}

impl<'a, T, const M: usize, const N: usize> IterRows<'a, T, M, N> {
    pub(crate) fn new(matrix: &'a Matrix<T, M, N>) -> Self {
        Self {
            matrix,
            alive: 0..M,
        }
    }
}

impl<'a, T, const M: usize, const N: usize> Iterator for IterRows<'a, T, M, N> {
    type Item = &'a Row<T, M, N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.alive.next().map(|i| self.matrix.row(i))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.alive.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.alive.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<T, const M: usize, const N: usize> DoubleEndedIterator for IterRows<'_, T, M, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.alive.next_back().map(|i| self.matrix.row(i))
    }
}

impl<T, const M: usize, const N: usize> ExactSizeIterator for IterRows<'_, T, M, N> {
    fn len(&self) -> usize {
        self.alive.len()
    }
}

impl<T, const M: usize, const N: usize> FusedIterator for IterRows<'_, T, M, N> {}

////////////////////////////////////////////////////////////////////////////////
// Immutable column iteration
////////////////////////////////////////////////////////////////////////////////

/// An iterator over the columns in a matrix.
pub struct IterColumns<'a, T, const M: usize, const N: usize> {
    matrix: &'a Matrix<T, M, N>,
    alive: Range<usize>,
}

impl<'a, T, const M: usize, const N: usize> IterColumns<'a, T, M, N> {
    pub(crate) fn new(matrix: &'a Matrix<T, M, N>) -> Self {
        Self {
            matrix,
            alive: 0..N,
        }
    }
}

impl<'a, T, const M: usize, const N: usize> Iterator for IterColumns<'a, T, M, N> {
    type Item = &'a Column<T, M, N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.alive.next().map(|i| self.matrix.column(i))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.alive.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.alive.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<T, const M: usize, const N: usize> DoubleEndedIterator for IterColumns<'_, T, M, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.alive.next_back().map(|i| self.matrix.column(i))
    }
}

impl<T, const M: usize, const N: usize> ExactSizeIterator for IterColumns<'_, T, M, N> {
    fn len(&self) -> usize {
        self.alive.len()
    }
}

impl<T, const M: usize, const N: usize> FusedIterator for IterColumns<'_, T, M, N> {}

////////////////////////////////////////////////////////////////////////////////
// Mutable row iteration
////////////////////////////////////////////////////////////////////////////////

/// A mutable iterator over the rows in a matrix.
pub struct IterRowsMut<'a, T, const M: usize, const N: usize> {
    // We need to use a raw pointer here because the compiler doesn't
    // know that we are yielding mutable references to *different* data
    // each time.
    matrix: *mut Matrix<T, M, N>,
    alive: Range<usize>,
    marker: PhantomData<&'a mut Matrix<T, M, N>>,
}

impl<'a, T, const M: usize, const N: usize> IterRowsMut<'a, T, M, N> {
    pub(crate) fn new(matrix: &'a mut Matrix<T, M, N>) -> Self {
        Self {
            matrix: matrix as *mut Matrix<T, M, N>,
            alive: 0..M,
            marker: PhantomData,
        }
    }
}

impl<'a, T, const M: usize, const N: usize> Iterator for IterRowsMut<'a, T, M, N> {
    type Item = &'a mut Row<T, M, N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.alive.next().map(|i| {
            // SAFETY: we yield a different row each time and `self.matrix`'s
            // lifetime is asserted by the `PhantomData`.
            unsafe { (*self.matrix).row_mut(i) }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.alive.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.alive.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<T, const M: usize, const N: usize> DoubleEndedIterator for IterRowsMut<'_, T, M, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.alive.next_back().map(|i| {
            // SAFETY: we yield a different row each time and `self.matrix`'s
            // lifetime is asserted by the `PhantomData`.
            unsafe { (*self.matrix).row_mut(i) }
        })
    }
}

impl<T, const M: usize, const N: usize> ExactSizeIterator for IterRowsMut<'_, T, M, N> {
    fn len(&self) -> usize {
        self.alive.len()
    }
}

impl<T, const M: usize, const N: usize> FusedIterator for IterRowsMut<'_, T, M, N> {}

////////////////////////////////////////////////////////////////////////////////
// Mutable column iteration
////////////////////////////////////////////////////////////////////////////////

/// A mutable iterator over the columns in a matrix.
pub struct IterColumnsMut<'a, T, const M: usize, const N: usize> {
    // We need to use a raw pointer here because the compiler doesn't
    // know that we are yielding mutable references to *different* data
    // each time.
    matrix: *mut Matrix<T, M, N>,
    alive: Range<usize>,
    marker: PhantomData<&'a mut Matrix<T, M, N>>,
}

impl<'a, T, const M: usize, const N: usize> IterColumnsMut<'a, T, M, N> {
    pub(crate) fn new(matrix: &'a mut Matrix<T, M, N>) -> Self {
        Self {
            matrix: matrix as *mut Matrix<T, M, N>,
            alive: 0..N,
            marker: PhantomData,
        }
    }
}

impl<'a, T, const M: usize, const N: usize> Iterator for IterColumnsMut<'a, T, M, N> {
    type Item = &'a mut Column<T, M, N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.alive.next().map(|i| {
            // SAFETY: we yield a different column each time and `self.matrix`'s
            // lifetime is asserted by the `PhantomData`.
            unsafe { (*self.matrix).column_mut(i) }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.alive.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.alive.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<T, const M: usize, const N: usize> DoubleEndedIterator for IterColumnsMut<'_, T, M, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.alive.next_back().map(|i| {
            // SAFETY: we yield a different column each time and `self.matrix`'s
            // lifetime is asserted by the `PhantomData`.
            unsafe { (*self.matrix).column_mut(i) }
        })
    }
}

impl<T, const M: usize, const N: usize> ExactSizeIterator for IterColumnsMut<'_, T, M, N> {
    fn len(&self) -> usize {
        self.alive.len()
    }
}

impl<T, const M: usize, const N: usize> FusedIterator for IterColumnsMut<'_, T, M, N> {}
