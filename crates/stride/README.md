# stride

[![Crates.io Version](https://img.shields.io/crates/v/stride.svg)](https://crates.io/crates/stride)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/stride)
[![Build Status](https://img.shields.io/github/workflow/status/rossmacarthur/vectrix/build/trunk)](https://github.com/rossmacarthur/vectrix/actions?query=workflow%3Abuild)

A strided slice type.

## Getting started

This crate provides a slice-like [`Stride<T, S>`] type where elements are spaced
a constant `S` elements in memory.

For example, given an underlying slice `&[1, 2, 3, 4, 5, 6]`, the elements
`&[1, 3, 5]` are a strided slice with a stride of 2. This crate makes use of
const generics to provide the stride value `S` at compile time so that there is
no runtime memory overhead to strided slices; `Stride` takes up the same amount
of space as a slice.

Many slice-like operations are implemented for `Stride` including iteration and
indexing. Method names are similar to those of the slice type.

[`Stride<T, S>`]: https://docs.rs/stride/0.1/stride/struct.Stride.html

```rust
use stride::Stride;

// The underlying data.
let data = &mut [1, 2, 7, 4, 5, 6];

// Create a strided slice with a stride of `2` referring to
// elements `1`, `7`, and `5`.
let stride = Stride::<_, 2>::new_mut(data);

assert_eq!(stride.len(), 3);

// We can use indexing to view values ..
assert_eq!(stride[0], 1);
assert_eq!(stride[1..3], &[7, 5]);

// .. or modify them.
stride[1] = 3;
assert_eq!(stride, &[1, 3, 5]);

assert_eq!(data, &[1, 2, 3, 4, 5, 6]);
```

See the [API documentation](https://docs.rs/stride) for more.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
