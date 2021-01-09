# vect*rs*

[![Crates.io Version](https://img.shields.io/crates/v/vectrs.svg)](https://crates.io/crates/vectrs)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/vectrs)
[![Build Status](https://img.shields.io/github/workflow/status/rossmacarthur/vectrs/build/master)](https://github.com/rossmacarthur/vectrs/actions?query=workflow%3Abuild)

A stack-allocated, constant-size, *n*-dimensional vector type implemented with
const generics.

This crate will work on stable Rust from Rust v1.51 onwards.

## Example usage

```rust
use std::iter;
use vectrs::{vector, Vector};

// construct using the `vector!` macro, from arrays, tuples, or even iterators
let v1 = vector![-1, 0]; // shortcut for Vector::new([-1, 0])
let v2 = Vector::from((4, 2));
let v3: Vector<_, 2> = iter::repeat(2).collect();

// numeric operations are implemented
assert_eq!(v1 * 2 + v2, v3);

// access/mutate components using dedicated accessors or indexing
assert_eq!(v3.x, 2);
assert_eq!(v3[1], 2);
```

See the [full documentation](https://docs.rs/vectrs) for more.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
