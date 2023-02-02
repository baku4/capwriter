#[allow(unused_imports)]
use super::{
    EndianType, WriteBytesExt, ReadBytesExt,
    Saveable, Loadable,
};
use std::io::{Read, Write, Error};

// u8
impl Saveable for Vec<u8> {
    fn save_to<W>(&self, writer: W) -> Result<(), Error> where
        W: Write,
    {
        (self as &[u8]).save_to(writer)
    }
    fn size_of(&self) -> usize {
        (self as &[u8]).size_of()
    }
}
impl Loadable for Vec<u8> {
    fn load_from<R>(mut reader: R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        reader.read_exact(&mut buffer)?;

        Ok(buffer)
    }
}


// u16
impl Saveable for Vec<u16> {
    fn save_to<W>(&self, writer: W) -> Result<(), Error> where
        W: Write
    {
        (self as &[u16]).save_to(writer)
    }
    fn size_of(&self) -> usize {
        (self as &[u16]).size_of()
    }
}
impl Loadable for Vec<u16> {
    fn load_from<R>(mut reader: R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}


// u32
impl Saveable for Vec<u32> {
    fn save_to<W>(&self, writer: W) -> Result<(), Error> where
        W: Write
    {
        (self as &[u32]).save_to(writer)
    }
    fn size_of(&self) -> usize {
        (self as &[u32]).size_of()
    }
}
impl Loadable for Vec<u32> {
    fn load_from<R>(mut reader: R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}


// u64
impl Saveable for Vec<u64> {
    fn save_to<W>(&self, writer: W) -> Result<(), Error> where
        W: Write
    {
        (self as &[u64]).save_to(writer)
    }
    fn size_of(&self) -> usize {
        (self as &[u64]).size_of()
    }
}
impl Loadable for Vec<u64> {
    fn load_from<R>(mut reader: R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}


// u128
impl Saveable for Vec<u128> {
    fn save_to<W>(&self, writer: W) -> Result<(), Error> where
        W: Write
    {
        (self as &[u128]).save_to(writer)
    }
    fn size_of(&self) -> usize {
        (self as &[u128]).size_of()
    }
}
impl Loadable for Vec<u128> {
    fn load_from<R>(mut reader: R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}


// usize
impl Saveable for Vec<usize> {
    fn save_to<W>(&self, writer: W) -> Result<(), Error> where
        W: Write
    {
        (self as &[usize]).save_to(writer)
    }
    fn size_of(&self) -> usize {
        (self as &[usize]).size_of()
    }
}
impl Loadable for Vec<usize> {
    fn load_from<R>(mut reader: R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}


// i8
impl Saveable for Vec<i8> {
    fn save_to<W>(&self, writer: W) -> Result<(), Error> where
        W: Write
    {
        (self as &[i8]).save_to(writer)
    }
    fn size_of(&self) -> usize {
        (self as &[i8]).size_of()
    }
}
impl Loadable for Vec<i8> {
    fn load_from<R>(mut reader: R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}


// i16
impl Saveable for Vec<i16> {
    fn save_to<W>(&self, writer: W) -> Result<(), Error> where
        W: Write
    {
        (self as &[i16]).save_to(writer)
    }
    fn size_of(&self) -> usize {
        (self as &[i16]).size_of()
    }
}
impl Loadable for Vec<i16> {
    fn load_from<R>(mut reader: R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}


// i32
impl Saveable for Vec<i32> {
    fn save_to<W>(&self, writer: W) -> Result<(), Error> where
        W: Write
    {
        (self as &[i32]).save_to(writer)
    }
    fn size_of(&self) -> usize {
        (self as &[i32]).size_of()
    }
}
impl Loadable for Vec<i32> {
    fn load_from<R>(mut reader: R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}


// i64
impl Saveable for Vec<i64> {
    fn save_to<W>(&self, writer: W) -> Result<(), Error> where
        W: Write
    {
        (self as &[i64]).save_to(writer)
    }
    fn size_of(&self) -> usize {
        (self as &[i64]).size_of()
    }
}
impl Loadable for Vec<i64> {
    fn load_from<R>(mut reader: R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}


// i128
impl Saveable for Vec<i128> {
    fn save_to<W>(&self, writer: W) -> Result<(), Error> where
        W: Write
    {
        (self as &[i128]).save_to(writer)
    }
    fn size_of(&self) -> usize {
        (self as &[i128]).size_of()
    }
}
impl Loadable for Vec<i128> {
    fn load_from<R>(mut reader: R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}


// isize
impl Saveable for Vec<isize> {
    fn save_to<W>(&self, writer: W) -> Result<(), Error> where
        W: Write
    {
        (self as &[isize]).save_to(writer)
    }
    fn size_of(&self) -> usize {
        (self as &[isize]).size_of()
    }
}
impl Loadable for Vec<isize> {
    fn load_from<R>(mut reader: R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::{Saveable, Loadable};

    #[test]
    fn test_all_vector() {
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
}