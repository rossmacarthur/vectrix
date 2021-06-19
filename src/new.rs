//! Generic constructors.

use core::iter::FromIterator;
use core::mem::{self, MaybeUninit};
use core::ptr;

use crate::prelude::*;

/// A macro for composing matrices.
///
/// This macro allows one to write such a matrix in the natural order. For
/// example:
///
/// ```rust
/// # use vectrix::matrix;
/// #
/// let m = matrix![
///     1.0, 4.0;
///     2.0, 5.0;
///     3.0, 6.0;
/// ];
/// ```
///
/// corresponds to the following matrix with three rows and two columns:
///
/// ```text
/// ┌            ┐
/// │  1.0  4.0  │
/// │  2.0  5.0  │
/// │  3.0  6.0  │
/// └            ┘
/// ```
///
/// Which is stored as an array of arrays in column-major order.
///
/// ```text
/// Matrix { data: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]] }
/// ```
#[cfg(feature = "macro")]
#[macro_export]
macro_rules! matrix {
    ($($data:tt)*) => {
        $crate::Matrix::from_column_major_order($crate::proc_macro::matrix!($($data)*))
    };
}

impl<T, const M: usize, const N: usize> Default for Matrix<T, M, N>
where
    T: Copy + Default,
{
    /// Create a new matrix using `T::default()` as an initializer.
    #[inline]
    fn default() -> Self {
        Self::repeat(T::default())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Uninit related methods
////////////////////////////////////////////////////////////////////////////////

impl<T, const M: usize, const N: usize> Matrix<MaybeUninit<T>, M, N> {
    /// Create a new matrix with uninitialized contents.
    #[inline]
    fn uninit() -> Self {
        // Safety: The `assume_init` is safe because the type we are claiming to
        // have initialized here is a bunch of `MaybeUninit`s, which do not
        // require initialization. Additionally, `Matrix` is `repr(transparent)`
        // with an array of arrays.
        //
        // Note: this is not the most ideal way of doing this. In the future
        // when Rust allows inline const expressions we might be able to use
        // `Self { data: [const { MaybeUninit::<T>::uninit() }; M] ; N] }`
        //
        // See https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        let matrix = MaybeUninit::uninit();
        unsafe { matrix.assume_init() }
    }

    /// Assumes the data is initialized and extracts each element as `T`.
    ///
    /// # Safety
    ///
    /// As with [`MaybeUninit::assume_init`], it is up to the caller to
    /// guarantee that the matrix is really is in an initialized state. Calling
    /// this when the contents are not yet fully initialized causes immediate
    /// undefined behavior.
    #[inline]
    unsafe fn assume_init(self) -> Matrix<T, M, N> {
        // Safety: The caller is responsible for all the elements being
        // initialized. Additionally, we know that `T` is the same size as
        // `MaybeUninit<T>`.
        //
        // Note: this is not the most ideal way of doing this. We should be able
        // to use [`mem::transmute()`] but it doesn't work yet :(.
        let src = &self as *const Self as *const Matrix<T, M, N>;
        let matrix = unsafe { ptr::read(src) };
        mem::forget(self);
        matrix
    }
}

////////////////////////////////////////////////////////////////////////////////
// FromIterator
////////////////////////////////////////////////////////////////////////////////

/// Pulls `M * N` items from `iter` and fills a matrix. If the iterator yields
/// fewer than `M * N` items, `Err(_)` is returned and all already yielded items
/// are dropped.
///
/// If `iter.next()` panics, all items already yielded by the iterator are
/// dropped.
pub fn collect<I, const M: usize, const N: usize>(iter: I) -> Result<Matrix<I::Item, M, N>, usize>
where
    I: IntoIterator,
{
    struct Guard<T> {
        ptr: *mut T,
        len: usize,
    }

    impl<T> Drop for Guard<T> {
        fn drop(&mut self) {
            let partial = ptr::slice_from_raw_parts_mut(self.ptr, self.len);
            // Safety: this raw slice will contain only the initialized objects.
            unsafe {
                ptr::drop_in_place(partial);
            }
        }
    }

    let mut iter = iter.into_iter();
    let mut matrix: Matrix<MaybeUninit<_>, M, N> = Matrix::uninit();
    let mut guard = Guard {
        ptr: matrix.as_mut_slice().as_mut_ptr(),
        len: 0,
    };
    while let Some(item) = iter.next() {
        matrix[guard.len] = MaybeUninit::new(item);
        guard.len += 1;
        if guard.len == M * N {
            mem::forget(guard);
            // Safety: the condition above asserts that all elements are
            // initialized.
            return Ok(unsafe { matrix.assume_init() });
        }
    }
    Err(guard.len)
}

impl<T, const M: usize, const N: usize> FromIterator<T> for Matrix<T, M, N> {
    /// Create a new matrix from an iterator.
    ///
    /// Elements will be filled in column-major order.
    ///
    /// # Panics
    ///
    /// If the iterator doesn't yield enough elements to fill the matrix.
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        collect(iter).unwrap_or_else(|len| {
            panic!(
                "collect iterator of length {} into `Matrix<_, {}, {}>`",
                len, M, N
            );
        })
    }
}
