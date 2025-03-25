# CapWriter
Fast saving and loading vector and slice via annotating "cap".

## Usage
```rust
use capwriter::{Save, Load};

let vec_to_save: Vec<i32> = vec![1, 2, 3, 4, 5];

// (1) Save
let mut buffer = Vec::new();
vec_to_save.save_to(&mut buffer).unwrap();
assert_eq!(vec_to_save.to_be_saved_size(), buffer.len()); // size can be estimated

// (2) Load
let vec_loaded = Vec::<i32>::load_from(&mut &buffer).unwrap();

assert_eq!(vec_to_save, vec_loaded);
```

## Supported types
- trait `Save` can be used in:
  - `Vec<T>`, `&[T]`
- trait `Load` can be used in:
  - `Vec<T>`
- For the `T`: `()`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `f32`, `f64`, `Option<T>`, `PhantomData<T>`
  - impl the trait `Pod (Plain Old Data)` in `bytemuck`(https://crates.io/crates/bytemuck) crate.

## Bench
* Run bench with `cargo bench`
* For `Vec<usize>` length of 10,000,000 (for v0.2.0)

    |      | capwriter | with serialized data |
    |------|-----------|----------------------|
    | save | 21.483 ms |       25.506 ms      |
    | load | 12.001 ms |       77.664 ms      |

## Caveat
This library provides fast saving and loading of vectors and slices in `Rust`. However, it is important to ensure that the input data matches the expected data type, as there is **NO** safe guard for type assertion. If the data type of the input data does not match the expected type, the following consequences may occur:
  1. The thread may panic.
  2. The data may be loaded successfully, but will not be usable due to the type mismatch.
  3. In some cases, the loader may allocate an entire memory space due to incorrect cap.

While scenario (2) and (3) are not common, it is important to be aware of the risks of using this library without proper type checking. It is recommended to thoroughly test the library with different types of data, and to handle errors appropriately in your application code to prevent panics and other unexpected behavior.
