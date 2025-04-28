use super::{AsyncSave, AsyncLoad};
use std::future::Future;
use std::io::Error;
use tokio::io::{AsyncWrite, AsyncRead, AsyncWriteExt, AsyncReadExt};

// u8
impl AsyncSave for u8 {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            writer.write_all(&[*self]).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            writer.write_all(&[*self]).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for u8 {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 1];
            reader.read_exact(&mut buf).await?;
            Ok(buf[0])
        }
    }
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 1];
            reader.read_exact(&mut buf).await?;
            Ok(buf[0])
        }
    }
}
// i8
impl AsyncSave for i8 {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            writer.write_all(&[*self as u8]).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            writer.write_all(&[*self as u8]).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for i8 {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 1];
            reader.read_exact(&mut buf).await?;
            Ok(buf[0] as i8)
        }
    }
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 1];
            reader.read_exact(&mut buf).await?;
            Ok(buf[0] as i8)
        }
    }
}
// u16
impl AsyncSave for u16 {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_le_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_be_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for u16 {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 2];
            reader.read_exact(&mut buf).await?;
            Ok(u16::from_le_bytes(buf))
        }
    }
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 2];
            reader.read_exact(&mut buf).await?;
            Ok(u16::from_be_bytes(buf))
        }
    }
}
// i16
impl AsyncSave for i16 {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_le_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_be_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for i16 {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 2];
            reader.read_exact(&mut buf).await?;
            Ok(i16::from_le_bytes(buf))
        }
    }
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 2];
            reader.read_exact(&mut buf).await?;
            Ok(i16::from_be_bytes(buf))
        }
    }
}
// u32
impl AsyncSave for u32 {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_le_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_be_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for u32 {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf).await?;
            Ok(u32::from_le_bytes(buf))
        }
    }
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf).await?;
            Ok(u32::from_be_bytes(buf))
        }
    }
}
// i32
impl AsyncSave for i32 {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_le_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_be_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for i32 {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf).await?;
            Ok(i32::from_le_bytes(buf))
        }
    }
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf).await?;
            Ok(i32::from_be_bytes(buf))
        }
    }
}
// u64
impl AsyncSave for u64 {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_le_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_be_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for u64 {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 8];
            reader.read_exact(&mut buf).await?;
            Ok(u64::from_le_bytes(buf))
        }
    }
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 8];
            reader.read_exact(&mut buf).await?;
            Ok(u64::from_be_bytes(buf))
        }
    }
}
// i64
impl AsyncSave for i64 {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_le_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_be_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for i64 {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 8];
            reader.read_exact(&mut buf).await?;
            Ok(i64::from_le_bytes(buf))
        }
    }
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 8];
            reader.read_exact(&mut buf).await?;
            Ok(i64::from_be_bytes(buf))
        }
    }
}
// usize
impl AsyncSave for usize {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            #[cfg(target_pointer_width = "32")]
            {
                let buf = (*self as u32).to_le_bytes();
                writer.write_all(&buf).await?;
                Ok(())
            }
            #[cfg(target_pointer_width = "64")]
            {
                let buf = (*self as u64).to_le_bytes();
                writer.write_all(&buf).await?;
                Ok(())
            }
        }
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            #[cfg(target_pointer_width = "32")]
            {
                let buf = (*self as u32).to_be_bytes();
                writer.write_all(&buf).await?;
                Ok(())
            }
            #[cfg(target_pointer_width = "64")]
            {
                let buf = (*self as u64).to_be_bytes();
                writer.write_all(&buf).await?;
                Ok(())
            }
        }
    }
}
impl AsyncLoad for usize {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            #[cfg(target_pointer_width = "32")]
            {
                let mut buf = [0; 4];
                reader.read_exact(&mut buf).await?;
                Ok(u32::from_le_bytes(buf) as usize)
            }
            #[cfg(target_pointer_width = "64")]
            {
                let mut buf = [0; 8];
                reader.read_exact(&mut buf).await?;
                Ok(u64::from_le_bytes(buf) as usize)
            }
        }
    }
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            #[cfg(target_pointer_width = "32")]
            {
                let mut buf = [0; 4];
                reader.read_exact(&mut buf).await?;
                Ok(u32::from_be_bytes(buf) as usize)
            }
            #[cfg(target_pointer_width = "64")]
            {
                let mut buf = [0; 8];
                reader.read_exact(&mut buf).await?;
                Ok(u64::from_be_bytes(buf) as usize)
            }
        }
    }
}
// isize
impl AsyncSave for isize {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            #[cfg(target_pointer_width = "32")]
            {
                let buf = (*self as i32).to_le_bytes();
                writer.write_all(&buf).await?;
                Ok(())
            }
            #[cfg(target_pointer_width = "64")]
            {
                let buf = (*self as i64).to_le_bytes();
                writer.write_all(&buf).await?;
                Ok(())
            }
        }
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            #[cfg(target_pointer_width = "32")]
            {
                let buf = (*self as i32).to_be_bytes();
                writer.write_all(&buf).await?;
                Ok(())
            }
            #[cfg(target_pointer_width = "64")]
            {
                let buf = (*self as i64).to_be_bytes();
                writer.write_all(&buf).await?;
                Ok(())
            }
        }
    }
}
impl AsyncLoad for isize {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            #[cfg(target_pointer_width = "32")]
            {
                let mut buf = [0; 4];
                reader.read_exact(&mut buf).await?;
                Ok(i32::from_le_bytes(buf) as isize)
            }
            #[cfg(target_pointer_width = "64")]
            {
                let mut buf = [0; 8];
                reader.read_exact(&mut buf).await?;
                Ok(i64::from_le_bytes(buf) as isize)
            }
        }
    }
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            #[cfg(target_pointer_width = "32")]
            {
                let mut buf = [0; 4];
                reader.read_exact(&mut buf).await?;
                Ok(i32::from_be_bytes(buf) as isize)
            }
            #[cfg(target_pointer_width = "64")]
            {
                let mut buf = [0; 8];
                reader.read_exact(&mut buf).await?;
                Ok(i64::from_be_bytes(buf) as isize)
            }
        }
    }
}

// f32
impl AsyncSave for f32 {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_le_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_be_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for f32 {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf).await?;
            Ok(f32::from_le_bytes(buf))
        }
    }
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf).await?;
            Ok(f32::from_be_bytes(buf))
        }
    }
}
// f64
impl AsyncSave for f64 {
    fn save_as_le<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_le_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, writer: &mut W) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Unpin + Send
    {
        async move {
            let buf = self.to_be_bytes();
            writer.write_all(&buf).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for f64 {
    fn load_as_le<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 8];
            reader.read_exact(&mut buf).await?;
            Ok(f64::from_le_bytes(buf))
        }
    }
    fn load_as_be<R>(reader: &mut R) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Unpin + Send,
        Self: Sized,
    {
        async move {
            let mut buf = [0; 8];
            reader.read_exact(&mut buf).await?;
            Ok(f64::from_be_bytes(buf))
        }
    }
}
