use crate::{Save, Load};

#[test]
fn usage_on_readme() {
    use crate::{Save, Load};

    let vec_to_save: Vec<i32> = vec![1, 2, 3, 4, 5];
    
    // (1) Save
    let mut buffer = Vec::new();
    vec_to_save.save_to(&mut buffer).unwrap();
    assert_eq!(vec_to_save.to_be_saved_size(), buffer.len()); // size can be estimated
    
    // (2) Load
    let vec_loaded = Vec::<i32>::load_from(&mut &buffer[..]).unwrap();
    
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
                    let mut buffer = Vec::new();

                    vec.save_to(&mut buffer).unwrap();

                    let loaded = Vec::<$ty>::load_from(&mut &buffer[..]).unwrap();

                    assert_eq!(vec, loaded);
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
fn is_accurate_to_be_saved_size_for_vector() {
    for n in 1..30 {
        macro_rules! test {
            ($ty:ty) => {
                let mut buffer: Vec<u8> = Vec::new();
                let vec_to_save: Vec<$ty> = vec![0; n];
                vec_to_save.save_to(&mut buffer).unwrap();
                assert_eq!(vec_to_save.to_be_saved_size(), buffer.len());
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
                        // 여기서 $size는 컴파일 타임 상수
                        let array: [$ty; $size] = [rng.gen(); $size];
                        let mut buffer = Vec::new();
                        array.save_to(&mut buffer).unwrap();
                        let loaded = <[$ty; $size]>::load_from(&mut &buffer[..]).unwrap();
                        assert_eq!(array, loaded, "Failed at size {}", $size);
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