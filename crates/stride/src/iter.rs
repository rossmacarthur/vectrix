use core::iter::*;
use core::slice;

use crate::Stride;

////////////////////////////////////////////////////////////////////////////////
// Immutable iteration
////////////////////////////////////////////////////////////////////////////////

/// Immutable stride iterator.
///
/// This struct is created by the [`iter()`][`Stride::iter()`] method on
/// strided slices.
#[derive(Debug, Clone)]
pub struct Iter<'a, T, const S: usize> {
    iter: StepBy<slice::Iter<'a, T>>,
}

impl<'a, T, const S: usize> Iter<'a, T, S> {
    pub(crate) fn new(stride: &'a Stride<T, S>) -> Self {
        Self {
            iter: stride.data.iter().step_by(S),
        }
    }
}

impl<'a, T, const S: usize> Iterator for Iter<'a, T, S> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
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

impl<'a, T, const S: usize> DoubleEndedIterator for Iter<'a, T, S> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, T, const S: usize> ExactSizeIterator for Iter<'a, T, S> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T, const S: usize> FusedIterator for Iter<'a, T, S> {}

impl<'a, T, const S: usize> IntoIterator for &'a Stride<T, S> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Mutable iteration
////////////////////////////////////////////////////////////////////////////////

/// Mutable stride iterator.
///
/// This struct is created by the [`iter_mut()`][`Stride::iter_mut()`] method on
/// strided slices.
#[derive(Debug)]
pub struct IterMut<'a, T, const S: usize> {
    iter: StepBy<slice::IterMut<'a, T>>,
}

impl<'a, T, const S: usize> IterMut<'a, T, S> {
    pub(crate) fn new(stride: &'a mut Stride<T, S>) -> Self {
        Self {
            iter: stride.data.iter_mut().step_by(S),
        }
    }
}

impl<'a, T, const S: usize> Iterator for IterMut<'a, T, S> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
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

impl<'a, T, const S: usize> DoubleEndedIterator for IterMut<'a, T, S> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, T, const S: usize> ExactSizeIterator for IterMut<'a, T, S> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T, const S: usize> FusedIterator for IterMut<'a, T, S> {}

impl<'a, T, const S: usize> IntoIterator for &'a mut Stride<T, S> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T, S>;

    fn into_iter(self) -> IterMut<'a, T, S> {
        self.iter_mut()
    }
}
