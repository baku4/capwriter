use super::{Save, Load};
use bytemuck::{
    Pod,
    cast_slice_mut as as_u8_mut_buf,
};
use std::io::{Read, Write, Error};

impl<const SIZE: usize, T: Pod> Save for [T; SIZE] {
    fn save_as_le<W>(&self, writer: &mut W) -> Result<(), Error> where
        W: Write
    {
        (&self[..]).save_as_le(writer)?;

        Ok(())
    }
    fn save_as_be<W>(&self, writer: &mut W) -> Result<(), Error> where
        W: Write
    {
        (&self[..]).save_as_be(writer)?;

        Ok(())   
    }
    fn to_be_saved_size(&self) -> usize {
        (&self[..]).to_be_saved_size()
    }
}

impl<const SIZE: usize, T: Pod> Load for [T; SIZE] {
    fn load_as_le<R>(reader: &mut R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        
        let size = u64::load_as_le(reader)? as usize;

        if size != SIZE {
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Size mismatch"));
        }
        let mut buffer = [T::zeroed(); SIZE];
        let casted_buffer: &mut [u8] = as_u8_mut_buf(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
    fn load_as_be<R>(reader: &mut R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        let size = u64::load_as_be(reader)? as usize;

        if size != SIZE {
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Size mismatch"));
        }
        let mut buffer = [T::zeroed(); SIZE];
        let casted_buffer: &mut [u8] = as_u8_mut_buf(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}