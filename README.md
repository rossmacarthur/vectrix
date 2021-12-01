# vectrix

[![Crates.io Version](https://img.shields.io/crates/v/vectrix.svg)](https://crates.io/crates/vectrix)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/vectrix)
[![Build Status](https://img.shields.io/github/workflow/status/rossmacarthur/vectrix/build/trunk)](https://github.com/rossmacarthur/vectrix/actions?query=workflow%3Abuild)

A stack-allocated, constant-size [`Matrix<T, M, N>`] type implemented with const
generics.

### Features

- Stable Rust.
- Generic matrix and vector types.
- `matrix!` / `vector!` macros that can be used in `const` contexts.
- Element access using `usize` and `(usize, usize)` indexing.
- Component access for small vectors (`.x`, `.y`, etc.).
- Row and column views of matrices.
- Iteration over elements, rows, and columns.
- Implementations of scalar operands for any size vectors and matrices.
- Implementations of addition, subtraction, and multiplication for
  appropriately sized vectors and matrices.
- And more ... see the [full documentation][docs].

[docs]: https://docs.rs/vectrix
[`Matrix<T, M, N>`]: https://docs.rs/vectrix/0.1/vectrix/struct.Matrix.html

### Example usage

The following demonstrates matrix multiplication.

```rust
use vectrix::{matrix, row_vector, vector};

let v1 = vector![1, 2, 3];     // same as `matrix![1; 2; 3]`
let v2 = row_vector![4, 5, 6]; // same as `matrix![4, 5, 6]`

assert_eq!(
    v1 * v2,
    matrix![
         4,  5, 6;
         8, 10, 12;
        12, 15, 18;
    ]
);
assert_eq!(v2 * v1, matrix![32]);
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
