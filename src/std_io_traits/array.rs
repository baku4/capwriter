use super::{Save, Load};
use bytemuck::{
    Pod,
    cast_slice_mut as as_u8_mut_buf,
};
use std::io::{Read, Write, Error};

impl<const SIZE: usize, T: Save + Pod> Save for [T; SIZE] {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        (&self[..]).save_as_ne(writer)
    }
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        (&self[..]).save_as_le(writer)
    }
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        (&self[..]).save_as_be(writer)
    }
    fn encoded_len(&self) -> usize {
        (&self[..]).encoded_len()
    }
}

impl<const SIZE: usize, T: Load + Pod> Load for [T; SIZE] {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let size = u64::load_as_ne(reader)? as usize;

        if size != SIZE {
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Size mismatch"));
        }
        let mut buffer = [T::zeroed(); SIZE];
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
            let size = u64::load_as_le(reader)? as usize;

            if size != SIZE {
                return Err(Error::new(std::io::ErrorKind::InvalidData, "Size mismatch"));
            }
            let mut buffer = [T::zeroed(); SIZE];
            for i in 0..size {
                buffer[i] = T::load_as_le(reader)?;
            }

            Ok(buffer)
        }
    }
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> {
        #[cfg(target_endian = "little")]
        {
            let size = u64::load_as_be(reader)? as usize;

            if size != SIZE {
                return Err(Error::new(std::io::ErrorKind::InvalidData, "Size mismatch"));
            }
            let mut buffer = [T::zeroed(); SIZE];
            for i in 0..size {
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