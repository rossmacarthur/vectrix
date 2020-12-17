//! Traits to allow component access for vectors.

use crate::Vector;

pub trait X {}
pub trait Y {}
pub trait Z {}
pub trait W {}

macro_rules! impl_trait {
    ($trait:ty, $($N:literal),+) => {
        $(
            impl<T> $trait for Vector<T, $N> {}
        )+
    }
}

macro_rules! impl_accessors {
    ($trait:ident, $get_n:ident, $get_n_ref:ident, $get_n_mut:ident, $n:literal) => {
        impl<T: Copy, const N: usize> Vector<T, N>
        where
            Self: $trait,
        {
            /// Returns this component of the vector.
            pub fn $get_n(&self) -> T {
                self.inner[$n]
            }

            /// Returns a reference to this component of the vector.
            pub fn $get_n_ref(&self) -> &T {
                &self.inner[$n]
            }

            /// Returns a mutable reference to this component of the vector.
            pub fn $get_n_mut(&mut self) -> &mut T {
                &mut self.inner[$n]
            }
        }
    };
}

impl_trait!(X, 1, 2, 3, 4);
impl_trait!(Y, 2, 3, 4);
impl_trait!(Z, 3, 4);
impl_trait!(W, 4);

impl_accessors!(X, x, x_ref, x_mut, 0);
impl_accessors!(Y, y, y_ref, y_mut, 1);
impl_accessors!(Z, z, z_ref, z_mut, 2);
impl_accessors!(W, w, w_ref, w_mut, 3);
