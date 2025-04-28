// For async I/O (tokio)
use tokio::io::{AsyncWrite, AsyncRead};
use std::future::Future;
use std::io::Error;

mod element; // AsyncSave + AsyncLoad
mod slice; // AsyncSave
mod vector; // AsyncSave + AsyncLoad
mod array; // AsyncSave + AsyncLoad
#[cfg(test)]
mod tests;

pub trait AsyncSave {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send;
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send;
}
pub trait AsyncLoad {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized;
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized;
}
