use core::cmp::{self, Ordering};
use core::hash::{Hash, Hasher};
use core::ops::*;

use crate::{Stride, StrideIndex};

impl<T, U, const S: usize, const R: usize> PartialEq<Stride<U, R>> for Stride<T, S>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Stride<U, R>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

fn partial_eq_slice<T, U, const S: usize>(stride: &Stride<T, S>, slice: &[U]) -> bool
where
    T: PartialEq<U>,
{
    stride.len() == slice.len() && stride.iter().zip(slice.iter()).all(|(a, b)| a == b)
}

impl<T, U, const S: usize, const N: usize> PartialEq<[U; N]> for Stride<T, S>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &[U; N]) -> bool {
        partial_eq_slice(self, other)
    }
}

impl<T, U, const S: usize, const N: usize> PartialEq<&[U; N]> for Stride<T, S>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&[U; N]) -> bool {
        partial_eq_slice(self, *other)
    }
}

impl<T, U, const S: usize> PartialEq<&[U]> for Stride<T, S>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&[U]) -> bool {
        partial_eq_slice(self, other)
    }
}

impl<T, U, const S: usize> PartialEq<[U]> for Stride<T, S>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &[U]) -> bool {
        partial_eq_slice(self, other)
    }
}

impl<T, const S: usize> Eq for Stride<T, S> where T: Eq {}

impl<T, const S: usize> Hash for Stride<T, S>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for element in self {
            element.hash(state);
        }
    }
}

impl<T, U, const S: usize, const R: usize> PartialOrd<Stride<U, R>> for Stride<T, S>
where
    T: PartialOrd<U>,
{
    fn partial_cmp(&self, other: &Stride<U, R>) -> Option<Ordering> {
        let len = cmp::min(self.len(), other.len());
        for i in 0..len {
            match self[i].partial_cmp(&other[i]) {
                Some(Ordering::Equal) => continue,
                non_eq => return non_eq,
            }
        }
        self.len().partial_cmp(&other.len())
    }
}

impl<T, const S: usize> Ord for Stride<T, S>
where
    T: Ord,
{
    fn cmp(&self, other: &Stride<T, S>) -> Ordering {
        let len = cmp::min(self.len(), other.len());
        for i in 0..len {
            match self[i].cmp(&other[i]) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            }
        }
        self.len().cmp(&other.len())
    }
}

impl<I, T, const S: usize> Index<I> for Stride<T, S>
where
    I: StrideIndex<Self>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        index.index(self)
    }
}

impl<I, T, const S: usize> IndexMut<I> for Stride<T, S>
where
    I: StrideIndex<Self>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_mut(self)
    }
}
