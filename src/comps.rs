//! Component access for matrices and constructors from components.

use crate::prelude::*;

////////////////////////////////////////////////////////////////////////////////
/// Accessors
////////////////////////////////////////////////////////////////////////////////

macro_rules! struct_coord {
    ($Coord:ident: $($comp:ident),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(C)]
        pub struct $Coord<T> {
            $(pub $comp: T),*
        }
    };
}

macro_rules! impl_deref {
    (($M:literal, $N:literal) -> $Target:ident) => {
        impl<T> Deref for Matrix<T, $M, $N> {
            type Target = $Target<T>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                let ptr = self.data.as_ptr() as *const $Target<T>;
                unsafe { &*ptr }
            }
        }

        impl<T> DerefMut for Matrix<T, $M, $N> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                let ptr = self.data.as_mut_ptr() as *mut $Target<T>;
                unsafe { &mut *ptr }
            }
        }
    };
}

struct_coord! { X: x }
struct_coord! { XY: x, y }
struct_coord! { XYZ: x, y, z }
struct_coord! { XYZW: x, y, z, w }
struct_coord! { XYZWA: x, y, z, w, a }
struct_coord! { XYZWAB: x, y, z, w, a, b }

// Safety: given ($M, $N) -> $Target
//         - $Target should be marked #[repr(C)].
//         - $Target<T> should be the same size as [T; $N].
// row vectors
impl_deref! { (1, 1) -> X }
impl_deref! { (1, 2) -> XY }
impl_deref! { (1, 3) -> XYZ }
impl_deref! { (1, 4) -> XYZW }
impl_deref! { (1, 5) -> XYZWA }
impl_deref! { (1, 6) -> XYZWAB }
// column vectors
impl_deref! { (2, 1) -> XY }
impl_deref! { (3, 1) -> XYZ }
impl_deref! { (4, 1) -> XYZW }
impl_deref! { (5, 1) -> XYZWA }
impl_deref! { (6, 1) -> XYZWAB }
