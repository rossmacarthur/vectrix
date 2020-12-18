pub trait Abs {
    fn abs(self) -> Self;
}

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ($ty:ty, $zero:literal) => {
        /// Returns the additive identity element of this type.
        impl Zero for $ty {
            #[inline]
            fn zero() -> $ty {
                $zero
            }
        }
    };
}

macro_rules! impl_abs {
    ($ty:ty) => {
        /// Returns the absolute value of `self`.
        impl Abs for $ty {
            #[inline]
            fn abs(self) -> $ty {
                self.abs()
            }
        }
    };
}

macro_rules! impl_abs_self {
    ($ty:ty) => {
        /// Returns the absolute value of `self`.
        impl Abs for $ty {
            #[inline]
            fn abs(self) -> $ty {
                self
            }
        }
    };
}

impl_abs_self!(u8);
impl_abs_self!(u16);
impl_abs_self!(u32);
impl_abs_self!(u64);
impl_abs_self!(u128);
impl_abs_self!(usize);

impl_abs!(i8);
impl_abs!(i16);
impl_abs!(i32);
impl_abs!(i64);
impl_abs!(i128);
impl_abs!(isize);

impl_abs!(f32);
impl_abs!(f64);

impl_zero!(u8, 0);
impl_zero!(u16, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);
impl_zero!(u128, 0);
impl_zero!(usize, 0);

impl_zero!(i8, 0);
impl_zero!(i16, 0);
impl_zero!(i32, 0);
impl_zero!(i64, 0);
impl_zero!(i128, 0);
impl_zero!(isize, 0);

impl_zero!(f32, 0.0);
impl_zero!(f64, 0.0);
