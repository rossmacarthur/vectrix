#![feature(min_const_generics)]

mod comps;
mod ops;
mod prelude;
mod traits;

use crate::prelude::Zero;

/// Represents an n-dimensional vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vector<T, const N: usize> {
    inner: [T; N],
}

impl<T: Copy + Zero, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        let inner = [Zero::zero(); N];
        Self { inner }
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(arr: [T; N]) -> Self {
        Self { inner: arr }
    }
}

macro_rules! impl_from_tuple {
    ($tuple:ty, $N:literal, $($n:ident),+) => {
        impl<T> From<$tuple> for Vector<T, $N> {
            fn from(($($n,)+): $tuple) -> Self {
                Self::from([$($n,)+])
            }
        }
    };
}

impl_from_tuple!((T, T), 2, x, y);
impl_from_tuple!((T, T, T), 3, x, y, z);
impl_from_tuple!((T, T, T, T), 4, x, y, z, w);
