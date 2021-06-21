//! Row and column slices of a matrix.

use core::iter::Sum;
use core::ops::{Deref, DerefMut, Mul};

use stride::Stride;

/// A row in a [`Matrix`][crate::Matrix].
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Row<T, const M: usize, const N: usize> {
    data: Stride<T, M>,
}

/// A column in a [`Matrix`][crate::Matrix].
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Column<T, const M: usize, const N: usize> {
    data: Stride<T, 1>,
}

////////////////////////////////////////////////////////////////////////////////
// Operations
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_ops {
    ($ty:ident<$S:tt>) => {
        impl<T, const M: usize, const N: usize> Deref for $ty<T, M, N> {
            type Target = Stride<T, $S>;

            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }

        impl<T, const M: usize, const N: usize> DerefMut for $ty<T, M, N> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.data
            }
        }

        impl<T, U, const M: usize, const N: usize, const S: usize> PartialEq<Stride<U, S>>
            for $ty<T, M, N>
        where
            T: PartialEq<U>,
        {
            fn eq(&self, other: &Stride<U, S>) -> bool {
                self.data.eq(other)
            }
        }

        impl<T, U, const M: usize, const N: usize> PartialEq<[U]> for $ty<T, M, N>
        where
            T: PartialEq<U>,
        {
            fn eq(&self, other: &[U]) -> bool {
                self.data.eq(other)
            }
        }

        impl<T, U, const M: usize, const N: usize, const P: usize> PartialEq<[U; P]>
            for $ty<T, M, N>
        where
            T: PartialEq<U>,
        {
            fn eq(&self, other: &[U; P]) -> bool {
                self.data.eq(other)
            }
        }
    };
}

impl_ops! { Row<M> }
impl_ops! { Column<1> }

////////////////////////////////////////////////////////////////////////////////
// Methods
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_view {
    ($ty:ident) => {
        impl<T, const M: usize, const N: usize> $ty<T, M, N> {
            pub(crate) fn new(data: &[T]) -> &Self {
                // SAFETY: `$ty` and `Stride` are both repr(transparent)
                unsafe { &*(data as *const [T] as *const Self) }
            }

            pub(crate) fn new_mut(data: &mut [T]) -> &mut Self {
                // SAFETY: `$ty` and `Stride` are both repr(transparent)
                unsafe { &mut *(data as *mut [T] as *mut Self) }
            }
        }
    };
}

impl_view! { Row }
impl_view! { Column }

impl<T, const M: usize, const N: usize> Row<T, M, N> {
    /// Returns the dot product between a row and column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vectrix::{vector, row_vector};
    /// #
    /// let row_vector = row_vector![1, 2, 3];
    /// let row = row_vector.row(0);
    ///
    /// let column_vector = vector![4, 5, 6];
    /// let column = column_vector.column(0);
    ///
    /// assert_eq!(row.dot(column), 32);
    /// ```
    #[inline]
    pub fn dot<const P: usize>(&self, other: &Column<T, N, P>) -> T
    where
        T: Copy + Mul<Output = T> + Sum,
    {
        (0..N).map(|i| self[i] * other[i]).sum()
    }
}
