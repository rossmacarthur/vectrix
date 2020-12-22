# vect*rs*

A stack-allocated, constant-size, *n*-dimensional vector type implemented with
const generics.

## Example usage

```rust
use vectrs::Vector;

// construct from arrays, tuples, iterators, etc
let v1 = Vector::from([-1, 0]);
let v2 = Vector::from((3, 2));
let v3: Vector<_, 2> = std::iter::repeat(2).collect();

// numeric operations are implemented
assert_eq!(v1 + v2, v3);

// access/mutate components using slice indexing or dedicated methods
assert_eq!(v2.x(), 3);
assert_eq!(v2[1], 2);
```

See the [full documentation](https://docs.rs/vectrs) for more.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
