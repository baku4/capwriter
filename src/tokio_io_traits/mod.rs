// For async I/O (tokio)
use tokio::io::{AsyncWrite, AsyncRead};
use std::io::Error;
use std::pin::Pin;

mod element; // AsyncSave + AsyncLoad
mod slice; // AsyncSave
mod vector; // AsyncSave + AsyncLoad
mod array; // AsyncSave + AsyncLoad
#[cfg(test)]
mod tests;

pub trait AsyncSave {
    async fn save_as_le<'a, W>(&'a self, writer: Pin<&'a mut W>) -> Result<(), Error> where
        W: AsyncWrite;
    async fn save_as_be<'a, W>(&'a self, writer: Pin<&'a mut W>) -> Result<(), Error> where
        W: AsyncWrite;
}
pub trait AsyncLoad {
    async fn load_as_le<R>(reader: Pin<&mut R>) -> Result<Self, Error> where
        R: AsyncRead,
        Self: Sized;
    async fn load_as_be<R>(reader: Pin<&mut R>) -> Result<Self, Error> where
        R: AsyncRead,
        Self: Sized;
}
