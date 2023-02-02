#[test]
fn test_on_readme() {
    use crate::{Saveable, Loadable};

    //  - Supported type:
    //    Vec<T>, &[T]: T of {
    //        u8, u16, u32, u64, u128, usize,
    //        i8, i16, i32, i64, i128, isize
    //    }
    
    let vec_to_save: Vec<i32> = vec![1, 2, 3, 4, 5];
    
    // (1) Save
    let mut buffer = Vec::new();
    vec_to_save.save_to(&mut buffer).unwrap();
    // (2) Load
    let vec_loaded = Vec::<i32>::load_from(
        std::io::Cursor::new(buffer)
    ).unwrap();
    
    assert_eq!(vec_to_save, vec_loaded);
}
