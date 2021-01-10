use core::ops::*;

use crate::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.array[i]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.array[i]
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector + T => Vector
////////////////////////////////////////////////////////////////////////////////

macro_rules! _impl_op_scalar {
    ($meth:ident, impl $trt:ident<$rhs:ty> for $lhs:ty where T: $($bound:tt)+) => {
        impl<'a, T: Base, const N: usize> $trt<$rhs> for $lhs
        where
            T: $($bound)+
        {
            type Output = Vector<T, N>;

            fn $meth(self, other: $rhs) -> Self::Output {
                self.map(|n| n.$meth(other))
            }
        }
    };
}

macro_rules! impl_op_scalar {
    ($trt:ident, $meth:ident) => {
        _impl_op_scalar! { $meth, impl $trt<    T> for  Vector<T, N> where T: $trt<Output = T>        } //  Vector +  T
        _impl_op_scalar! { $meth, impl $trt<&'a T> for  Vector<T, N> where T: $trt<&'a T, Output = T> } //  Vector + &T
        _impl_op_scalar! { $meth, impl $trt<    T> for &Vector<T, N> where T: $trt<Output = T>        } // &Vector +  T
        _impl_op_scalar! { $meth, impl $trt<&'a T> for &Vector<T, N> where T: $trt<&'a T, Output = T> } // &Vector + &T
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
// Vector += T
////////////////////////////////////////////////////////////////////////////////

macro_rules! _impl_op_assign_scalar {
    ($meth:ident, impl $trt:ident<$rhs:ty> where T: $($bound:tt)+) => {
        impl<'a, T: Base, const N: usize> $trt<$rhs> for Vector<T, N>
        where
            T: $($bound)+
        {
            fn $meth(&mut self, other: $rhs) {
                for i in 0..N {
                    self[i].$meth(other);
                }
            }
        }
    };
}

macro_rules! impl_op_assign_scalar {
    ($trt:ident, $meth:ident) => {
        _impl_op_assign_scalar! { $meth, impl $trt<    T> where T: $trt        } // Vector +=  T
        _impl_op_assign_scalar! { $meth, impl $trt<&'a T> where T: $trt<&'a T> } // Vector += &T
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
// Vector + Vector => Vector
////////////////////////////////////////////////////////////////////////////////

macro_rules! _impl_op {
    ($meth:ident, impl $trt:ident<$rhs:ty> for $lhs:ty, $($deref:tt)?) => {
        impl<T: Base, const N: usize> $trt<$rhs> for $lhs
        where
            T: $trt<Output = T>
        {
            type Output = Vector<T, N>;

            fn $meth(self, other: $rhs) -> Self::Output {
                let mut vector = $($deref)? self;
                for i in 0..N {
                    vector[i] = self[i].$meth(other[i]);
                }
                vector
            }
        }
    };
}

macro_rules! impl_op {
    ($trt:ident, $meth:ident) => {
        _impl_op! { $meth, impl $trt< Vector<T, N>> for  Vector<T, N>,   } //  Vector +  Vector
        _impl_op! { $meth, impl $trt< Vector<T, N>> for &Vector<T, N>, * } //  Vector + &Vector
        _impl_op! { $meth, impl $trt<&Vector<T, N>> for  Vector<T, N>,   } // &Vector +  Vector
        _impl_op! { $meth, impl $trt<&Vector<T, N>> for &Vector<T, N>, * } // &Vector + &Vector
    };
}

impl_op! { Add, add }
impl_op! { Sub, sub }

////////////////////////////////////////////////////////////////////////////////
// Vector += Vector
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_op_assign {
    (impl $trt:ident<$rhs:ty>, $meth:ident) => {
        impl<T: Base, const N: usize> $trt<$rhs> for Vector<T, N>
        where
            T: $trt,
        {
            fn $meth(&mut self, other: $rhs) {
                for i in 0..N {
                    self[i].$meth(other[i]);
                }
            }
        }
    };
}

impl_op_assign! { impl AddAssign< Vector<T, N>>, add_assign }
impl_op_assign! { impl AddAssign<&Vector<T, N>>, add_assign }
impl_op_assign! { impl SubAssign< Vector<T, N>>, sub_assign }
impl_op_assign! { impl SubAssign<&Vector<T, N>>, sub_assign }

////////////////////////////////////////////////////////////////////////////////
// -Vector
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_op_unary {
    (impl $trt:ident, $meth:ident for $lhs:ty) => {
        impl<T: Base, const N: usize> $trt for $lhs
        where
            T: $trt<Output = T>,
        {
            type Output = Vector<T, N>;

            fn $meth(self) -> Self::Output {
                self.map($trt::$meth)
            }
        }
    };
}

impl_op_unary! { impl Neg, neg for  Vector<T, N> }
impl_op_unary! { impl Neg, neg for &Vector<T, N> }
impl_op_unary! { impl Not, not for  Vector<T, N> }
impl_op_unary! { impl Not, not for &Vector<T, N> }
