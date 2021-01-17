#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]

use core::ops::*;

use crate::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

impl<T, const M: usize, const N: usize> Index<(usize, usize)> for Matrix<T, M, N> {
    type Output = T;

    #[inline]
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[j][i]
    }
}

impl<T, const M: usize, const N: usize> IndexMut<(usize, usize)> for Matrix<T, M, N> {
    #[inline]
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.data[j][i]
    }
}

impl<T, const M: usize, const N: usize> Index<usize> for Matrix<T, M, N> {
    type Output = T;

    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        &self.as_slice()[idx]
    }
}

impl<T, const M: usize, const N: usize> IndexMut<usize> for Matrix<T, M, N> {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[idx]
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
                for idx in 0..(M * N) {
                    self[idx].$meth(other);
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
                for idx in 0..(M * N) {
                    matrix[idx] = self[idx].$meth(other[idx]);
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
// Matrix += Matrix
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_op_assign {
    (impl $trt:ident<$rhs:ty>, $meth:ident) => {
        impl<T, const M: usize, const N: usize> $trt<$rhs> for Matrix<T, M, N>
        where
            T: Copy + $trt,
        {
            fn $meth(&mut self, other: $rhs) {
                for idx in 0..(M * N) {
                    self[idx].$meth(other[idx]);
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
