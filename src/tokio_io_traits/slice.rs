use super::AsyncSave;
use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use bytemuck::Pod;

impl<T: Pod + Sync> AsyncSave for &[T] {
    async fn save_as_le<'a, W>(&'a self, mut writer: Pin<&'a mut W>) -> Result<(), Error> where
            W: AsyncWrite
    {
        let length = self.len() as u64;
        writer.as_mut().write_all(&length.to_le_bytes()).await?;
        writer.as_mut().write_all(bytemuck::cast_slice(self)).await?;
        Ok(())
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let length = self.len() as u64;
            writer.write_all(&length.to_be_bytes()).await?;
            writer.write_all(bytemuck::cast_slice(self)).await?;
            Ok(())
        }
    }   
}
