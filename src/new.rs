//! Generic constructors.

use core::iter::FromIterator;
use core::mem::{self, MaybeUninit};
use core::{hint, ptr};

use crate::Matrix;

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

impl<T: Default, const M: usize, const N: usize> Default for Matrix<T, M, N> {
    /// Create a new matrix using `T::default()` as an initializer.
    ///
    /// **Note:** this implementation will not be as efficient for types that
    /// are `Copy`. In most cases it would be better to use one of the
    /// following:
    /// - [`Matrix::zero()`][Matrix::zero]
    /// - [`Matrix::repeat(T::default())`][Matrix::repeat]
    #[inline]
    fn default() -> Self {
        Self::repeat_with(T::default)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Uninit related methods
////////////////////////////////////////////////////////////////////////////////

/// Size-heterogeneous transmutation.
///
/// This is required because the compiler doesn't yet know how to deal with the
/// size of const arrays. We should be able to use [`mem::transmute()`] but it
/// doesn't work yet :(.
#[inline]
pub unsafe fn transmute_unchecked<A, B>(a: A) -> B {
    let b = unsafe { ptr::read(&a as *const A as *const B) };
    mem::forget(a);
    b
}

impl<T, const M: usize, const N: usize> Matrix<MaybeUninit<T>, M, N> {
    /// Create a new matrix with uninitialized contents.
    #[inline]
    pub(crate) fn uninit() -> Self {
        // SAFETY: The `assume_init` is safe because the type we are claiming to
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
    pub(crate) unsafe fn assume_init(self) -> Matrix<T, M, N> {
        // SAFETY: The caller is responsible for all the elements being
        // initialized. Additionally, we know that `T` is the same size as
        // `MaybeUninit<T>`.
        unsafe { transmute_unchecked(self) }
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
pub fn collect<I, T, const M: usize, const N: usize>(iter: I) -> Result<Matrix<T, M, N>, usize>
where
    I: IntoIterator<Item = T>,
{
    struct Guard<U> {
        ptr: *mut U,
        len: usize,
    }

    impl<U> Drop for Guard<U> {
        fn drop(&mut self) {
            let partial = ptr::slice_from_raw_parts_mut(self.ptr, self.len);
            // SAFETY: this raw slice will contain only the initialized objects.
            unsafe {
                ptr::drop_in_place(partial);
            }
        }
    }

    let mut matrix: Matrix<MaybeUninit<T>, M, N> = Matrix::uninit();
    let mut guard = Guard {
        ptr: matrix.as_mut_ptr() as *mut T,
        len: 0,
    };
    for item in iter {
        let dst = unsafe { matrix.get_unchecked_mut(guard.len) };
        *dst = MaybeUninit::new(item);
        guard.len += 1;
        if guard.len == M * N {
            mem::forget(guard);
            // SAFETY: the condition above asserts that all elements are
            // initialized.
            return Ok(unsafe { matrix.assume_init() });
        }
    }
    Err(guard.len)
}

/// Like [`collect()`] except the caller must guarantee that the iterator will
/// yield enough elements to fill the matrix.
pub unsafe fn collect_unchecked<I, T, const M: usize, const N: usize>(iter: I) -> Matrix<T, M, N>
where
    I: IntoIterator<Item = T>,
{
    match collect(iter) {
        Ok(matrix) => matrix,
        Err(_) => {
            // SAFETY: the caller guarantees the iterator will yield enough
            // elements, so this error case can never be reached.
            unsafe { hint::unreachable_unchecked() }
        }
    }
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
