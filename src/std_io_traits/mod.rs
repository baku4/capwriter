mod element;  // Save + Load
mod slice;    // Save
mod vector;   // Save + Load
mod array;    // Save + Load
#[cfg(test)]
mod tests;

use std::io::{Write, Read, Error};
pub trait Save {
    fn save_as_ne<W: Write>(&self, writer: &mut W) -> Result<(), Error>;
    fn save_as_le<W: Write>(&self, writer: &mut W) -> Result<(), Error>;
    fn save_as_be<W: Write>(&self, writer: &mut W) -> Result<(), Error>;
    fn encoded_len(&self) -> usize;
}
pub trait Load {
    fn load_as_ne<R: Read>(reader: &mut R) -> Result<Self, Error> where Self: Sized;
    fn load_as_le<R: Read>(reader: &mut R) -> Result<Self, Error> where Self: Sized;
    fn load_as_be<R: Read>(reader: &mut R) -> Result<Self, Error> where Self: Sized;
}
