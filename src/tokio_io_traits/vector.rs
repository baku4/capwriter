use super::{AsyncLoad, AsyncSave};
use std::future::Future;
use std::io::Error;
use tokio::io::{AsyncWrite, AsyncRead, AsyncReadExt};
use bytemuck::Pod;

impl<T: Pod+ Sync> AsyncSave for Vec<T> {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move { (self as &[T]).save_as_le(writer).await }
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move { (self as &[T]).save_as_be(writer).await }
    }
}
impl<T: Pod + Send> AsyncLoad for Vec<T> {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut len_buf = [0; 8];
            reader.read_exact(&mut len_buf).await?;

            let len = u64::from_le_bytes(len_buf) as usize;
            let mut buffer = vec![T::zeroed(); len];
            let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer);
            reader.read_exact(casted_buffer).await?;
            Ok(buffer)
        }
    }
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut len_buf = [0; 8];
            reader.read_exact(&mut len_buf).await?;

            let len = u64::from_be_bytes(len_buf) as usize;
            let mut buffer = vec![T::zeroed(); len];
            let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer);
            reader.read_exact(casted_buffer).await?;
            Ok(buffer)
        }
    }
}