# CapWriter
A fast serializer for `Vec`/slice with a lightweight cap" header.

## Usage
```rust
use capwriter::{Save, Load};

let original: Vec<i32> = vec![1, 2, 3, 4, 5];

// (1) Save
let mut buf = Vec::new();
original.save_as_ne(&mut buf)?;
assert_eq!(original.to_be_saved_size(), buf.len()); // size can be estimated

// (2) Load
let decoded = Vec::<i32>::load_as_ne(&mut &buffer).unwrap();
assert_eq!(original, decoded);
```

## Supported types
- trait `Save` can be used in:
  - `Vec<T>`, `&[T]`, `[T; usize]`
- trait `Load` can be used in:
  - `Vec<T>`, `[T; usize]`
- For the `T`: `()`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `f32`, `f64`, `Option<T>`, `PhantomData<T>`
  - impl the trait `Pod (Plain Old Data)` in `bytemuck`(https://crates.io/crates/bytemuck) crate.

## Bench
* Run bench with `cargo bench`
* For `Vec<usize>` length of 10,000,000 (for v0.2.0)

    |      | capwriter | with serialized data |
    |------|-----------|----------------------|
    | save | 21.483 ms |       25.506 ms      |
    | load | 12.001 ms |       77.664 ms      |

## Notes
### Cap size
Since v0.4, the header uses `u64` instead of `usize` so that data are portable across 32- and 64-bit platforms.
### Safety
`capwriter` does *no runtime type checking*. Supplying a buffer whose contents do not match the expected `T` results in *undefined behaviour* (panic, corrupted data, or excessive allocation). Ensure that the producer and consumer agree on the exact element type.
