use super::{
    Result, error_msg,
    EndianType, WriteBytesExt, ReadBytesExt,
    Saveable, Loadable,
};

// Savable
mod slice;
// Savable + Loadable
mod vector;
