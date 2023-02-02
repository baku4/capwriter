#[cfg(target_endian = "little")]
type EndianType = byteorder::LittleEndian;
#[cfg(target_endian = "big")]
type EndianType = byteorder::BigEndian;
use byteorder::{ReadBytesExt, WriteBytesExt};

// Requirements
use std::io::{Write, Read, Error};
pub trait Saveable {
    fn save_to<W>(&self, writer: W) -> Result<(), Error> where
        W: Write;
    fn size_of(&self) -> usize;
}
pub trait Loadable {
    fn load_from<R>(reader: R) -> Result<Self, Error> where
        R: Read,
        Self: Sized;
}

mod implementation;
#[cfg(test)]
mod tests;
