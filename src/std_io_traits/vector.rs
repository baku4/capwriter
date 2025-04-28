use super::{Save, Load};
use bytemuck::{
    Pod,
    cast_slice_mut as as_u8_mut_buf,
};
use std::io::{Read, Write, Error};

impl<T: Pod> Save for Vec<T> {
    fn save_as_le<W>(&self, writer: &mut W) -> Result<(), Error> where
        W: Write
    {
        (self as &[T]).save_as_le(writer)
    }
    fn save_as_be<W>(&self, writer: &mut W) -> Result<(), Error> where
        W: Write
    {
        (self as &[T]).save_as_be(writer)
    }
    fn to_be_saved_size(&self) -> usize {
        (self as &[T]).to_be_saved_size()
    }
}
impl<T: Pod> Load for Vec<T> {
    fn load_as_le<R>(reader: &mut R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        let mut len_buf = [0; 8];
        reader.read_exact(&mut len_buf)?;

        let len = u64::from_le_bytes(len_buf) as usize;

        let mut buffer = vec![T::zeroed(); len];
        let casted_buffer: &mut [u8] = as_u8_mut_buf(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
    fn load_as_be<R>(reader: &mut R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        let mut len_buf = [0; 8];
        reader.read_exact(&mut len_buf)?;

        let len = u64::from_be_bytes(len_buf) as usize;

        let mut buffer = vec![T::zeroed(); len];
        let casted_buffer: &mut [u8] = as_u8_mut_buf(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}
