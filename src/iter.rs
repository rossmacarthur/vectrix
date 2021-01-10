use core::fmt;
use core::iter::*;
use core::ops::Range;

use crate::prelude::*;

/// An iterator that moves out of a vector.
///
/// This `struct` is created by the `.into_iter()` method on [`Vector`]
/// (provided by the [`IntoIterator`] trait).
///
/// # Examples
///
/// ```
/// # use vectrs::{vector, IntoIter};
/// #
/// let v = vector![0, 1, 2];
/// let iter: IntoIter<_, 3> = v.into_iter();
/// ```
#[derive(Clone)]
pub struct IntoIter<T, const N: usize> {
    vector: Vector<T, N>,
    alive: Range<usize>,
}

impl<T, const N: usize> IntoIter<T, N> {
    /// Creates a new iterator over the given vector.
    fn new(vector: Vector<T, N>) -> Self {
        Self {
            vector,
            alive: 0..N,
        }
    }

    fn as_slice(&self) -> &[T] {
        unsafe { self.vector.array.get_unchecked(self.alive.clone()) }
    }
}

impl<T: Copy, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Get the next index from the front.
        self.alive
            .next()
            .map(|idx| *unsafe { self.vector.array.get_unchecked(idx) })
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

impl<T: Copy, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // Get the next index from the back.
        self.alive
            .next_back()
            .map(|idx| *unsafe { self.vector.array.get_unchecked(idx) })
    }
}

impl<T: Copy, const N: usize> ExactSizeIterator for IntoIter<T, N> {
    fn len(&self) -> usize {
        // Will never underflow due to the invariant `alive.start <= alive.end`.
        self.alive.end - self.alive.start
    }
}

impl<T: Copy, const N: usize> FusedIterator for IntoIter<T, N> {}

impl<T: fmt::Debug, const N: usize> fmt::Debug for IntoIter<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Only print the elements that were not yielded yet: we cannot
        // access the yielded elements anymore.
        f.debug_tuple("IntoIter").field(&self.as_slice()).finish()
    }
}

impl<T: Copy, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<T: Base, const N: usize> FromIterator<T> for Vector<T, N> {
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

impl<T: Base, const N: usize> Sum<Vector<T, N>> for Vector<T, N>
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
