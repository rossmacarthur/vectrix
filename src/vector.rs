//! Component access for vectors and constructors from components.

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

////////////////////////////////////////////////////////////////////////////////
/// Constructors
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_row_vector {
    ($N:literal: $($comp:ident),+) => {
        impl<T> RowVector<T, $N> {
            /// Creates a new vector from the given components.
            pub const fn new($($comp: T),+) -> Self {
                Self { data: [$([$comp]),+]}
            }
        }

        impl<T> From<[T; $N]> for RowVector<T, $N>  {
            fn from([$($comp),*]: [T; $N]) -> Self {
                Self { data: [$([$comp]),+] }
            }
        }
    }
}

macro_rules! impl_column_vector {
    ($M:literal: $($comp:ident),+) => {
        impl<T> ColumnVector<T, $M> {
            /// Creates a new vector from the given components.
            pub const fn new($($comp: T),+) -> Self {
                Self { data: [[$($comp),+]]}
            }
        }

        impl<T> From<[T; $M]> for ColumnVector<T, $M> {
            fn from(data: [T; $M]) -> Self {
                Self { data: [data] }
            }
        }
    }
}

impl_row_vector! { 1: x }
impl_row_vector! { 2: x, y }
impl_row_vector! { 3: x, y, z }
impl_row_vector! { 4: x, y, z, w }
impl_row_vector! { 5: x, y, z, w, a }
impl_row_vector! { 6: x, y, z, w, a, b }

impl_column_vector! { 2: x, y }
impl_column_vector! { 3: x, y, z }
impl_column_vector! { 4: x, y, z, w }
impl_column_vector! { 5: x, y, z, w, a }
impl_column_vector! { 6: x, y, z, w, a, b }
