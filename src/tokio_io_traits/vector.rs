use super::{AsyncLoad, AsyncSave};
use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use tokio::io::{AsyncWrite, AsyncRead, AsyncReadExt};
use bytemuck::Pod;

impl<T: AsyncSave + Pod + Sync> AsyncSave for Vec<T> {
    fn save_as_ne<'a, W>(&'a self, writer: Pin<&'a mut W>) -> impl Future<Output = Result<(), Error>> + Send where 
        W: AsyncWrite + Send
    {
        async move {
            let slice: &[T] = self;
            slice.save_as_ne(writer).await
        }
    }

    fn save_as_le<'a, W>(&'a self, writer: Pin<&'a mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send
    {
        async move {
            let slice: &[T] = self;
            slice.save_as_le(writer).await
        }
    }
    fn save_as_be<'a, W>(&'a self, writer: Pin<&'a mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send
    {
        async move {
            let slice: &[T] = self;
            slice.save_as_be(writer).await
        }
    }
}
impl<T: AsyncLoad + Pod + Send> AsyncLoad for Vec<T> {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send, Self: Sized
    {
        async move {
            let len = u64::load_as_ne(reader.as_mut()).await? as usize;

            let mut buffer = vec![T::zeroed(); len];
            let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer);
            reader.as_mut().read_exact(casted_buffer).await?;
            Ok(buffer)
        }
    }
    fn load_as_le<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send, Self: Sized
    {
        async move {
            #[cfg(target_endian = "little")]
            {
                Self::load_as_ne(reader).await
            }
            #[cfg(target_endian = "big")]
            {
                let len = u64::load_as_le(reader.as_mut()).await? as usize;

                let mut buffer = vec![T::zeroed(); len];
                for i in 0..len {
                    buffer[i] = T::load_as_le(reader.as_mut()).await?;
                }
                Ok(buffer)
            }
        }
    }
    fn load_as_be<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send, Self: Sized
    {
        async move {
            #[cfg(target_endian = "little")]
            {
                let len = u64::load_as_be(reader.as_mut()).await? as usize;

                let mut buffer = vec![T::zeroed(); len];
                for i in 0..len {
                    buffer[i] = T::load_as_be(reader.as_mut()).await?;
                }
                Ok(buffer)
            }
            #[cfg(target_endian = "big")]
            {
                Self::load_as_ne(reader).await
            }
        }
    }
}