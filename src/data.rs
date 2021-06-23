use stride::Stride;

mod private {
    use super::*;

    pub trait Sealed {}
    impl<T, const M: usize, const N: usize> Sealed for [[T; M]; N] {}
    impl<T, const S: usize> Sealed for Stride<T, S> {}
}

/// A type representing the underlying memory for a matrix.
pub unsafe trait Data<T, const M: usize, const N: usize>: private::Sealed + Sized {}

unsafe impl<T, const M: usize, const N: usize> Data<T, M, N> for [[T; M]; N] {}
