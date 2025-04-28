use super::Save;
use bytemuck::{
    Pod,
    cast_slice as as_u8_buf,
};
use std::io::{Write, Error};

const SIZE_OF_LENGTH: usize = 8;

impl<T: Pod> Save for &[T] {
    fn save_as_le<W>(&self, writer: &mut W) -> Result<(), Error> where
        W: Write
    {
        (self.len() as u64).save_as_le(writer)?;
        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn save_as_be<W>(&self, writer: &mut W) -> Result<(), Error> where
        W: Write
    {
        (self.len() as u64).save_as_be(writer)?;
        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn to_be_saved_size(&self) -> usize {
        let buf: &[u8] = as_u8_buf(self);
        buf.len() + SIZE_OF_LENGTH
    }
}
