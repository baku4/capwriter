use super::{AsyncSave, AsyncLoad};
use std::pin::Pin;
use std::io::Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

// u8
impl AsyncSave for u8 {
    async fn save_as_le<'a, W>(&'a self, mut writer: Pin<&'a mut W>) -> Result<(), Error> where
        W: AsyncWrite
    {
        writer.as_mut().write(&[*self]).await?;
        Ok(())
    }
    async fn save_as_be<'a, W>(&'a self, mut writer: Pin<&'a mut W>) -> Result<(), Error> where
        W: AsyncWrite
    {
        writer.as_mut().write(&[*self]).await?;
        Ok(())
    }
}
impl AsyncLoad for u8 {
    async fn load_as_le<R>(mut reader: Pin<&mut R>) -> Result<Self, Error> where
        R: AsyncRead,
        Self: Sized,
    {
        let mut buf = [0; 1];
        reader.as_mut().read_exact(&mut buf).await?;
        Ok(buf[0])
    }
    async fn load_as_be<R>(mut reader: Pin<&mut R>) -> Result<Self, Error> where
        R: AsyncRead,
        Self: Sized,
    {
        let mut buf = [0; 1];
        reader.as_mut().read_exact(&mut buf).await?;
        Ok(buf[0])
    }
}
