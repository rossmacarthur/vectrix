//! Component access for vectors and constructors from components.

use core::ops::{Deref, DerefMut};

use crate::{Matrix, RowVector, Vector};

////////////////////////////////////////////////////////////////////////////////
// Accessors
////////////////////////////////////////////////////////////////////////////////

macro_rules! struct_coord {
    ($Coord:ident: $($comp:ident),*) => {
        #[allow(clippy::upper_case_acronyms)]
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
                let ptr = self.as_ptr() as *const $Target<T>;
                unsafe { &*ptr }
            }
        }

        impl<T> DerefMut for Matrix<T, $M, $N> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                let ptr = self.as_mut_ptr() as *mut $Target<T>;
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

// SAFETY: given ($M, $N) -> $Target
//         - $Target should be marked #[repr(C)].
//         - $Target<T> should be the same size as [T; $N].
impl_deref! { (1, 1) -> X }
// row vectors
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
// Macros
////////////////////////////////////////////////////////////////////////////////

/// A macro for composing row vectors.
#[macro_export]
macro_rules! row_vector {
    ($repeat:expr; $n:expr) => {
        $crate::RowVector::from_column_major_order([[$repeat]; $n])
    };
    ($($value:expr),* $(,)?) => {
        $crate::RowVector::from_column_major_order([$([$value]),*])
    };
}

/// A macro for composing vectors.
#[macro_export]
macro_rules! vector {
    ($repeat:expr; $n:expr) => {
        $crate::Vector::from_column_major_order([[$repeat; $n]])
    };
    ($($value:expr),* $(,)?) => {
        $crate::Vector::from_column_major_order([[$($value),*]])
    };
}

////////////////////////////////////////////////////////////////////////////////
// Component constructors
////////////////////////////////////////////////////////////////////////////////

impl<T> RowVector<T, 1> {
    /// Creates a new vector from the given components.
    pub const fn new(x: T) -> Self {
        Self { data: [[x]] }
    }
}

impl<T> RowVector<T, 2> {
    /// Creates a new vector from the given components.
    pub const fn new(x: T, y: T) -> Self {
        Self { data: [[x], [y]] }
    }
}

impl<T> RowVector<T, 3> {
    /// Creates a new vector from the given components.
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self {
            data: [[x], [y], [z]],
        }
    }
}

impl<T> RowVector<T, 4> {
    /// Creates a new vector from the given components.
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {
            data: [[x], [y], [z], [w]],
        }
    }
}

impl<T> RowVector<T, 5> {
    /// Creates a new vector from the given components.
    pub const fn new(x: T, y: T, z: T, w: T, a: T) -> Self {
        Self {
            data: [[x], [y], [z], [w], [a]],
        }
    }
}

impl<T> RowVector<T, 6> {
    /// Creates a new vector from the given components.
    pub const fn new(x: T, y: T, z: T, w: T, a: T, b: T) -> Self {
        Self {
            data: [[x], [y], [z], [w], [a], [b]],
        }
    }
}

impl<T> Vector<T, 2> {
    /// Creates a new vector from the given components.
    pub const fn new(x: T, y: T) -> Self {
        Self { data: [[x, y]] }
    }
}

impl<T> Vector<T, 3> {
    /// Creates a new vector from the given components.
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { data: [[x, y, z]] }
    }
}

impl<T> Vector<T, 4> {
    /// Creates a new vector from the given components.
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {
            data: [[x, y, z, w]],
        }
    }
}

impl<T> Vector<T, 5> {
    /// Creates a new vector from the given components.
    pub const fn new(x: T, y: T, z: T, w: T, a: T) -> Self {
        Self {
            data: [[x, y, z, w, a]],
        }
    }
}
impl<T> Vector<T, 6> {
    /// Creates a new vector from the given components.
    pub const fn new(x: T, y: T, z: T, w: T, a: T, b: T) -> Self {
        Self {
            data: [[x, y, z, w, a, b]],
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// From array
////////////////////////////////////////////////////////////////////////////////

impl<T> From<[T; 1]> for Matrix<T, 1, 1> {
    fn from(arr: [T; 1]) -> Self {
        Self { data: [arr] }
    }
}

impl<T> From<[T; 2]> for RowVector<T, 2> {
    fn from([x, y]: [T; 2]) -> Self {
        Self { data: [[x], [y]] }
    }
}

impl<T> From<[T; 3]> for RowVector<T, 3> {
    fn from([x, y, z]: [T; 3]) -> Self {
        Self {
            data: [[x], [y], [z]],
        }
    }
}

impl<T> From<[T; 4]> for RowVector<T, 4> {
    fn from([x, y, z, w]: [T; 4]) -> Self {
        Self {
            data: [[x], [y], [z], [w]],
        }
    }
}

impl<T> From<[T; 5]> for RowVector<T, 5> {
    fn from([x, y, z, w, a]: [T; 5]) -> Self {
        Self {
            data: [[x], [y], [z], [w], [a]],
        }
    }
}

impl<T> From<[T; 6]> for RowVector<T, 6> {
    fn from([x, y, z, w, a, b]: [T; 6]) -> Self {
        Self {
            data: [[x], [y], [z], [w], [a], [b]],
        }
    }
}

impl<T> From<[T; 2]> for Vector<T, 2> {
    fn from(arr: [T; 2]) -> Self {
        Self { data: [arr] }
    }
}

impl<T> From<[T; 3]> for Vector<T, 3> {
    fn from(arr: [T; 3]) -> Self {
        Self { data: [arr] }
    }
}

impl<T> From<[T; 4]> for Vector<T, 4> {
    fn from(arr: [T; 4]) -> Self {
        Self { data: [arr] }
    }
}

impl<T> From<[T; 5]> for Vector<T, 5> {
    fn from(arr: [T; 5]) -> Self {
        Self { data: [arr] }
    }
}

impl<T> From<[T; 6]> for Vector<T, 6> {
    fn from(arr: [T; 6]) -> Self {
        Self { data: [arr] }
    }
}

////////////////////////////////////////////////////////////////////////////////
// From tuple
////////////////////////////////////////////////////////////////////////////////

impl<T> From<(T,)> for Matrix<T, 1, 1> {
    fn from((x,): (T,)) -> Self {
        Self { data: [[x]] }
    }
}

impl<T> From<(T, T)> for RowVector<T, 2> {
    fn from((x, y): (T, T)) -> Self {
        Self { data: [[x], [y]] }
    }
}

impl<T> From<(T, T, T)> for RowVector<T, 3> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self {
            data: [[x], [y], [z]],
        }
    }
}

impl<T> From<(T, T, T, T)> for RowVector<T, 4> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        Self {
            data: [[x], [y], [z], [w]],
        }
    }
}

impl<T> From<(T, T, T, T, T)> for RowVector<T, 5> {
    fn from((x, y, z, w, a): (T, T, T, T, T)) -> Self {
        Self {
            data: [[x], [y], [z], [w], [a]],
        }
    }
}

impl<T> From<(T, T, T, T, T, T)> for RowVector<T, 6> {
    fn from((x, y, z, w, a, b): (T, T, T, T, T, T)) -> Self {
        Self {
            data: [[x], [y], [z], [w], [a], [b]],
        }
    }
}

impl<T> From<(T, T)> for Vector<T, 2> {
    fn from((x, y): (T, T)) -> Self {
        Self { data: [[x, y]] }
    }
}

impl<T> From<(T, T, T)> for Vector<T, 3> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self { data: [[x, y, z]] }
    }
}

impl<T> From<(T, T, T, T)> for Vector<T, 4> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        Self {
            data: [[x, y, z, w]],
        }
    }
}

impl<T> From<(T, T, T, T, T)> for Vector<T, 5> {
    fn from((x, y, z, w, a): (T, T, T, T, T)) -> Self {
        Self {
            data: [[x, y, z, w, a]],
        }
    }
}

impl<T> From<(T, T, T, T, T, T)> for Vector<T, 6> {
    fn from((x, y, z, w, a, b): (T, T, T, T, T, T)) -> Self {
        Self {
            data: [[x, y, z, w, a, b]],
        }
    }
}
