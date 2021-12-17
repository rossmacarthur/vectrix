# vectrix

[![Crates.io Version](https://img.shields.io/crates/v/vectrix.svg)](https://crates.io/crates/vectrix)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/vectrix)
[![Build Status](https://img.shields.io/github/workflow/status/rossmacarthur/vectrix/build/trunk)](https://github.com/rossmacarthur/vectrix/actions?query=workflow%3Abuild)

This crate provides a stack-allocated, constant-size [`Matrix<T, M, N>`][struct.Matrix]
type implemented using const generics.

## 🚀 Getting started

Add the following to your Cargo manifest.

```toml
[dependencies]
vectrix = "0.2"
```

`no_std` is also supported by disabling the default std feature.

```toml
[dependencies]
vectrix = { version = "0.2", default-features = false, features = ["macro"] }
```

## 🤸 Usage

### Types

The base [`Matrix<T, M, N>`][struct.Matrix] type represents a matrix with `M` rows and `N`
columns. This type is a backed by an array of arrays. The data is stored in
column-major order. Some convenient aliases are provided for common
matrices, like vectors.

* [`Matrix<T, M, N>`][struct.Matrix] → a generic matrix type with `M` rows and `N` columns.
* [`Vector<T, M>`][struct.Vector] → a column vector with `M` rows.
* [`RowVector<T, N>`][struct.RowVector] → a row vector with `N` columns.

### Macros

Macros are provided for easy construction of the provided types. These
macros will also work in `const` contexts.

* The [`matrix!`][macro.matrix] macro can be used to construct a new [`Matrix`][struct.Matrix] of any
  size.
  
  ```rust
  let matrix = matrix![
      1, 3, 5;
      2, 4, 6;
  ];
  ```
  
  In the above example `matrix` is a `Matrix<_, 2, 3>` type, having 2 rows and
  3 columns.

* The [`vector!`][macro.vector] and [`row_vector!`][macro.row_vector] macros can be used to to construct
  column and row vectors respectively.
  
  ```rust
  let vector = vector![1, 3, 3, 7];
  //  ^^^^^^ type `Vector<_, 4>`
  assert_eq!(vector, matrix![1; 3; 3; 7]);
  
  let vector = row_vector![1, 3, 3, 7];
  //  ^^^^^^ type `RowVector<_, 4>`
  assert_eq!(vector, matrix![1, 3, 3, 7]);
  ```

### Constructors

Commonly used constructors are listed below.

* [`::zero()`][struct.Matrix::zero] → constructs a new matrix filled with
  [`T::zero()`][Zero::zero].
* [`::identity()`][struct.Matrix::identity] → constructs a new identity matrix.
* [`::repeat(..)`][struct.Matrix::repeat] → constructs a new matrix filled with
  the provided value.
* [`::repeat_with(..)`][struct.Matrix::repeat_with] → constructs a new matrix
  filled with values computed by the provided closure.
* [`::from_iter(..)`][core::iter::FromIterator::from_iter] → constructs a
  new matrix from an iterator.
* [`::new(..)`][struct.Matrix::new] → constructs a new vector using the
  provided components.

### Accessing elements

Three types of element access are available.

* `usize` indexing selects the nth element in the matrix as viewed in
  column-major order.
  
  ```rust
  let matrix = matrix![
      1, 2, 3;
      4, 5, 6;
  ];
  assert_eq!(matrix[1], 4);
  ```

* `(usize, usize)` indexing selects the element at a particular row and
  column position.
  
  ```rust
  let matrix = matrix![
      1, 2, 3;
      4, 5, 6;
  ];
  assert_eq!(matrix[(1, 0)], 4);
  ```

* Component accessors are available for small vectors using traditional
  names.
  
  ```rust
  let mut vector = vector![1, 2, 3, 4, 0, 0];
  vector.y = 3;
  vector.w = 7;
  assert_eq!(vector.x, 1);
  assert_eq!(vector.y, 3);
  assert_eq!(vector.z, 3);
  assert_eq!(vector.w, 7);
  assert_eq!(vector.a, 0);
  assert_eq!(vector.b, 0);
  ```

### Accessing a row or column

You can get a reference to particular row or column using the
[`.row()`][struct.Matrix::row] or [`.column()`][struct.Matrix::column] methods. You
can get a mutable reference using the `_mut` variants.

```rust
let mut matrix = matrix![
    1, 2, 3;
    4, 7, 6;
];
let row = matrix.row_mut(1);
row[1] = 5;
assert_eq!(matrix.column(1), &[2, 5]);
```

### Iteration

Element-wise, column-major order iteration is provided using the following
methods.

* [`.into_iter()`][struct.Matrix::into_iter] → consumes the matrix and returns
  an owned iterator over each element.
* [`.iter()`][struct.Matrix::iter] → returns an iterator over a reference to
  each element.
* [`.iter_mut()`][struct.Matrix::iter_mut] → returns an iterator over a mutable
  reference to each element.

Iteration over rows and columns is provide using the following methods.

* [`.iter_rows()`][struct.Matrix::iter_rows] → returns an iterator over a
  reference to each row.
* [`.iter_rows_mut()`][struct.Matrix::iter_rows_mut] → returns an iterator over
  mutable reference to each row.
* [`.iter_columns()`][struct.Matrix::iter_columns] → returns an iterator over a
  reference to each column.
* [`.iter_columns_mut()`][struct.Matrix::iter_columns_mut] → returns an
  iterator over a mutable reference to each column.

### Slice representation

A slice view of the underlying data is provided using
[`.as_slice()`][struct.Matrix::as_slice] and
[`.as_mut_slice()`][struct.Matrix::as_mut_slice].

```rust
let mut matrix = matrix![
    1, 3, 5;
    2, 3, 6;
];
matrix.as_mut_slice()[3] = 4;
assert_eq!(matrix.as_slice(), &[1, 2, 3, 4, 5, 6]);
```

### Debug

The [`Debug`][core::fmt::Debug] implementation will print out vectors as
lists and matrices as a list of lists in column-major order.

```rust
let v = vector![1.1, 2.0];
let m = matrix![1, 2; 3, 4];
println!("vector: {:.2?}", v);
println!("matrix: {:?}", m);
```

This will output:

```text
vector: [1.10, 2.00]
matrix: [[1, 3], [2, 4]]
```

### Display

The [`Display`][core::fmt::Display] implementation will print out the
matrix in the traditional box bracket format. Precision is supported as well
as most of the other formatting traits like
[`LowerHex`][core::fmt::LowerHex].

```rust
let cv = vector![1.1, 2.0];
let rv = row_vector![1.1, 2.0];
let m = matrix![1, 2; 3, 4];
println!("column vector: {:.2}", cv);
println!("row vector: {:.1}", rv);
println!("matrix: {:b}", m);
```

This will output:

```text
column vector:
 ┌      ┐
 │ 1.10 │
 │ 2.00 │
 └      ┘

row vector:
 ┌          ┐
 │ 1.1  2.0 │
 └          ┘

matrix:
 ┌         ┐
 │  1   10 │
 │ 11  100 │
 └         ┘
```

### Operations

[`Matrix`][struct.Matrix] implements many built-in operators. With scalar operands almost
all operators are implemented and they simply apply the operation to each
element in the matrix. Unary operators will do the equivalent. In the
following example each element in the matrix is multiplied by 2.

```rust
let matrix = matrix![
    1, -3;
    3, -7;
];
let expected = matrix![
    2, -6;
    6, -14;
];
assert_eq!(matrix * 2, expected);
```

[`Matrix`][struct.Matrix] supports addition and subtraction with same size matrices for
element-wise addition and subtraction. In the following example a matrix
is added to itself.

```rust
let matrix = matrix![
    1, -3;
    3, -7;
];
let expected = matrix![
    2, -6;
    6, -14;
];
assert_eq!(matrix + matrix, expected);
```

[core::fmt::Debug]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[core::fmt::Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[core::fmt::LowerHex]: https://doc.rust-lang.org/std/fmt/trait.LowerHex.html
[core::iter::FromIterator::from_iter]: https://doc.rust-lang.org/std/iter/trait.FromIterator.html#tymethod.from_iter
[macro.matrix]: https://docs.rs/vectrix/0.2/vectrix/macro.matrix.html
[macro.row_vector]: https://docs.rs/vectrix/0.2/vectrix/macro.row_vector.html
[macro.vector]: https://docs.rs/vectrix/0.2/vectrix/macro.vector.html
[struct.Matrix]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html
[struct.Matrix::as_mut_slice]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.as_mut_slice
[struct.Matrix::as_slice]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.as_slice
[struct.Matrix::column]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.column
[struct.Matrix::identity]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.identity
[struct.Matrix::into_iter]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.into_iter
[struct.Matrix::iter]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.iter
[struct.Matrix::iter_columns]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.iter_columns
[struct.Matrix::iter_columns_mut]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.iter_columns_mut
[struct.Matrix::iter_mut]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.iter_mut
[struct.Matrix::iter_rows]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.iter_rows
[struct.Matrix::iter_rows_mut]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.iter_rows_mut
[struct.Matrix::new]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.new
[struct.Matrix::repeat]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.repeat
[struct.Matrix::repeat_with]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.repeat_with
[struct.Matrix::row]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.row
[struct.Matrix::zero]: https://docs.rs/vectrix/0.2/vectrix/struct.Matrix.html#method.zero
[struct.RowVector]: https://docs.rs/vectrix/0.2/vectrix/struct.RowVector.html
[struct.Vector]: https://docs.rs/vectrix/0.2/vectrix/struct.Vector.html
[Zero::zero]: https://docs.rs/vectrix/0.2/vectrix/traits/trait.Zero.html#tymethod.zero


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
