use super::Save;
use bytemuck::{
    Pod,
    cast_slice as as_u8_buf,
};
use std::io::{Write, Error};

const SIZE_OF_LENGTH: usize = 8;

impl<T: Save + Pod> Save for [T] {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        let length = self.len() as u64;
        writer.write_all(&length.to_ne_bytes())?;
        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        #[cfg(target_endian = "little")]
        {
            self.save_as_ne(writer)
        }
        #[cfg(target_endian = "big")]
        {
            self.len().save_as_le(writer)?;
            for x in self.iter() {
                x.save_as_le(writer)?;
            }
            Ok(())
        }
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        #[cfg(target_endian = "little")]
        {
            self.len().save_as_be(writer)?;
            for x in self.iter() {
                x.save_as_be(writer)?;
            }
            Ok(())
        }
        #[cfg(target_endian = "big")]
        {
            self.save_as_ne(writer)
        }
    }
    fn encoded_len(&self) -> usize {
        let buf: &[u8] = as_u8_buf(self);
        buf.len() + SIZE_OF_LENGTH
    }
}
