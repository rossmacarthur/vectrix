//! This crate provides a slice-like [`Stride<T, S>`] type where elements are
//! spaced a constant `S` elements in memory.
//!
//! For example, given an underlying slice `&[1, 2, 3, 4, 5, 6]`, the elements
//! `&[1, 3, 5]` are a strided slice with a stride of 2. This crate makes use of
//! const generics to provide the stride value `S` at compile time so that there
//! is no runtime memory overhead to strided slices; `Stride` takes up the same
//! amount of space as a slice.
//!
//! Many slice-like operations are implemented for `Stride` including iteration
//! and indexing. Method names are similar to those of the slice type.
//!
//! Where you want a strided slice use:
//! - [`::new()`][`Stride::new`] to construct a [`&Stride<T, S>`][`Stride`] that
//!   wraps a [`&[T]`][`slice`].
//! - [`::new_mut()`][`Stride::new_mut`] to construct a
//!   [`&mut Stride<T, S>`][`Stride`] that wraps a [`&mut [T]`][`slice`].
//!
//! ```rust
//! use stride::Stride;
//!
//! // The underlying data.
//! let data = &mut [1, 2, 7, 4, 5, 6];
//!
//! // Create a strided slice with a stride of `2` referring to
//! // elements `1`, `7`, and `5`.
//! let stride = Stride::<_, 2>::new_mut(data);
//!
//! assert_eq!(stride.len(), 3);
//!
//! // We can use indexing to view values ..
//! assert_eq!(stride[0], 1);
//! assert_eq!(stride[1..3], &[7, 5]);
//!
//! // .. or modify them.
//! stride[1] = 3;
//! assert_eq!(stride, &[1, 3, 5]);
//! assert_eq!(data, &[1, 2, 3, 4, 5, 6]);
//! ```

#![no_std]
#![allow(unused_unsafe)]

mod index;
mod iter;
mod ops;

use core::fmt;

pub use crate::index::StrideIndex;
pub use crate::iter::{Iter, IterMut};

/// A constant strided slice.
#[repr(transparent)]
pub struct Stride<T, const S: usize> {
    data: [T],
}

impl<T, const S: usize> fmt::Debug for Stride<T, S>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T, const S: usize> Default for &Stride<T, S> {
    fn default() -> Self {
        Stride::new(&[])
    }
}

impl<T, const S: usize> Default for &mut Stride<T, S> {
    fn default() -> Self {
        Stride::new_mut(&mut [])
    }
}

impl<T, const S: usize> Stride<T, S> {
    /// Constructs a new strided slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stride::Stride;
    /// #
    /// let data = &[1, 2, 3, 4, 5, 6];
    /// let stride = Stride::<_, 3>::new(data);
    /// ```
    pub fn new(data: &[T]) -> &Self {
        unsafe { &*(data as *const [T] as *const Self) }
    }

    /// Constructs a new mutable strided slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stride::Stride;
    /// #
    /// let data = &mut [1, 2, 3, 4, 5, 6];
    /// let stride = Stride::<_, 3>::new_mut(data);
    /// ```
    pub fn new_mut(data: &mut [T]) -> &mut Self {
        unsafe { &mut *(data as *mut [T] as *mut Self) }
    }

    /// Returns the number of elements in the strided slice.
    ///
    /// This is equivalent to the ceiling division of the underlying slice
    /// length by `S`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stride::Stride;
    /// #
    /// let data = &[1, 2, 3, 4, 5, 6];
    /// assert_eq!(Stride::<_, 1>::new(data).len(), 6);
    /// assert_eq!(Stride::<_, 2>::new(data).len(), 3);
    /// assert_eq!(Stride::<_, 3>::new(data).len(), 2);
    /// ```
    pub const fn len(&self) -> usize {
        (self.data.len() + S - 1) / S
    }

    /// Returns `true` if the strided slice has a length of 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stride::Stride;
    /// #
    /// let stride = Stride::<_, 3>::new(&[1, 2, 3, 4, 5, 6]);
    /// assert!(!stride.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a raw pointer to the underlying slice's buffer.
    ///
    /// *See [`slice::as_ptr()`].*
    pub const fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the underlying slice's buffer.
    ///
    /// *See [`slice::as_mut_ptr()`].*
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    /// Returns a reference to an element or substride depending on the type of
    /// index.
    ///
    /// - If given a position, returns a reference to the element at that
    ///   position or `None` if out of bounds.
    /// - If given a range, returns the substride corresponding to that range,
    ///   or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stride::Stride;
    /// #
    /// let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    /// assert_eq!(stride.get(1), Some(&3));
    /// assert_eq!(stride.get(0..2), Some(Stride::<_, 2>::new(&[1, 2, 3, 4])));
    /// assert_eq!(stride.get(3), None);
    /// assert_eq!(stride.get(0..4), None);
    /// ```
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: StrideIndex<Stride<T, S>>,
    {
        index.get(self)
    }

    /// Returns a mutable reference to an element or substride depending on the
    /// type of index (see [`get`]) or `None` if the index is out of bounds.
    ///
    /// [`get`]: #method.get
    ///
    /// # Examples
    ///
    /// ```
    /// # use stride::Stride;
    /// #
    /// let data = &mut [0, 1, 2, 3];
    /// let stride = Stride::<_, 2>::new_mut(data);
    ///
    /// if let Some(elem) = stride.get_mut(1) {
    ///     *elem = 42;
    /// }
    /// assert_eq!(stride, Stride::<_, 2>::new(&[0, 1, 42, 3]));
    /// ```
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: StrideIndex<Stride<T, S>>,
    {
        index.get_mut(self)
    }

    /// Returns a reference to an element or substride, without doing bounds
    /// checking.
    ///
    /// For a safe alternative see [`get`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [`get`]: #method.get
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: StrideIndex<Self>,
    {
        unsafe { &*index.get_unchecked(self) }
    }

    /// Returns a mutable reference to an element or substride, without doing
    /// bounds checking.
    ///
    /// For a safe alternative see [`get_mut`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [`get_mut`]: #method.get_mut
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
    where
        I: StrideIndex<Self>,
    {
        unsafe { &mut *index.get_unchecked_mut(self) }
    }

    /// Returns a reference to the first element of the strided slice, or `None`
    /// if it is empty.
    pub fn first(&self) -> Option<&T> {
        self.get(0)
    }

    /// Returns a mutable reference to the first element of the strided slice,
    /// or `None` if it is empty.
    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    /// Returns a reference to the last element of the strided slice, or `None`
    /// if it is empty.
    pub fn last(&self) -> Option<&T> {
        self.get(self.len().saturating_sub(1))
    }

    /// Returns a mutable reference to the last element of the strided slice, or
    /// `None` if it is empty.
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.get_mut(self.len().saturating_sub(1))
    }

    /// Swaps two elements in the strided slice.
    ///
    /// # Arguments
    ///
    /// - `a` - The index of the first element
    /// - `b` - The index of the second element
    ///
    /// # Panics
    ///
    /// If `a` or `b` are out of bounds.
    pub fn swap(&mut self, a: usize, b: usize) {
        self.data.swap(a * S, b * S)
    }

    /// Returns an iterator over the stride.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stride::Stride;
    /// #
    /// let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    /// let mut iterator = stride.iter();
    /// assert_eq!(iterator.next(), Some(&1));
    /// assert_eq!(iterator.next(), Some(&3));
    /// assert_eq!(iterator.next(), Some(&5));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<T, S> {
        Iter::new(self)
    }

    /// Returns an iterator over the stride that allows modifying each value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stride::Stride;
    /// #
    /// let slice = &mut [1, 1, 2, 2, 3, 3];
    /// let stride = Stride::<_, 2>::new_mut(slice);
    /// for elem in stride.iter_mut() {
    ///     *elem *= 2;
    /// }
    /// assert_eq!(slice, &[2, 1, 4, 2, 6, 3]);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T, S> {
        IterMut::new(self)
    }
}

impl<T> Stride<T, 1> {
    /// Returns a slice containing the entire strided slice.
    ///
    /// Only available on strided slices with a stride of `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stride::Stride;
    /// #
    /// let slice = &[1, 2, 3];
    /// let stride = Stride::<_, 1>::new(slice);
    /// assert_eq!(stride.as_slice(), slice);
    /// ```
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Returns a mutable slice containing the entire strided slice.
    ///
    /// Only available on strided slices with a stride of `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stride::Stride;
    /// #
    /// let slice = &mut [1, 2, 7];
    /// let stride = Stride::<_, 1>::new_mut(slice);
    /// stride.as_mut_slice()[2] = 3;
    /// assert_eq!(slice, &[1, 2, 3])
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }
}
