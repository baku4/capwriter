use crate::{Save, Load, EndianType, ReadBytesExt};
use bytemuck::{
    Pod,
    cast_slice_mut as as_u8_mut_buf,
};
use std::io::{Read, Write, Error};

impl<const SIZE: usize, T: Pod> Save for [T; SIZE] {
    fn save_to<W>(&self, writer: &mut W) -> Result<(), Error> where
        W: Write
    {
        (&self[..]).save_to(writer)?;

        Ok(())
    }
    fn to_be_saved_size(&self) -> usize {
        (&self[..]).to_be_saved_size()
    }
}

impl<const SIZE: usize, T: Pod> Load for [T; SIZE] {
    fn load_from<R>(reader: &mut R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let size = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let size = reader.read_u64::<EndianType>()? as usize;

        if size != SIZE {
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Size mismatch"));
        }
        let mut buffer = [T::zeroed(); SIZE];
        let casted_buffer: &mut [u8] = as_u8_mut_buf(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}