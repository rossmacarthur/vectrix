use core::iter::*;
use core::slice;

use crate::Stride;

/// Immutable stride iterator.
///
/// This struct is created by the [`iter()`][`Stride::iter()`] method on
/// strided slices.
#[derive(Debug, Clone)]
pub struct Iter<'a, T, const S: usize> {
    iter: StepBy<slice::Iter<'a, T>>,
}

/// Mutable stride iterator.
///
/// This struct is created by the [`iter_mut()`][`Stride::iter_mut()`] method on
/// strided slices.
#[derive(Debug)]
pub struct IterMut<'a, T, const S: usize> {
    iter: StepBy<slice::IterMut<'a, T>>,
}

macro_rules! impl_iterator {
    (
        with $Stride:ty, $meth:ident
        impl Iterator<Item = $Item:ty> for $Iter:ty
    ) => {
        impl<'a, T, const S: usize> $Iter {
            pub(crate) fn new(stride: $Stride) -> Self {
                Self {
                    iter: stride.data.$meth().step_by(S),
                }
            }
        }

        impl<'a, T, const S: usize> Iterator for $Iter {
            type Item = $Item;

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

        impl<'a, T, const S: usize> DoubleEndedIterator for $Iter {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.iter.next_back()
            }
        }

        impl<'a, T, const S: usize> ExactSizeIterator for $Iter {
            fn len(&self) -> usize {
                self.iter.len()
            }
        }

        impl<'a, T, const S: usize> FusedIterator for $Iter {}

        impl<'a, T, const S: usize> IntoIterator for $Stride {
            type Item = $Item;
            type IntoIter = $Iter;

            fn into_iter(self) -> $Iter {
                self.$meth()
            }
        }
    };
}

impl_iterator! {
    with &'a Stride<T, S>, iter
    impl Iterator<Item = &'a T> for Iter<'a, T, S>
}

impl_iterator! {
    with &'a mut Stride<T, S>, iter_mut
    impl Iterator<Item = &'a mut T> for IterMut<'a, T, S>
}
