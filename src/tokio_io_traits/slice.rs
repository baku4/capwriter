use super::AsyncSave;
use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use bytemuck::Pod;

impl<T: AsyncSave + Pod + Sync> AsyncSave for &[T] {
    fn save_as_ne<'a, W >(&'a self, mut writer: Pin<&'a mut W>) -> impl Future<Output = Result<(), Error>> + Send where 
        W: AsyncWrite + Send
    {
        async move {
            let length = self.len() as u64;
            writer.write_all(&length.to_ne_bytes()).await?;
            writer.write_all(bytemuck::cast_slice(self)).await?;
            Ok(())
        }
    }
    fn save_as_le<'a, W>(&'a self, mut writer: Pin<&'a mut W>) -> impl Future<Output = Result<(), Error>> + Send where 
        W: AsyncWrite + Send
    {
        async move {
            #[cfg(target_endian = "little")]
            {
                self.save_as_ne(writer).await
            }
            #[cfg(target_endian = "big")]
            {
                let length = self.len() as u64;
                writer.as_mut().write_all(&length.to_le_bytes()).await?;
                for x in self.iter() {
                    x.save_as_le(writer.as_mut()).await?;
                }
                Ok(())
            }
        }
    }
    fn save_as_be<'a, W>(&'a self, mut writer: Pin<&'a mut W>) -> impl Future<Output = Result<(), Error>> + Send where 
        W: AsyncWrite + Send
    {
        async move {
            #[cfg(target_endian = "little")]
            {
                let length = self.len() as u64;
                writer.as_mut().write_all(&length.to_be_bytes()).await?;
                for x in self.iter() {
                    x.save_as_be(writer.as_mut()).await?;
                }
                Ok(())
            }
            #[cfg(target_endian = "big")]
            {
                self.save_as_ne(writer).await
            }
        }
    }
}
