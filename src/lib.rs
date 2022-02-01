// Error propagation
use anyhow::{Result, bail as error_msg};

// Endian type
#[cfg(target_endian = "little")]
type EndianType = byteorder::LittleEndian;
#[cfg(target_endian = "big")]
type EndianType = byteorder::BigEndian;

// Read & Write extension
use byteorder::{ReadBytesExt, WriteBytesExt};

// Main trait
use std::io::{Write, Read};

pub trait Saveable {
    fn save_to<W>(&self, writer: W) -> Result<()> where
        W: Write;
}

pub trait Loadable {
    fn load_from<R>(reader: R) -> Result<Self> where
        R: Read,
        Self: Sized;
}

// Implementations
mod implementation;
