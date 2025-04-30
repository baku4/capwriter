use super::{AsyncLoad, AsyncSave};
use std::future::Future;
use std::pin::Pin;
use std::io::Error;
use tokio::io::{AsyncWrite, AsyncRead, AsyncReadExt};
use bytemuck::{
    Pod,
    cast_slice_mut as as_u8_mut_buf,
};

impl<const SIZE: usize, T: AsyncSave + Pod + Sync> AsyncSave for [T; SIZE] {
    fn save_as_ne<'a, W>(&'a self, writer: Pin<&'a mut W>) -> impl Future<Output = Result<(), Error>> + Send where 
        W: AsyncWrite + Send
    {
        async move { (&self[..]).save_as_ne(writer).await }
    }

    fn save_as_le<'a, W>(&'a self, writer: Pin<&'a mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send
    {
        async move { (&self[..]).save_as_le(writer).await }
    }
    fn save_as_be<'a, W>(&'a self, writer: Pin<&'a mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send
    {
        async move { (&self[..]).save_as_be(writer).await }
    }
}

impl<const SIZE: usize, T: AsyncLoad + Pod + Send> AsyncLoad for [T; SIZE] {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send, Self: Sized
    {
        async move {
            let size = u64::load_as_ne(reader.as_mut()).await? as usize;

            if size != SIZE {
                return Err(Error::new(std::io::ErrorKind::InvalidData, "Size mismatch"));
            }

            let mut buffer = [T::zeroed(); SIZE];
            let casted_buffer: &mut [u8] = as_u8_mut_buf(&mut buffer);
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
                let size = u64::load_as_le(reader.as_mut()).await? as usize;

                if size != SIZE {
                    return Err(Error::new(std::io::ErrorKind::InvalidData, "Size mismatch"));
                }

                let mut buffer = [T::zeroed(); SIZE];
                for i in 0..size {
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
                let size = u64::load_as_be(reader.as_mut()).await? as usize;

                if size != SIZE {
                    return Err(Error::new(std::io::ErrorKind::InvalidData, "Size mismatch"));
                }

                let mut buffer = [T::zeroed(); SIZE];
                for i in 0..size {
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
