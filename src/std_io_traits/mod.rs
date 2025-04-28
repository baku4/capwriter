mod element; // Save + Load
mod slice; // Save
mod vector; // Save + Load
mod array; // Save + Load
#[cfg(test)]
mod tests;

use std::io::{Write, Read, Error};
pub trait Save {
    fn save_as_le<W>(&self, writer: &mut W) -> Result<(), Error> where
        W: Write;
    fn save_as_be<W>(&self, writer: &mut W) -> Result<(), Error> where
        W: Write;
    fn to_be_saved_size(&self) -> usize;
}
pub trait Load {
    fn load_as_le<R>(reader: &mut R) -> Result<Self, Error> where
        R: Read,
        Self: Sized;
    fn load_as_be<R>(reader: &mut R) -> Result<Self, Error> where
        R: Read,
        Self: Sized;
}
