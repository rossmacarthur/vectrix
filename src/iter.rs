use core::fmt;
use core::iter::*;
use core::marker::PhantomData;
use core::ops::Range;

use crate::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// T iteration
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
        self.alive.next().map(|i| {
            let slice = self.matrix.as_slice();
            *unsafe { slice.get_unchecked(i) }
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
        self.alive.next_back().map(|i| {
            let slice = self.matrix.as_slice();
            *unsafe { slice.get_unchecked(i) }
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

////////////////////////////////////////////////////////////////////////////////
// Immutable row/column iteration
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_view {
    (
        $(#[$meta:meta])*
        impl Iterator<Item = $Item:ident> for $View:ident
        where
            Dimension = $D:ident,
            Method = $meth:ident,
    ) => {
        $(#[$meta])*
        pub struct $View<'a, T, const M: usize, const N: usize> {
            matrix: &'a Matrix<T, M, N>,
            alive: Range<usize>,
        }

        impl<'a, T, const M: usize, const N: usize> $View<'a, T, M, N> {
            pub(crate) fn new(matrix: &'a Matrix<T, M, N>) -> Self {
                Self {
                    matrix,
                    alive: 0..$D,
                }
            }
        }

        impl<'a, T, const M: usize, const N: usize> Iterator for $View<'a, T, M, N> {
            type Item = &'a $Item<T, M, N>;

            fn next(&mut self) -> Option<Self::Item> {
                self.alive.next().map(|i| self.matrix.$meth(i))
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

        impl<T, const M: usize, const N: usize> DoubleEndedIterator for $View<'_, T, M, N> {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.alive.next_back().map(|i| self.matrix.$meth(i))
            }
        }

        impl<T, const M: usize, const N: usize> ExactSizeIterator for $View<'_, T, M, N> {
            fn len(&self) -> usize {
                self.alive.end - self.alive.start
            }
        }

        impl<T, const M: usize, const N: usize> FusedIterator for $View<'_, T, M, N> {}
    };
}

impl_view! {
    /// An iterator over rows in a matrix.
    impl Iterator<Item = Row> for IterRows
    where
        Dimension = M,
        Method = row,
}
impl_view! {
    /// An iterator over columns in a matrix.
    impl Iterator<Item = Column> for IterColumns
    where
        Dimension = N,
        Method = column,
}

////////////////////////////////////////////////////////////////////////////////
// Mutable row/column iteration
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_view_mut {
    (
        $(#[$meta:meta])*
        impl Iterator<Item = $Item:ident> for $View:ident
        where
            Dimension = $D:ident,
            Method = $meth:ident,
    ) => {
        $(#[$meta])*
        pub struct $View<'a, T, const M: usize, const N: usize> {
            // We need to use a raw pointer here because the compiler doesn't
            // know that we are yielding mutable references to *different* data
            // each time.
            matrix: *mut Matrix<T, M, N>,
            alive: Range<usize>,
            marker: PhantomData<&'a mut Matrix<T, M, N>>,
        }

        impl<'a, T, const M: usize, const N: usize> $View<'a, T, M, N> {
            pub(crate) fn new(matrix: &'a mut Matrix<T, M, N>) -> Self {
                Self {
                    matrix: matrix as *mut Matrix<T, M, N>,
                    alive: 0..$D,
                    marker: PhantomData,
                }
            }
        }

        impl<'a, T, const M: usize, const N: usize> Iterator for $View<'a, T, M, N> {
            type Item = &'a mut $Item<T, M, N>;

            fn next(&mut self) -> Option<Self::Item> {
                self.alive.next().map(|i| {
                    // Safety: we yield a different row/column each time and
                    // `self.matrix`'s lifetime is asserted by the `PhantomData`.
                    unsafe { (*self.matrix).$meth(i) }
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

        impl<T, const M: usize, const N: usize> DoubleEndedIterator for $View<'_, T, M, N> {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.alive.next_back().map(|i| {
                    // Safety: we yield a different row/column each time and
                    // `self.matrix`'s lifetime is asserted by the `PhantomData`.
                    unsafe { (*self.matrix).$meth(i) }
                })
            }
        }

        impl<T, const M: usize, const N: usize> ExactSizeIterator for $View<'_, T, M, N> {
            fn len(&self) -> usize {
                self.alive.end - self.alive.start
            }
        }

        impl<T, const M: usize, const N: usize> FusedIterator for $View<'_, T, M, N> {}
    };
}

impl_view_mut! {
    /// A mutable iterator over rows in a matrix.
    impl Iterator<Item = Row> for IterRowsMut
    where
        Dimension = M,
        Method = row_mut,
}
impl_view_mut! {
    /// A mutable iterator over columns in a matrix.
    impl Iterator<Item = Column> for IterColumnsMut
    where
        Dimension = N,
        Method = column_mut,
}
