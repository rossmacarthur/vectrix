use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::Stride;

mod private {
    use super::*;
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for Range<usize> {}
    impl Sealed for RangeFrom<usize> {}
    impl Sealed for RangeFull {}
    impl Sealed for RangeInclusive<usize> {}
    impl Sealed for RangeTo<usize> {}
    impl Sealed for RangeToInclusive<usize> {}
}

/// A simple trait to map stride indexes to slice indexes.
trait Unstride: private::Sealed {
    fn unstride<const S: usize>(self) -> Self;
}

/// A helper trait used for indexing operations.
///
/// This is the [`Stride`] version of [`SliceIndex`][`core::slice::SliceIndex`].
/// You should not use or implement this trait directly but instead use the
/// corresponding methods on [`Stride`].
///
/// # Safety
///
/// Implementations of this trait have to promise that if the argument
/// to `get_(mut_)unchecked` is a safe reference, then so is the result.
pub unsafe trait StrideIndex<T: ?Sized>: private::Sealed {
    /// The output type returned by methods.
    type Output: ?Sized;

    /// Returns a shared reference to the output at this location, if in
    /// bounds.
    fn get(self, stride: &T) -> Option<&Self::Output>;

    /// Returns a mutable reference to the output at this location, if in
    /// bounds.
    fn get_mut(self, stride: &mut T) -> Option<&mut Self::Output>;

    /// Returns a shared reference to the output at this location, without
    /// performing any bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index or a dangling `stride`
    /// pointer is *[undefined behavior]* even if the resulting reference is not
    /// used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    unsafe fn get_unchecked(self, stride: *const T) -> *const Self::Output;

    /// Returns a mutable reference to the output at this location, without
    /// performing any bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index or a dangling `stride`
    /// pointer is *[undefined behavior]* even if the resulting reference is not
    /// used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    unsafe fn get_unchecked_mut(self, stride: *mut T) -> *mut Self::Output;

    /// Returns a shared reference to the output at this location, panicking
    /// if out of bounds.
    #[track_caller]
    fn index(self, stride: &T) -> &Self::Output;

    /// Returns a mutable reference to the output at this location, panicking
    /// if out of bounds.
    #[track_caller]
    fn index_mut(self, stride: &mut T) -> &mut Self::Output;
}

impl Unstride for usize {
    fn unstride<const S: usize>(self) -> Self {
        self * S
    }
}

impl Unstride for Range<usize> {
    fn unstride<const S: usize>(self) -> Self {
        Range {
            start: self.start * S,
            end: self.end * S,
        }
    }
}

impl Unstride for RangeFrom<usize> {
    fn unstride<const S: usize>(self) -> Self {
        RangeFrom {
            start: self.start * S,
        }
    }
}

impl Unstride for RangeFull {
    fn unstride<const S: usize>(self) -> Self {
        self
    }
}

impl Unstride for RangeInclusive<usize> {
    fn unstride<const S: usize>(self) -> Self {
        RangeInclusive::new(self.start() * S, self.end() * S)
    }
}

impl Unstride for RangeTo<usize> {
    fn unstride<const S: usize>(self) -> Self {
        RangeTo { end: self.end * S }
    }
}

impl Unstride for RangeToInclusive<usize> {
    fn unstride<const S: usize>(self) -> Self {
        RangeToInclusive { end: self.end * S }
    }
}

unsafe impl<T, const S: usize> StrideIndex<Stride<T, S>> for usize {
    type Output = T;

    fn get(self, stride: &Stride<T, S>) -> Option<&Self::Output> {
        let i = self.unstride::<S>();
        stride.data.get(i)
    }

    fn get_mut(self, stride: &mut Stride<T, S>) -> Option<&mut Self::Output> {
        let i = self.unstride::<S>();
        stride.data.get_mut(i)
    }

    unsafe fn get_unchecked(self, stride: *const Stride<T, S>) -> *const Self::Output {
        let i = self.unstride::<S>();
        unsafe { (*stride).data.get_unchecked(i) }
    }

    unsafe fn get_unchecked_mut(self, stride: *mut Stride<T, S>) -> *mut Self::Output {
        let i = self.unstride::<S>();
        unsafe { (*stride).data.get_unchecked_mut(i) }
    }

    #[track_caller]
    fn index(self, stride: &Stride<T, S>) -> &Self::Output {
        let i = self.unstride::<S>();
        &stride.data[i]
    }

    #[track_caller]
    fn index_mut(self, stride: &mut Stride<T, S>) -> &mut Self::Output {
        let i = self.unstride::<S>();
        &mut stride.data[i]
    }
}

macro_rules! impl_stride_index {
    ($Index:ty) => {
        unsafe impl<T, const S: usize> StrideIndex<Stride<T, S>> for $Index {
            type Output = Stride<T, S>;

            fn get(self, stride: &Stride<T, S>) -> Option<&Self::Output> {
                let i = self.unstride::<S>();
                stride.data.get(i).map(Stride::new)
            }

            fn get_mut(self, stride: &mut Stride<T, S>) -> Option<&mut Self::Output> {
                let i = self.unstride::<S>();
                stride.data.get_mut(i).map(Stride::new_mut)
            }

            unsafe fn get_unchecked(self, stride: *const Stride<T, S>) -> *const Self::Output {
                let i = self.unstride::<S>();
                let slice = unsafe { (*stride).data.get_unchecked(i) };
                Stride::new(slice)
            }

            unsafe fn get_unchecked_mut(self, stride: *mut Stride<T, S>) -> *mut Self::Output {
                let i = self.unstride::<S>();
                let slice = unsafe { (*stride).data.get_unchecked_mut(i) };
                Stride::new_mut(slice)
            }

            #[track_caller]
            fn index(self, stride: &Stride<T, S>) -> &Self::Output {
                let i = self.unstride::<S>();
                Stride::new(&stride.data[i])
            }

            #[track_caller]
            fn index_mut(self, stride: &mut Stride<T, S>) -> &mut Self::Output {
                let i = self.unstride::<S>();
                Stride::new_mut(&mut stride.data[i])
            }
        }
    };
}

impl_stride_index! { Range<usize> }
impl_stride_index! { RangeFrom<usize> }
impl_stride_index! { RangeFull }
impl_stride_index! { RangeInclusive<usize> }
impl_stride_index! { RangeTo<usize> }
impl_stride_index! { RangeToInclusive<usize> }
