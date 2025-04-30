// For async I/O (tokio)
use tokio::io::{AsyncWrite, AsyncRead};
use std::io::Error;
use std::pin::Pin;
use std::future::Future;

mod element; // AsyncSave + AsyncLoad
mod slice;   // AsyncSave
mod vector;  // AsyncSave + AsyncLoad
mod array;   // AsyncSave + AsyncLoad
#[cfg(test)]
mod tests;

pub trait AsyncSave {
    fn save_as_ne<'a, W >(&'a self, writer: Pin<&'a mut W>) -> impl Future<Output = Result<(), Error>> + Send where 
        W: AsyncWrite + Send;
    fn save_as_le<'a, W>(&'a self, writer: Pin<&'a mut W>) -> impl Future<Output = Result<(), Error>> + Send where 
        W: AsyncWrite + Send;
    fn save_as_be<'a, W>(&'a self, writer: Pin<&'a mut W>) -> impl Future<Output = Result<(), Error>> + Send where 
        W: AsyncWrite + Send;
}
pub trait AsyncLoad {
    fn load_as_ne<R>(reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send, Self: Sized;
    fn load_as_le<R>(reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send, Self: Sized;
    fn load_as_be<R>(reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send, Self: Sized;
}
