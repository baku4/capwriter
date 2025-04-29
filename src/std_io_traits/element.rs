use super::{Save, Load};
use std::io::{Read, Write, Error};

// u8
impl Save for u8 {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&[*self])?;
        Ok(())
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&[*self])?;
        Ok(())
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&[*self])?;
        Ok(())
    }
    fn encoded_len(&self) -> usize {
        1
    }
}
impl Load for u8 {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }
    fn load_as_le<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}
// i8
impl Save for i8 {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&[*self as u8])?;
        Ok(())
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&[*self as u8])?;
        Ok(())
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&[*self as u8])?;
        Ok(())
    }
    fn encoded_len(&self) -> usize {
        1
    }
}
impl Load for i8 {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }
    fn load_as_le<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }
}

// u16
impl Save for u16 {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_ne_bytes())?;
        Ok(())
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
    fn encoded_len(&self) -> usize {
        2
    }
}
impl Load for u16 {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(u16::from_ne_bytes(buf))
    }
    fn load_as_le<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }
}
// i16
impl Save for i16 {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_ne_bytes())?;
        Ok(())
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
    fn encoded_len(&self) -> usize {
        2
    }
}
impl Load for i16 {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(i16::from_ne_bytes(buf))
    }
    fn load_as_le<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }
}
// u32
impl Save for u32 {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_ne_bytes())?;
        Ok(())
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
    fn encoded_len(&self) -> usize {
        4
    }
}
impl Load for u32 {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_ne_bytes(buf))
    }
    fn load_as_le<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }
}
// i32
impl Save for i32 {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_ne_bytes())?;
        Ok(())
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
    fn encoded_len(&self) -> usize {
        4
    }
}
impl Load for i32 {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(i32::from_ne_bytes(buf))
    }
    fn load_as_le<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }
}

// u64
impl Save for u64 {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_ne_bytes())?;
        Ok(())
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
    fn encoded_len(&self) -> usize {
        8
    }
}
impl Load for u64 {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(u64::from_ne_bytes(buf))
    }
    fn load_as_le<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }
}
// i64
impl Save for i64 {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_ne_bytes())?;
        Ok(())
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
    fn encoded_len(&self) -> usize {
        8
    }
}
impl Load for i64 {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(i64::from_ne_bytes(buf))
    }
    fn load_as_le<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }
}

// u128
impl Save for u128 {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_ne_bytes())?;
        Ok(())
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
    fn encoded_len(&self) -> usize {
        16
    }
}
impl Load for u128 {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 16];
        reader.read_exact(&mut buf)?;
        Ok(u128::from_ne_bytes(buf))
    }
    fn load_as_le<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 16];
        reader.read_exact(&mut buf)?;
        Ok(u128::from_le_bytes(buf))
    }
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 16];
        reader.read_exact(&mut buf)?;
        Ok(u128::from_be_bytes(buf))
    }
}
// i128
impl Save for i128 {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_ne_bytes())?;
        Ok(())
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
    fn encoded_len(&self) -> usize {
        16
    }
}
impl Load for i128 {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 16];
        reader.read_exact(&mut buf)?;
        Ok(i128::from_ne_bytes(buf))
    }
    fn load_as_le<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 16];
        reader.read_exact(&mut buf)?;
        Ok(i128::from_le_bytes(buf))
    }
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 16];
        reader.read_exact(&mut buf)?;
        Ok(i128::from_be_bytes(buf))
    }
}

// usize
impl Save for usize {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        #[cfg(target_pointer_width = "32")]
        {
            (*self as u32).save_as_ne(writer)
        }
        #[cfg(target_pointer_width = "64")]
        {
            (*self as u64).save_as_ne(writer)
        }
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        #[cfg(target_pointer_width = "32")]
        {
            (*self as u32).save_as_le(writer)
        }
        #[cfg(target_pointer_width = "64")]
        {
            (*self as u64).save_as_le(writer)
        }
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        #[cfg(target_pointer_width = "32")]
        {
            (*self as u32).save_as_be(writer)
        }
        #[cfg(target_pointer_width = "64")]
        {
            (*self as u64).save_as_be(writer)
        }
    }
    fn encoded_len(&self) -> usize {
        #[cfg(target_pointer_width = "32")]
        return 4;
        #[cfg(target_pointer_width = "64")]
        return 8;
    }
}
impl Load for usize {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> {
        #[cfg(target_pointer_width = "32")]
        {
            u32::load_as_ne(reader).map(|v| v as usize)
        }
        #[cfg(target_pointer_width = "64")]
        {
            u64::load_as_ne(reader).map(|v| v as usize)
        }
    }
    fn load_as_le<R: Read>(reader: &mut R) -> Result<Self, Error> {
        #[cfg(target_pointer_width = "32")]
        {
            u32::load_as_le(reader).map(|v| v as usize)
        }
        #[cfg(target_pointer_width = "64")]
        {
            u64::load_as_le(reader).map(|v| v as usize)
        }
    }
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> {
        #[cfg(target_pointer_width = "32")]
        {
            u32::load_as_be(reader).map(|v| v as usize)
        }
        #[cfg(target_pointer_width = "64")]
        {
            u64::load_as_be(reader).map(|v| v as usize)
        }
    }
}
// isize
impl Save for isize {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        #[cfg(target_pointer_width = "32")]
        {
            (*self as i32).save_as_ne(writer)
        }
        #[cfg(target_pointer_width = "64")]
        {
            (*self as i64).save_as_ne(writer)
        }
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        #[cfg(target_pointer_width = "32")]
        {
            (*self as i32).save_as_le(writer)
        }
        #[cfg(target_pointer_width = "64")]
        {
            (*self as i64).save_as_le(writer)
        }
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        #[cfg(target_pointer_width = "32")]
        {
            (*self as i32).save_as_be(writer)
        }
        #[cfg(target_pointer_width = "64")]
        {
            (*self as i64).save_as_be(writer)
        }
    }
    fn encoded_len(&self) -> usize {
        #[cfg(target_pointer_width = "32")]
        return 4;
        #[cfg(target_pointer_width = "64")]
        return 8;
    }
}
impl Load for isize {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> {
        #[cfg(target_pointer_width = "32")]
        {
            i32::load_as_ne(reader).map(|v| v as isize)
        }
        #[cfg(target_pointer_width = "64")]
        {
            i64::load_as_ne(reader).map(|v| v as isize)
        }
    }
    fn load_as_le<R: Read>(reader: &mut R) -> Result<Self, Error> {
        #[cfg(target_pointer_width = "32")]
        {
            i32::load_as_le(reader).map(|v| v as isize)
        }
        #[cfg(target_pointer_width = "64")]
        {
            i64::load_as_le(reader).map(|v| v as isize)
        }
    }
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> {
        #[cfg(target_pointer_width = "32")]
        {
            i32::load_as_be(reader).map(|v| v as isize)
        }
        #[cfg(target_pointer_width = "64")]
        {
            i64::load_as_be(reader).map(|v| v as isize)
        }
    }
}