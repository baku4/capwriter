#[allow(unused_imports)]
use super::{
    Result, error_msg,
    EndianType, WriteBytesExt, ReadBytesExt,
    Saveable, Loadable,
};

use bytemuck::cast_slice as as_u8_buf;


#[cfg(target_pointer_width = "32")]
const SIZE_OF_LENGTH: usize = 4;
#[cfg(target_pointer_width = "64")]
const SIZE_OF_LENGTH: usize = 8;

// u8
impl Saveable for &[u8] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        writer.write_all(self)?;

        Ok(())
    }
    fn size_of(&self) -> usize {
        self.len() + SIZE_OF_LENGTH
    }
}

// u16
impl Saveable for &[u16] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn size_of(&self) -> usize {
        2 * self.len() + SIZE_OF_LENGTH
    }
}


// u32
impl Saveable for &[u32] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn size_of(&self) -> usize {
        4 * self.len() + SIZE_OF_LENGTH
    }
}


// u64
impl Saveable for &[u64] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn size_of(&self) -> usize {
        8 * self.len() + SIZE_OF_LENGTH
    }
}


// u128
impl Saveable for &[u128] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn size_of(&self) -> usize {
        8 * self.len() + SIZE_OF_LENGTH
    }
}


// usize
impl Saveable for &[usize] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn size_of(&self) -> usize {
        SIZE_OF_LENGTH * self.len() + SIZE_OF_LENGTH
    }
}


// i8
impl Saveable for &[i8] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn size_of(&self) -> usize {
        self.len() + SIZE_OF_LENGTH
    }
}


// i16
impl Saveable for &[i16] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn size_of(&self) -> usize {
        2 * self.len() + SIZE_OF_LENGTH
    }
}


// i32
impl Saveable for &[i32] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn size_of(&self) -> usize {
        4 * self.len() + SIZE_OF_LENGTH
    }
}


// i64
impl Saveable for &[i64] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn size_of(&self) -> usize {
        8 * self.len() + SIZE_OF_LENGTH
    }
}


// i128
impl Saveable for &[i128] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn size_of(&self) -> usize {
        8 * self.len() + SIZE_OF_LENGTH
    }
}


// isize
impl Saveable for &[isize] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn size_of(&self) -> usize {
        SIZE_OF_LENGTH * self.len() + SIZE_OF_LENGTH
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        for n in 1..30 {
            {
                let mut buffer: Vec<u8> = Vec::new();
                let u8_vec = vec![0_u8; n];
                u8_vec.save_to(&mut buffer).unwrap();
                assert_eq!(u8_vec.size_of(), buffer.len());
            }
            {
                let mut buffer: Vec<u8> = Vec::new();
                let u16_vec = vec![0_u16; n];
                u16_vec.save_to(&mut buffer).unwrap();
                assert_eq!(u16_vec.size_of(), buffer.len());
            }
            {
                let mut buffer: Vec<u8> = Vec::new();
                let u32_vec = vec![0_u32; n];
                u32_vec.save_to(&mut buffer).unwrap();
                assert_eq!(u32_vec.size_of(), buffer.len());
            }
            {
                let mut buffer: Vec<u8> = Vec::new();
                let u64_vec = vec![0_u64; n];
                u64_vec.save_to(&mut buffer).unwrap();
                assert_eq!(u64_vec.size_of(), buffer.len());
            }
            {
                let mut buffer: Vec<u8> = Vec::new();
                let usize_vec = vec![0_usize; n];
                usize_vec.save_to(&mut buffer).unwrap();
                assert_eq!(usize_vec.size_of(), buffer.len());
            }
            {
                let mut buffer: Vec<u8> = Vec::new();
                let i16_vec = vec![0_i16; n];
                i16_vec.save_to(&mut buffer).unwrap();
                assert_eq!(i16_vec.size_of(), buffer.len());
            }
            {
                let mut buffer: Vec<u8> = Vec::new();
                let i32_vec = vec![0_i32; n];
                i32_vec.save_to(&mut buffer).unwrap();
                assert_eq!(i32_vec.size_of(), buffer.len());
            }
            {
                let mut buffer: Vec<u8> = Vec::new();
                let i64_vec = vec![0_i64; n];
                i64_vec.save_to(&mut buffer).unwrap();
                assert_eq!(i64_vec.size_of(), buffer.len());
            }
            {
                let mut buffer: Vec<u8> = Vec::new();
                let isize_vec = vec![0_isize; n];
                isize_vec.save_to(&mut buffer).unwrap();
                assert_eq!(isize_vec.size_of(), buffer.len());
            }
        }
    }
}