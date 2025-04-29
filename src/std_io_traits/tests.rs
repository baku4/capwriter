use super::{Save, Load};

#[test]
fn usage_on_readme() {
    use crate::{Save, Load};

    let vec_to_save: Vec<i32> = vec![1, 2, 3, 4, 5];
    
    // (1) Save
    let mut buffer = Vec::new();
    vec_to_save.save_as_le(&mut buffer).unwrap();
    assert_eq!(vec_to_save.encoded_len(), buffer.len()); // size can be estimated
    
    // (2) Load
    let vec_loaded = Vec::<i32>::load_as_le(&mut &buffer[..]).unwrap();
    
    assert_eq!(vec_to_save, vec_loaded);
}

#[test]
fn are_equal_saved_and_loaded_for_vector() {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let vec_len_list: Vec<usize> = (0..10).map(|v| 2_i32.pow(v) as usize).collect();
    let n = 100;

    for len in vec_len_list {
        for _ in 0..n {
            macro_rules! test {
                ($ty:ty) => {
                    let vec: Vec<$ty> = (0..len).map(|_| { rng.gen() }).collect();
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
    let mut rng = rand::thread_rng();
    let n = 100;

    for _ in 0..n {
        macro_rules! test {
            ($ty:ty, [ $($size:literal),+ ]) => {
                $(
                    {
                        let array: [$ty; $size] = [rng.gen(); $size];
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
