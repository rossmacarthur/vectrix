#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]

use core::iter::Sum;
use core::ops::*;

use crate::prelude::*;

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

macro_rules! _impl_op_scalar {
    ($meth:ident, impl $trt:ident<$rhs:ty> for $lhs:ty where T: $($bound:tt)+) => {
        impl<'a, T, const M: usize, const N: usize> $trt<$rhs> for $lhs
        where
            T: Copy + Default + $($bound)+
        {
            type Output = Matrix<T, M, N>;

            fn $meth(self, other: $rhs) -> Self::Output {
                self.map(|n: T| n.$meth(other))
            }
        }
    };
}

macro_rules! impl_op_scalar {
    ($trt:ident, $meth:ident) => {
        _impl_op_scalar! { $meth, impl $trt<    T> for  Matrix<T, M, N> where T: $trt<Output = T>        } //  Matrix +  T
        _impl_op_scalar! { $meth, impl $trt<&'a T> for  Matrix<T, M, N> where T: $trt<&'a T, Output = T> } //  Matrix + &T
        _impl_op_scalar! { $meth, impl $trt<    T> for &Matrix<T, M, N> where T: $trt<Output = T>        } // &Matrix +  T
        _impl_op_scalar! { $meth, impl $trt<&'a T> for &Matrix<T, M, N> where T: $trt<&'a T, Output = T> } // &Matrix + &T
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

macro_rules! _impl_op_assign_scalar {
    ($meth:ident, impl $trt:ident<$rhs:ty> where T: $($bound:tt)+) => {
        impl<'a, T, const M: usize, const N: usize> $trt<$rhs> for Matrix<T, M, N>
        where
            T: Copy + $($bound)+
        {
            fn $meth(&mut self, other: $rhs) {
                for i in 0..(M * N) {
                    self[i].$meth(other);
                }
            }
        }
    };
}

macro_rules! impl_op_assign_scalar {
    ($trt:ident, $meth:ident) => {
        _impl_op_assign_scalar! { $meth, impl $trt<    T> where T: $trt        } // Matrix +=  T
        _impl_op_assign_scalar! { $meth, impl $trt<&'a T> where T: $trt<&'a T> } // Matrix += &T
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

macro_rules! _impl_op {
    ($meth:ident, impl $trt:ident<$rhs:ty> for $lhs:ty, $($deref:tt)?) => {
        impl<T, const M: usize, const N: usize> $trt<$rhs> for $lhs
        where
            T: Copy + $trt<Output = T>
        {
            type Output = Matrix<T, M, N>;

            fn $meth(self, other: $rhs) -> Self::Output {
                let mut matrix = $($deref)? self;
                for i in 0..(M * N) {
                    matrix[i] = self[i].$meth(other[i]);
                }
                matrix
            }
        }
    };
}

macro_rules! impl_op {
    ($trt:ident, $meth:ident) => {
        _impl_op! { $meth, impl $trt< Matrix<T, M, N>> for  Matrix<T, M, N>,   } //  Matrix +  Matrix
        _impl_op! { $meth, impl $trt< Matrix<T, M, N>> for &Matrix<T, M, N>, * } //  Matrix + &Matrix
        _impl_op! { $meth, impl $trt<&Matrix<T, M, N>> for  Matrix<T, M, N>,   } // &Matrix +  Matrix
        _impl_op! { $meth, impl $trt<&Matrix<T, M, N>> for &Matrix<T, M, N>, * } // &Matrix + &Matrix
    };
}

impl_op! { Add, add }
impl_op! { Sub, sub }

////////////////////////////////////////////////////////////////////////////////
// Matrix * Matrix
////////////////////////////////////////////////////////////////////////////////

fn matrix_mul<T, const M: usize, const N: usize, const P: usize>(
    lhs: &Matrix<T, M, N>,
    rhs: &Matrix<T, N, P>,
) -> Matrix<T, M, P>
where
    T: Copy + Default + Mul<Output = T> + Sum,
{
    let mut matrix = Matrix::default();
    for i in 0..M {
        for j in 0..P {
            matrix[(i, j)] = lhs.row(i).dot(rhs.column(j));
        }
    }
    matrix
}

macro_rules! impl_op_mul_mul {
    ($lhs:ty, $rhs:ty) => {
        impl<T, const N: usize, const M: usize, const P: usize> Mul<$rhs> for $lhs
        where
            T: Copy + Default + Mul<Output = T> + Sum,
        {
            type Output = Matrix<T, M, P>;

            fn mul(self, rhs: $rhs) -> Self::Output {
                matrix_mul(&self, &rhs)
            }
        }
    };
}

impl_op_mul_mul! {  Matrix<T, M, N>,  Matrix<T, N, P> }
impl_op_mul_mul! {  Matrix<T, M, N>, &Matrix<T, N, P> }
impl_op_mul_mul! { &Matrix<T, M, N>,  Matrix<T, N, P> }
impl_op_mul_mul! { &Matrix<T, M, N>, &Matrix<T, N, P> }

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
    (impl $trt:ident, $meth:ident for $lhs:ty) => {
        impl<T, const M: usize, const N: usize> $trt for $lhs
        where
            T: Copy + Default + $trt<Output = T>,
        {
            type Output = Matrix<T, M, N>;

            fn $meth(self) -> Self::Output {
                self.map($trt::$meth)
            }
        }
    };
}

impl_op_unary! { impl Neg, neg for  Matrix<T, M, N> }
impl_op_unary! { impl Neg, neg for &Matrix<T, M, N> }
impl_op_unary! { impl Not, not for  Matrix<T, M, N> }
impl_op_unary! { impl Not, not for &Matrix<T, M, N> }
