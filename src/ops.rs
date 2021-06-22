#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]

use core::iter::Sum;
use core::ops::*;

use crate::{Matrix, MatrixIndex, Zero};

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

impl<T, I, const M: usize, const N: usize> Index<I> for Matrix<T, M, N>
where
    I: MatrixIndex<Self>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &I::Output {
        index.index(self)
    }
}

impl<T, I, const M: usize, const N: usize> IndexMut<I> for Matrix<T, M, N>
where
    I: MatrixIndex<Self>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        index.index_mut(self)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix + T
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_op_scalar {
    ($trt:ident, $meth:ident) => {
        // Matrix + T
        impl<T, const M: usize, const N: usize> $trt<T> for Matrix<T, M, N>
        where
            T: Copy + $trt<Output = T>,
        {
            type Output = Matrix<T, M, N>;

            fn $meth(mut self, other: T) -> Self::Output {
                for i in 0..(M * N) {
                    self[i] = self[i].$meth(other);
                }
                self
            }
        }

        // Matrix + &T
        impl<T, const M: usize, const N: usize> $trt<&T> for Matrix<T, M, N>
        where
            T: Copy + $trt<Output = T>,
        {
            type Output = Matrix<T, M, N>;

            fn $meth(mut self, other: &T) -> Self::Output {
                for i in 0..(M * N) {
                    self[i] = self[i].$meth(*other);
                }
                self
            }
        }

        // &Matrix + T
        impl<T, const M: usize, const N: usize> $trt<T> for &Matrix<T, M, N>
        where
            T: Copy + Zero + $trt<Output = T>,
        {
            type Output = Matrix<T, M, N>;

            fn $meth(self, other: T) -> Self::Output {
                let mut matrix = Self::Output::zero();
                for i in 0..(M * N) {
                    matrix[i] = self[i].$meth(other);
                }
                matrix
            }
        }

        // &Matrix + &T
        impl<T, const M: usize, const N: usize> $trt<&T> for &Matrix<T, M, N>
        where
            T: Copy + Zero + $trt<Output = T>,
        {
            type Output = Matrix<T, M, N>;

            fn $meth(self, other: &T) -> Self::Output {
                let mut matrix = Self::Output::zero();
                for i in 0..(M * N) {
                    matrix[i] = self[i].$meth(*other);
                }
                matrix
            }
        }
    };
}

impl_op_scalar! { Add, add }
impl_op_scalar! { Sub, sub }
impl_op_scalar! { Mul, mul }
impl_op_scalar! { Div, div }
impl_op_scalar! { Rem, rem }

impl_op_scalar! { BitAnd, bitand }
impl_op_scalar! { BitOr, bitor }
impl_op_scalar! { BitXor, bitxor }
impl_op_scalar! { Shl, shl }
impl_op_scalar! { Shr, shr }

////////////////////////////////////////////////////////////////////////////////
// Matrix += T
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_op_assign_scalar {
    ($trt:ident, $meth:ident) => {
        // Matrix += T
        impl<'a, T, const M: usize, const N: usize> $trt<T> for Matrix<T, M, N>
        where
            T: Copy + $trt<T>,
        {
            fn $meth(&mut self, other: T) {
                for i in 0..(M * N) {
                    self[i].$meth(other);
                }
            }
        }

        // Matrix += &T
        impl<T, const M: usize, const N: usize> $trt<&T> for Matrix<T, M, N>
        where
            T: Copy + $trt<T>,
        {
            fn $meth(&mut self, other: &T) {
                for i in 0..(M * N) {
                    self[i].$meth(*other);
                }
            }
        }
    };
}

impl_op_assign_scalar! { AddAssign, add_assign }
impl_op_assign_scalar! { SubAssign, sub_assign }
impl_op_assign_scalar! { MulAssign, mul_assign }
impl_op_assign_scalar! { DivAssign, div_assign }
impl_op_assign_scalar! { RemAssign, rem_assign }

impl_op_assign_scalar! { BitAndAssign, bitand_assign }
impl_op_assign_scalar! { BitOrAssign, bitor_assign }
impl_op_assign_scalar! { BitXorAssign, bitxor_assign }
impl_op_assign_scalar! { ShlAssign, shl_assign }
impl_op_assign_scalar! { ShrAssign, shr_assign }

////////////////////////////////////////////////////////////////////////////////
// Matrix + Matrix
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_op {
    ($trt:ident, $meth:ident) => {
        // Matrix + Matrix
        impl<T, const M: usize, const N: usize> $trt<Matrix<T, M, N>> for Matrix<T, M, N>
        where
            T: Copy + $trt<Output = T>,
        {
            type Output = Matrix<T, M, N>;

            fn $meth(mut self, other: Matrix<T, M, N>) -> Self::Output {
                for i in 0..(M * N) {
                    self[i] = self[i].$meth(other[i]);
                }
                self
            }
        }

        // Matrix + &Matrix
        impl<T, const M: usize, const N: usize> $trt<&Matrix<T, M, N>> for Matrix<T, M, N>
        where
            T: Copy + $trt<Output = T>,
        {
            type Output = Matrix<T, M, N>;

            fn $meth(mut self, other: &Matrix<T, M, N>) -> Self::Output {
                for i in 0..(M * N) {
                    self[i] = self[i].$meth(other[i]);
                }
                self
            }
        }

        // &Matrix + Matrix
        impl<T, const M: usize, const N: usize> $trt<Matrix<T, M, N>> for &Matrix<T, M, N>
        where
            T: Copy + Zero + $trt<Output = T>,
        {
            type Output = Matrix<T, M, N>;

            fn $meth(self, other: Matrix<T, M, N>) -> Self::Output {
                let mut matrix = *self;
                for i in 0..(M * N) {
                    matrix[i] = self[i].$meth(other[i]);
                }
                matrix
            }
        }

        // &Matrix + &Matrix
        impl<T, const M: usize, const N: usize> $trt<&Matrix<T, M, N>> for &Matrix<T, M, N>
        where
            T: Copy + Zero + $trt<Output = T>,
        {
            type Output = Matrix<T, M, N>;

            fn $meth(self, other: &Matrix<T, M, N>) -> Self::Output {
                let mut matrix = *self;
                for i in 0..(M * N) {
                    matrix[i] = self[i].$meth(other[i]);
                }
                matrix
            }
        }
    };
}

impl_op! { Add, add }
impl_op! { Sub, sub }

////////////////////////////////////////////////////////////////////////////////
// Matrix * Matrix
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_op_mul {
    ($lhs:ty, $rhs:ty) => {
        impl<T, const N: usize, const M: usize, const P: usize> Mul<$rhs> for $lhs
        where
            T: Copy + Zero + Mul<Output = T> + Sum,
        {
            type Output = Matrix<T, M, P>;

            fn mul(self, rhs: $rhs) -> Self::Output {
                let mut matrix = Self::Output::zero();
                for i in 0..M {
                    for j in 0..P {
                        matrix[(i, j)] = self.row(i).dot(rhs.column(j));
                    }
                }
                matrix
            }
        }
    };
}

impl_op_mul! {  Matrix<T, M, N>,  Matrix<T, N, P> }
impl_op_mul! {  Matrix<T, M, N>, &Matrix<T, N, P> }
impl_op_mul! { &Matrix<T, M, N>,  Matrix<T, N, P> }
impl_op_mul! { &Matrix<T, M, N>, &Matrix<T, N, P> }

////////////////////////////////////////////////////////////////////////////////
// Matrix += Matrix
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_op_assign {
    (impl $trt:ident<$rhs:ty>, $meth:ident) => {
        impl<T, const M: usize, const N: usize> $trt<$rhs> for Matrix<T, M, N>
        where
            T: Copy + $trt,
        {
            fn $meth(&mut self, other: $rhs) {
                for i in 0..(M * N) {
                    self[i].$meth(other[i]);
                }
            }
        }
    };
}

impl_op_assign! { impl AddAssign< Matrix<T, M, N>>, add_assign }
impl_op_assign! { impl AddAssign<&Matrix<T, M, N>>, add_assign }
impl_op_assign! { impl SubAssign< Matrix<T, M, N>>, sub_assign }
impl_op_assign! { impl SubAssign<&Matrix<T, M, N>>, sub_assign }

////////////////////////////////////////////////////////////////////////////////
// -Matrix
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_op_unary {
    ($trt:ident, $meth:ident) => {
        impl<T, const M: usize, const N: usize> $trt for Matrix<T, M, N>
        where
            T: Copy + Zero + $trt<Output = T>,
        {
            type Output = Matrix<T, M, N>;

            fn $meth(mut self) -> Self::Output {
                for i in 0..(M * N) {
                    self[i] = self[i].$meth();
                }
                self
            }
        }

        impl<T, const M: usize, const N: usize> $trt for &Matrix<T, M, N>
        where
            T: Copy + Zero + $trt<Output = T>,
        {
            type Output = Matrix<T, M, N>;

            fn $meth(self) -> Self::Output {
                let mut matrix = Self::Output::zero();
                for i in 0..(M * N) {
                    matrix[i] = self[i].$meth();
                }
                matrix
            }
        }
    };
}

impl_op_unary! { Neg, neg }
impl_op_unary! { Not, not }
