#[cfg(target_endian = "little")]
type EndianType = byteorder::LittleEndian;
#[cfg(target_endian = "big")]
type EndianType = byteorder::BigEndian;
use byteorder::{ReadBytesExt, WriteBytesExt};

// Requirements
use std::io::{Write, Read, Error};
pub trait Save {
    fn save_to<W>(&self, writer: &mut W) -> Result<(), Error> where
        W: Write;
    fn to_be_saved_size(&self) -> usize;
}
pub trait Load {
    fn load_from<R>(reader: &mut R) -> Result<Self, Error> where
        R: Read,
        Self: Sized;
}

mod implementation;
#[cfg(test)]
mod tests;
