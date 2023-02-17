use crate::{Save, EndianType, WriteBytesExt};
use bytemuck::{
    Pod,
    cast_slice as as_u8_buf,
};
use std::io::{Write, Error};

#[cfg(target_pointer_width = "32")]
const SIZE_OF_LENGTH: usize = 4;
#[cfg(target_pointer_width = "64")]
const SIZE_OF_LENGTH: usize = 8;

impl<T: Pod> Save for &[T] {
    fn save_to<W>(&self, mut writer: W) -> Result<(), Error> where
        W: Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        writer.write_all(as_u8_buf(self))?;

        Ok(())
    }
    fn to_be_saved_size(&self) -> usize {
        let buf: &[u8] = as_u8_buf(self);
        buf.len() + SIZE_OF_LENGTH
    }
}
