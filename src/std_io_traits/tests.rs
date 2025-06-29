use super::{Save, Load};

#[test]
fn usage_on_readme() {
    use crate::{Save, Load};

    let original: Vec<i32> = vec![1, 2, 3, 4, 5];

    // (1) Save
    let mut buf = Vec::new();
    original.save_as_ne(&mut buf).unwrap();
    assert_eq!(original.encoded_len(), buf.len()); // size can be estimated
    
    // (2) Load
    let decoded = Vec::<i32>::load_as_ne(&mut &buf[..]).unwrap();
    assert_eq!(original, decoded);
}



#[test]
fn are_equal_saved_and_loaded_for_vector() {
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

                    vec.save_as_le(&mut buffer_le).unwrap();
                    vec.save_as_be(&mut buffer_be).unwrap();

                    let loaded_le = Vec::<$ty>::load_as_le(&mut &buffer_le[..]).unwrap();
                    let loaded_be = Vec::<$ty>::load_as_be(&mut &buffer_be[..]).unwrap();

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

#[test]
fn is_accurate_encoded_len_for_vector() {
    for n in 1..30 {
        macro_rules! test {
            ($ty:ty) => {
                let mut buffer_le: Vec<u8> = Vec::new();
                let mut buffer_be: Vec<u8> = Vec::new();

                let vec_to_save: Vec<$ty> = vec![0; n];
                vec_to_save.save_as_le(&mut buffer_le).unwrap();
                vec_to_save.save_as_be(&mut buffer_be).unwrap();

                assert_eq!(vec_to_save.encoded_len(), buffer_le.len());
                assert_eq!(vec_to_save.encoded_len(), buffer_be.len());
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

#[test]
fn are_equal_saved_and_loaded_for_array() {
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

                        array.save_as_le(&mut buffer_le).unwrap();
                        array.save_as_be(&mut buffer_be).unwrap();

                        let loaded_le = <[$ty; $size]>::load_as_le(&mut &buffer_le[..]).unwrap();
                        let loaded_be = <[$ty; $size]>::load_as_be(&mut &buffer_be[..]).unwrap();
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

#[test]
fn is_accurate_encoded_len_for_array() {
    macro_rules! test {
        ($ty:ty, [ $($size:literal),+ ]) => {
            $(
                {
                    let array: [$ty; $size] = [0; $size];
                    let mut buffer_le = Vec::new();
                    let mut buffer_be = Vec::new();

                    array.save_as_le(&mut buffer_le).unwrap();
                    array.save_as_be(&mut buffer_be).unwrap();

                    assert_eq!(array.encoded_len(), buffer_le.len());
                    assert_eq!(array.encoded_len(), buffer_be.len());
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

#[test]
fn are_equal_saved_and_loaded_for_tuples() {
    use rand::Rng;
    let mut rng = rand::rng();
    let n = 100;

    for _ in 0..n {
        // Test tuple of size 2
        {
            let tuple = (rng.random::<u32>(), rng.random::<i64>());
            let mut buffer_le = Vec::new();
            let mut buffer_be = Vec::new();

            tuple.save_as_le(&mut buffer_le).unwrap();
            tuple.save_as_be(&mut buffer_be).unwrap();

            let loaded_le = <(u32, i64)>::load_as_le(&mut &buffer_le[..]).unwrap();
            let loaded_be = <(u32, i64)>::load_as_be(&mut &buffer_be[..]).unwrap();

            assert_eq!(tuple, loaded_le);
            assert_eq!(tuple, loaded_be);
        }

        // Test tuple of size 3
        {
            let tuple = (rng.random::<u8>(), rng.random::<i16>(), rng.random::<u32>());
            let mut buffer_le = Vec::new();
            let mut buffer_be = Vec::new();

            tuple.save_as_le(&mut buffer_le).unwrap();
            tuple.save_as_be(&mut buffer_be).unwrap();

            let loaded_le = <(u8, i16, u32)>::load_as_le(&mut &buffer_le[..]).unwrap();
            let loaded_be = <(u8, i16, u32)>::load_as_be(&mut &buffer_be[..]).unwrap();

            assert_eq!(tuple, loaded_le);
            assert_eq!(tuple, loaded_be);
        }

        // Test tuple of size 4
        {
            let tuple = (rng.random::<u16>(), rng.random::<i32>(), rng.random::<u64>(), rng.random::<i8>());
            let mut buffer_le = Vec::new();
            let mut buffer_be = Vec::new();

            tuple.save_as_le(&mut buffer_le).unwrap();
            tuple.save_as_be(&mut buffer_be).unwrap();

            let loaded_le = <(u16, i32, u64, i8)>::load_as_le(&mut &buffer_le[..]).unwrap();
            let loaded_be = <(u16, i32, u64, i8)>::load_as_be(&mut &buffer_be[..]).unwrap();

            assert_eq!(tuple, loaded_le);
            assert_eq!(tuple, loaded_be);
        }
    }
}

#[test]
fn is_accurate_encoded_len_for_tuples() {
    let tuple2 = (1u32, 2i64);
    let tuple3 = (1u8, 2i16, 3u32);
    let tuple4 = (1u16, 2i32, 3u64, 4i8);

    let mut buffer_le = Vec::new();
    let mut buffer_be = Vec::new();

    // Test tuple of size 2
    tuple2.save_as_le(&mut buffer_le).unwrap();
    tuple2.save_as_be(&mut buffer_be).unwrap();
    assert_eq!(tuple2.encoded_len(), buffer_le.len());
    assert_eq!(tuple2.encoded_len(), buffer_be.len());
    buffer_le.clear();
    buffer_be.clear();

    // Test tuple of size 3
    tuple3.save_as_le(&mut buffer_le).unwrap();
    tuple3.save_as_be(&mut buffer_be).unwrap();
    assert_eq!(tuple3.encoded_len(), buffer_le.len());
    assert_eq!(tuple3.encoded_len(), buffer_be.len());
    buffer_le.clear();
    buffer_be.clear();

    // Test tuple of size 4
    tuple4.save_as_le(&mut buffer_le).unwrap();
    tuple4.save_as_be(&mut buffer_be).unwrap();
    assert_eq!(tuple4.encoded_len(), buffer_le.len());
    assert_eq!(tuple4.encoded_len(), buffer_be.len());
}
