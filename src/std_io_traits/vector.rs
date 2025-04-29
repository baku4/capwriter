use std::io::{Read, Write, Error};
use bytemuck::{
    Pod,
    cast_slice_mut as as_u8_mut_buf,
};
use super::{Save, Load};

impl<T: Save + Pod> Save for Vec<T> {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        (self as &[T]).save_as_ne(writer)
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        (self as &[T]).save_as_le(writer)
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        (self as &[T]).save_as_be(writer)
    }
    fn encoded_len(&self) -> usize {
        (self as &[T]).encoded_len()
    }
}
impl<T: Load + Pod> Load for Vec<T> {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let len = u64::load_as_ne(reader)? as usize;

        let mut buffer = vec![T::zeroed(); len];
        let casted_buffer: &mut [u8] = as_u8_mut_buf(&mut buffer);
        reader.read_exact(casted_buffer)?;
        Ok(buffer)
    }
    fn load_as_le<R: Read>(reader: &mut R) -> Result<Self, Error> {
        #[cfg(target_endian = "little")]
        {
            Self::load_as_ne(reader)
        }
        #[cfg(target_endian = "big")]
        {
            let len = u64::load_as_le(reader)? as usize;

            let mut buffer = vec![T::zeroed(); len];
            for i in 0..len {
                buffer[i] = T::load_as_le(reader)?;
            }

            Ok(buffer)
        }
    }
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> {
        #[cfg(target_endian = "little")]
        {
            let len = u64::load_as_be(reader)? as usize;

            let mut buffer = vec![T::zeroed(); len];
            for i in 0..len {
                buffer[i] = T::load_as_be(reader)?;
            }

            Ok(buffer)
        }
        #[cfg(target_endian = "big")]
        {
            Self::load_as_ne(reader)
        }
    }
}
