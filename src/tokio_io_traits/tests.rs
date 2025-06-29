use std::pin::Pin;
use super::{AsyncSave, AsyncLoad};

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn usage_on_readme() {
    use crate::{AsyncSave, AsyncLoad};
    use std::pin::Pin;

    let original: Vec<i32> = vec![1, 2, 3, 4, 5];
    
    // (1) Save
    let mut buf = Vec::new();
    original.save_as_ne(Pin::new(&mut buf)).await.unwrap();

    // (2) Load
    let decoded = Vec::<i32>::load_as_ne(Pin::new(&mut &buf[..])).await.unwrap();
    assert_eq!(original, decoded);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn are_equal_saved_and_loaded_for_vector() {
    use rand::Rng;
    let mut rng = rand::rng();

    let vec_len_list: Vec<usize> = (0..10).map(|v| 2_i32.pow(v) as usize).collect();
    let n = 100;

    for len in vec_len_list {
        for _ in 0..n {
            macro_rules! test {
                ($ty:ty, $sampling_ty:ty) => {
                    let vec: Vec<$ty> = (0..len).map(|_| { rng.random::<$sampling_ty>() as $ty }).collect();
                    let mut buffer_le = Vec::new();
                    let mut buffer_be = Vec::new();

                    vec.save_as_le(Pin::new(&mut buffer_le)).await.unwrap();
                    vec.save_as_be(Pin::new(&mut buffer_be)).await.unwrap();

                    let loaded_le = Vec::<$ty>::load_as_le(Pin::new(&mut &buffer_le[..])).await.unwrap();
                    let loaded_be = Vec::<$ty>::load_as_be(Pin::new(&mut &buffer_be[..])).await.unwrap();

                    assert_eq!(vec, loaded_le);
                    assert_eq!(vec, loaded_be);
                };
            }
            
            test!(u8, u8);
            test!(u16, u16);
            test!(u32, u32);
            test!(u64, u64);
            test!(u128, u128);
            test!(usize, u32);
            test!(usize, u64);
            test!(i8, i8);
            test!(i16, i16);
            test!(i32, i32);
            test!(i64, i64);
            test!(i128, i128);
            test!(isize, i32);
            test!(isize, i64);
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn are_equal_size_as_std_io_for_vector() {
    use crate::std_io_traits::Save;
    for n in 1..30 {
        macro_rules! test {
            ($ty:ty) => {
                let mut buffer_le: Vec<u8> = Vec::new();
                let mut buffer_le_std_io: Vec<u8> = Vec::new();
                let mut buffer_be: Vec<u8> = Vec::new();
                let mut buffer_be_std_io: Vec<u8> = Vec::new();

                let vec_to_save: Vec<$ty> = vec![0; n];
                AsyncSave::save_as_le(&vec_to_save, Pin::new(&mut buffer_le)).await.unwrap();
                Save::save_as_le(&vec_to_save, &mut buffer_le_std_io).unwrap();
                AsyncSave::save_as_be(&vec_to_save, Pin::new(&mut buffer_be)).await.unwrap();
                Save::save_as_be(&vec_to_save, &mut buffer_be_std_io).unwrap();

                assert_eq!(buffer_le_std_io.len(), buffer_le.len());
                assert_eq!(buffer_le_std_io.len(), buffer_be_std_io.len());
            };
        }

        test!(u8);
        test!(u16);
        test!(u32);
        test!(u64);
        test!(u128);
        test!(usize);
        test!(i8);
        test!(i16);
        test!(i32);
        test!(i64);
        test!(i128);
        test!(isize);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn are_equal_saved_and_loaded_for_array() {
    use rand::Rng;
    let mut rng = rand::rng();
    let n = 100;

    for _ in 0..n {
        macro_rules! test {
            ($ty:ty, $sampling_ty:ty, [ $($size:literal),+ ]) => {
                $(
                    {
                        let array: [$ty; $size] = [rng.random::<$sampling_ty>() as $ty; $size];
                        let mut buffer_le = Vec::new();
                        let mut buffer_be = Vec::new();

                        array.save_as_le(Pin::new(&mut buffer_le)).await.unwrap();
                        array.save_as_be(Pin::new(&mut buffer_be)).await.unwrap();

                        let loaded_le = <[$ty; $size]>::load_as_le(Pin::new(&mut &buffer_le[..])).await.unwrap();
                        let loaded_be = <[$ty; $size]>::load_as_be(Pin::new(&mut &buffer_be[..])).await.unwrap();
                        assert_eq!(array, loaded_le, "Failed at size {}", $size);
                        assert_eq!(array, loaded_be, "Failed at size {}", $size);
                    }
                )*
            }
        }
            
        test!(u8, u8, [1, 2, 4, 8, 16, 32, 64]);
        test!(u16, u16, [1, 2, 4, 8, 16, 32, 64]);
        test!(u32, u32, [1, 2, 4, 8, 16, 32, 64]);
        test!(u64, u64, [1, 2, 4, 8, 16, 32, 64]);
        test!(u128, u128, [1, 2, 4, 8, 16, 32, 64]);
        test!(usize, u32, [1, 2, 4, 8, 16, 32, 64]);
        test!(usize, u64, [1, 2, 4, 8, 16, 32, 64]);
        test!(i8, i8, [1, 2, 4, 8, 16, 32, 64]);
        test!(i16, i16, [1, 2, 4, 8, 16, 32, 64]);
        test!(i32, i32, [1, 2, 4, 8, 16, 32, 64]);
        test!(i64, i64, [1, 2, 4, 8, 16, 32, 64]);
        test!(i128, i128, [1, 2, 4, 8, 16, 32, 64]);
        test!(isize, i32, [1, 2, 4, 8, 16, 32, 64]);
        test!(isize, i64, [1, 2, 4, 8, 16, 32, 64]);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn are_equal_size_as_std_io_for_array() {
    use crate::std_io_traits::Save;
    macro_rules! test {
        ($ty:ty, [ $($size:literal),+ ]) => {
            $(
                {
                    let array: [$ty; $size] = [0; $size];
                    let mut buffer_le = Vec::new();
                    let mut buffer_le_std_io = Vec::new();
                    let mut buffer_be = Vec::new();
                    let mut buffer_be_std_io = Vec::new();

                    AsyncSave::save_as_le(&array, Pin::new(&mut buffer_le)).await.unwrap();
                    Save::save_as_le(&array, &mut buffer_le_std_io).unwrap();
                    AsyncSave::save_as_be(&array, Pin::new(&mut buffer_be)).await.unwrap();
                    Save::save_as_be(&array, &mut buffer_be_std_io).unwrap();

                    assert_eq!(buffer_le_std_io.len(), buffer_le.len());
                    assert_eq!(buffer_be_std_io.len(), buffer_be.len());
                }
            )*
        }
    }

    test!(u8, [1, 2, 4, 8, 16, 32, 64]);
    test!(u16, [1, 2, 4, 8, 16, 32, 64]);
    test!(u32, [1, 2, 4, 8, 16, 32, 64]);
    test!(u64, [1, 2, 4, 8, 16, 32, 64]);
    test!(u128, [1, 2, 4, 8, 16, 32, 64]);
    test!(usize, [1, 2, 4, 8, 16, 32, 64]);
    test!(i8, [1, 2, 4, 8, 16, 32, 64]);
    test!(i16, [1, 2, 4, 8, 16, 32, 64]);
    test!(i32, [1, 2, 4, 8, 16, 32, 64]);
    test!(i64, [1, 2, 4, 8, 16, 32, 64]);
    test!(i128, [1, 2, 4, 8, 16, 32, 64]);
    test!(isize, [1, 2, 4, 8, 16, 32, 64]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn are_equal_saved_and_loaded_for_tuples() {
    use rand::Rng;
    let mut rng = rand::rng();
    let n = 100;

    for _ in 0..n {
        // Test tuple of size 2
        {
            let tuple = (rng.random::<u32>(), rng.random::<i64>());
            let mut buffer_le = Vec::new();
            let mut buffer_be = Vec::new();

            tuple.save_as_le(Pin::new(&mut buffer_le)).await.unwrap();
            tuple.save_as_be(Pin::new(&mut buffer_be)).await.unwrap();

            let loaded_le = <(u32, i64)>::load_as_le(Pin::new(&mut &buffer_le[..])).await.unwrap();
            let loaded_be = <(u32, i64)>::load_as_be(Pin::new(&mut &buffer_be[..])).await.unwrap();

            assert_eq!(tuple, loaded_le);
            assert_eq!(tuple, loaded_be);
        }

        // Test tuple of size 3
        {
            let tuple = (rng.random::<u8>(), rng.random::<i16>(), rng.random::<u32>());
            let mut buffer_le = Vec::new();
            let mut buffer_be = Vec::new();

            tuple.save_as_le(Pin::new(&mut buffer_le)).await.unwrap();
            tuple.save_as_be(Pin::new(&mut buffer_be)).await.unwrap();

            let loaded_le = <(u8, i16, u32)>::load_as_le(Pin::new(&mut &buffer_le[..])).await.unwrap();
            let loaded_be = <(u8, i16, u32)>::load_as_be(Pin::new(&mut &buffer_be[..])).await.unwrap();

            assert_eq!(tuple, loaded_le);
            assert_eq!(tuple, loaded_be);
        }

        // Test tuple of size 4
        {
            let tuple = (rng.random::<u16>(), rng.random::<i32>(), rng.random::<u64>(), rng.random::<i8>());
            let mut buffer_le = Vec::new();
            let mut buffer_be = Vec::new();

            tuple.save_as_le(Pin::new(&mut buffer_le)).await.unwrap();
            tuple.save_as_be(Pin::new(&mut buffer_be)).await.unwrap();

            let loaded_le = <(u16, i32, u64, i8)>::load_as_le(Pin::new(&mut &buffer_le[..])).await.unwrap();
            let loaded_be = <(u16, i32, u64, i8)>::load_as_be(Pin::new(&mut &buffer_be[..])).await.unwrap();

            assert_eq!(tuple, loaded_le);
            assert_eq!(tuple, loaded_be);
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn are_equal_size_as_std_io_for_tuples() {
    use crate::std_io_traits::Save;

    // Test tuple of size 2
    {
        let tuple = (1u32, 2i64);
        let mut buffer_le = Vec::new();
        let mut buffer_le_std_io = Vec::new();
        let mut buffer_be = Vec::new();
        let mut buffer_be_std_io = Vec::new();

        AsyncSave::save_as_le(&tuple, Pin::new(&mut buffer_le)).await.unwrap();
        Save::save_as_le(&tuple, &mut buffer_le_std_io).unwrap();
        AsyncSave::save_as_be(&tuple, Pin::new(&mut buffer_be)).await.unwrap();
        Save::save_as_be(&tuple, &mut buffer_be_std_io).unwrap();

        assert_eq!(buffer_le_std_io.len(), buffer_le.len());
        assert_eq!(buffer_be_std_io.len(), buffer_be.len());
    }

    // Test tuple of size 3
    {
        let tuple = (1u8, 2i16, 3u32);
        let mut buffer_le = Vec::new();
        let mut buffer_le_std_io = Vec::new();
        let mut buffer_be = Vec::new();
        let mut buffer_be_std_io = Vec::new();

        AsyncSave::save_as_le(&tuple, Pin::new(&mut buffer_le)).await.unwrap();
        Save::save_as_le(&tuple, &mut buffer_le_std_io).unwrap();
        AsyncSave::save_as_be(&tuple, Pin::new(&mut buffer_be)).await.unwrap();
        Save::save_as_be(&tuple, &mut buffer_be_std_io).unwrap();

        assert_eq!(buffer_le_std_io.len(), buffer_le.len());
        assert_eq!(buffer_be_std_io.len(), buffer_be.len());
    }

    // Test tuple of size 4
    {
        let tuple = (1u16, 2i32, 3u64, 4i8);
        let mut buffer_le = Vec::new();
        let mut buffer_le_std_io = Vec::new();
        let mut buffer_be = Vec::new();
        let mut buffer_be_std_io = Vec::new();

        AsyncSave::save_as_le(&tuple, Pin::new(&mut buffer_le)).await.unwrap();
        Save::save_as_le(&tuple, &mut buffer_le_std_io).unwrap();
        AsyncSave::save_as_be(&tuple, Pin::new(&mut buffer_be)).await.unwrap();
        Save::save_as_be(&tuple, &mut buffer_be_std_io).unwrap();

        assert_eq!(buffer_le_std_io.len(), buffer_le.len());
        assert_eq!(buffer_be_std_io.len(), buffer_be.len());
    }
}
