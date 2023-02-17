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
    let vec_loaded = Vec::<i32>::load_from(
        std::io::Cursor::new(buffer)
    ).unwrap();
    
    assert_eq!(vec_to_save, vec_loaded);
}


#[test]
fn are_equal_saved_and_loaded_for_all_vector() {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let vec_len_list: Vec<usize> = (0..10).map(|v| 2_i32.pow(v) as usize).collect();
    let n = 100;

    for len in vec_len_list {
        for _ in 0..n {
            // u8
            {
                let vec: Vec<u8> = (0..len).map(|_| { rng.gen() }).collect();
                let mut buffer = Vec::new();

                vec.save_to(&mut buffer).unwrap();

                let loaded = Vec::load_from(std::io::Cursor::new(buffer)).unwrap();

                assert_eq!(vec, loaded);
            }
            // u16
            {
                let vec: Vec<u16> = (0..len).map(|_| { rng.gen() }).collect();
                let mut buffer = Vec::new();

                vec.save_to(&mut buffer).unwrap();

                let loaded = Vec::load_from(std::io::Cursor::new(buffer)).unwrap();

                assert_eq!(vec, loaded);
            }
            // u32
            {
                let vec: Vec<u32> = (0..len).map(|_| { rng.gen() }).collect();
                let mut buffer = Vec::new();

                vec.save_to(&mut buffer).unwrap();

                let loaded = Vec::load_from(std::io::Cursor::new(buffer)).unwrap();

                assert_eq!(vec, loaded);
            }
            // u64
            {
                let vec: Vec<u64> = (0..len).map(|_| { rng.gen() }).collect();
                let mut buffer = Vec::new();

                vec.save_to(&mut buffer).unwrap();

                let loaded = Vec::load_from(std::io::Cursor::new(buffer)).unwrap();

                assert_eq!(vec, loaded);
            }
            // u128
            {
                let vec: Vec<u128> = (0..len).map(|_| { rng.gen() }).collect();
                let mut buffer = Vec::new();

                vec.save_to(&mut buffer).unwrap();

                let loaded = Vec::load_from(std::io::Cursor::new(buffer)).unwrap();

                assert_eq!(vec, loaded);
            }
            // usize
            {
                let vec: Vec<usize> = (0..len).map(|_| { rng.gen() }).collect();
                let mut buffer = Vec::new();

                vec.save_to(&mut buffer).unwrap();

                let loaded = Vec::load_from(std::io::Cursor::new(buffer)).unwrap();

                assert_eq!(vec, loaded);
            }

            // i8
            {
                let vec: Vec<i8> = (0..len).map(|_| { rng.gen() }).collect();
                let mut buffer = Vec::new();

                vec.save_to(&mut buffer).unwrap();

                let loaded = Vec::load_from(std::io::Cursor::new(buffer)).unwrap();

                assert_eq!(vec, loaded);
            }
            // i16
            {
                let vec: Vec<i16> = (0..len).map(|_| { rng.gen() }).collect();
                let mut buffer = Vec::new();

                vec.save_to(&mut buffer).unwrap();

                let loaded = Vec::load_from(std::io::Cursor::new(buffer)).unwrap();

                assert_eq!(vec, loaded);
            }
            // i32
            {
                let vec: Vec<i32> = (0..len).map(|_| { rng.gen() }).collect();
                let mut buffer = Vec::new();

                vec.save_to(&mut buffer).unwrap();

                let loaded = Vec::load_from(std::io::Cursor::new(buffer)).unwrap();

                assert_eq!(vec, loaded);
            }
            // i64
            {
                let vec: Vec<i64> = (0..len).map(|_| { rng.gen() }).collect();
                let mut buffer = Vec::new();

                vec.save_to(&mut buffer).unwrap();

                let loaded = Vec::load_from(std::io::Cursor::new(buffer)).unwrap();

                assert_eq!(vec, loaded);
            }
            // i128
            {
                let vec: Vec<i128> = (0..len).map(|_| { rng.gen() }).collect();
                let mut buffer = Vec::new();

                vec.save_to(&mut buffer).unwrap();

                let loaded = Vec::load_from(std::io::Cursor::new(buffer)).unwrap();

                assert_eq!(vec, loaded);
            }
            // isize
            {
                let vec: Vec<isize> = (0..len).map(|_| { rng.gen() }).collect();
                let mut buffer = Vec::new();

                vec.save_to(&mut buffer).unwrap();

                let loaded = Vec::load_from(std::io::Cursor::new(buffer)).unwrap();

                assert_eq!(vec, loaded);
            }
        }
    }
}

#[test]
fn is_accurate_to_be_saved_size() {
    for n in 1..30 {
        {
            let mut buffer: Vec<u8> = Vec::new();
            let u8_vec = vec![0_u8; n];
            u8_vec.save_to(&mut buffer).unwrap();
            assert_eq!(u8_vec.to_be_saved_size(), buffer.len());
        }
        {
            let mut buffer: Vec<u8> = Vec::new();
            let u16_vec = vec![0_u16; n];
            u16_vec.save_to(&mut buffer).unwrap();
            assert_eq!(u16_vec.to_be_saved_size(), buffer.len());
        }
        {
            let mut buffer: Vec<u8> = Vec::new();
            let u32_vec = vec![0_u32; n];
            u32_vec.save_to(&mut buffer).unwrap();
            assert_eq!(u32_vec.to_be_saved_size(), buffer.len());
        }
        {
            let mut buffer: Vec<u8> = Vec::new();
            let u64_vec = vec![0_u64; n];
            u64_vec.save_to(&mut buffer).unwrap();
            assert_eq!(u64_vec.to_be_saved_size(), buffer.len());
        }
        {
            let mut buffer: Vec<u8> = Vec::new();
            let usize_vec = vec![0_usize; n];
            usize_vec.save_to(&mut buffer).unwrap();
            assert_eq!(usize_vec.to_be_saved_size(), buffer.len());
        }
        {
            let mut buffer: Vec<u8> = Vec::new();
            let i16_vec = vec![0_i16; n];
            i16_vec.save_to(&mut buffer).unwrap();
            assert_eq!(i16_vec.to_be_saved_size(), buffer.len());
        }
        {
            let mut buffer: Vec<u8> = Vec::new();
            let i32_vec = vec![0_i32; n];
            i32_vec.save_to(&mut buffer).unwrap();
            assert_eq!(i32_vec.to_be_saved_size(), buffer.len());
        }
        {
            let mut buffer: Vec<u8> = Vec::new();
            let i64_vec = vec![0_i64; n];
            i64_vec.save_to(&mut buffer).unwrap();
            assert_eq!(i64_vec.to_be_saved_size(), buffer.len());
        }
        {
            let mut buffer: Vec<u8> = Vec::new();
            let isize_vec = vec![0_isize; n];
            isize_vec.save_to(&mut buffer).unwrap();
            assert_eq!(isize_vec.to_be_saved_size(), buffer.len());
        }
    }
}
