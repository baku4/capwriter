use crate::{Save, Load, EndianType, ReadBytesExt};
use bytemuck::{
    Pod,
    cast_slice_mut as as_u8_mut_buf,
};
use std::io::{Read, Write, Error};

impl<T: Pod> Save for Vec<T> {
    fn save_to<W>(&self, writer: W) -> Result<(), Error> where
        W: Write,
    {
        (self as &[T]).save_to(writer)
    }
    fn to_be_saved_size(&self) -> usize {
        (self as &[T]).to_be_saved_size()
    }
}
impl<T: Pod> Load for Vec<T> {
    fn load_from<R>(mut reader: R) -> Result<Self, Error> where
        R: Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![T::zeroed(); len];
        let casted_buffer: &mut [u8] = as_u8_mut_buf(&mut buffer);
        reader.read_exact(casted_buffer)?;

        Ok(buffer)
    }
}
