use super::{AsyncLoad, AsyncSave};
use std::future::Future;
use std::io::Error;
use tokio::io::{AsyncWrite, AsyncRead, AsyncReadExt};
use bytemuck::{
    Pod,
    cast_slice_mut as as_u8_mut_buf,
};

impl<const SIZE: usize, T: Pod + Sync> AsyncSave for [T; SIZE] {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move { (&self[..]).save_as_le(writer).await }
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move { (&self[..]).save_as_be(writer).await }
    }
}

impl<const SIZE: usize, T: Pod + Send> AsyncLoad for [T; SIZE] {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let size = u64::load_as_le(reader).await? as usize;

            if size != SIZE {
                return Err(Error::new(std::io::ErrorKind::InvalidData, "Size mismatch"));
            }
            let mut buffer = [T::zeroed(); SIZE];
            let casted_buffer: &mut [u8] = as_u8_mut_buf(&mut buffer);
            reader.read_exact(casted_buffer).await?;

            Ok(buffer)
        }
    }
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let size = u64::load_as_be(reader).await? as usize;

            if size != SIZE {
                return Err(Error::new(std::io::ErrorKind::InvalidData, "Size mismatch"));
            }
            let mut buffer = [T::zeroed(); SIZE];
            let casted_buffer: &mut [u8] = as_u8_mut_buf(&mut buffer);
            reader.read_exact(casted_buffer).await?;
    
            Ok(buffer)
        }
    }
}