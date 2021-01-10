//! Allow component access for vectors.

use crate::prelude::*;

macro_rules! struct_coord {
    ($Coord:ident: $($comps:ident),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(C)]
        pub struct $Coord<T> {
            $(pub $comps: T),*
        }
    };
}

macro_rules! impl_deref {
    ($N:literal -> $Target:ident) => {
        impl<T> Deref for Vector<T, $N> {
            type Target = $Target<T>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                let ptr = self.array.as_ptr() as *const $Target<T>;
                unsafe { &*ptr }
            }
        }

        impl<T> DerefMut for Vector<T, $N> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                let ptr = self.array.as_mut_ptr() as *mut $Target<T>;
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

// Safety: given $N -> $Target
// - $Target should be marked #[repr(C)].
// - $Target<T> should be the same size as [T; $N].
impl_deref! { 1 -> X }
impl_deref! { 2 -> XY }
impl_deref! { 3 -> XYZ }
impl_deref! { 4 -> XYZW }
impl_deref! { 5 -> XYZWA }
impl_deref! { 6 -> XYZWAB }
