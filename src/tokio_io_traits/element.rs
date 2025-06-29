use super::{AsyncSave, AsyncLoad};
use std::pin::Pin;
use std::io::Error;
use std::future::Future;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

// u8
impl AsyncSave for u8 {
    fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&[*self]).await?;
            Ok(())
        }
    }
    fn save_as_le<W>(&self, writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        self.save_as_ne(writer)
    }
    fn save_as_be<W>(&self, writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        self.save_as_ne(writer)
    }
}
impl AsyncLoad for u8 {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 1];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(buffer[0])
        }
    }
    fn load_as_le<R>(reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        Self::load_as_ne(reader)
    }
    fn load_as_be<R>(reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        Self::load_as_ne(reader)
    }
}
// i8
impl AsyncSave for i8 {
    fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&[*self as u8]).await?;
            Ok(())
        }
    }
    fn save_as_le<W>(&self, writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        self.save_as_ne(writer)
    }
    fn save_as_be<W>(&self, writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        self.save_as_ne(writer)
    }
}
impl AsyncLoad for i8 {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 1];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(buffer[0] as i8)
        }
    }
    fn load_as_le<R>(reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        Self::load_as_ne(reader)
    }
    fn load_as_be<R>(reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        Self::load_as_ne(reader)
    }
}

// u16
impl AsyncSave for u16 {
    fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_ne_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_le<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_le_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_be_bytes()).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for u16 {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 2];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(u16::from_ne_bytes(buffer))
        }
    }
    fn load_as_le<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 2];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(u16::from_le_bytes(buffer))
        }
    }
    fn load_as_be<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 2];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(u16::from_be_bytes(buffer))
        }
    }
}

// i16
impl AsyncSave for i16 {
    fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_ne_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_le<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_le_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_be_bytes()).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for i16 {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 2];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(i16::from_ne_bytes(buffer))
        }
    }
    fn load_as_le<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 2];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(i16::from_le_bytes(buffer))
        }
    }
    fn load_as_be<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 2];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(i16::from_be_bytes(buffer))
        }
    }
}

// u32
impl AsyncSave for u32 {
    fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_ne_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_le<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_le_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_be_bytes()).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for u32 {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 4];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(u32::from_ne_bytes(buffer))
        }
    }
    fn load_as_le<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 4];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(u32::from_le_bytes(buffer))
        }
    }
    fn load_as_be<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 4];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(u32::from_be_bytes(buffer))
        }
    }
}
// i32
impl AsyncSave for i32 {
    fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_ne_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_le<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_le_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_be_bytes()).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for i32 {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 4];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(i32::from_ne_bytes(buffer))
        }
    }
    fn load_as_le<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 4];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(i32::from_le_bytes(buffer))
        }
    }
    fn load_as_be<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 4];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(i32::from_be_bytes(buffer))
        }
    }
}

// u64
impl AsyncSave for u64 {
    fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_ne_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_le<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_le_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_be_bytes()).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for u64 {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 8];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(u64::from_ne_bytes(buffer))
        }
    }
    fn load_as_le<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 8];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(u64::from_le_bytes(buffer))
        }
    }
    fn load_as_be<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 8];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(u64::from_be_bytes(buffer))
        }
    }
}
// i64
impl AsyncSave for i64 {
    fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_ne_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_le<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_le_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_be_bytes()).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for i64 {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 8];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(i64::from_ne_bytes(buffer))
        }
    }
    fn load_as_le<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 8];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(i64::from_le_bytes(buffer))
        }
    }
    fn load_as_be<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 8];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(i64::from_be_bytes(buffer))
        }
    }
}

// u128
impl AsyncSave for u128 {
    fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_ne_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_le<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_le_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_be_bytes()).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for u128 {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 16];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(u128::from_ne_bytes(buffer))
        }
    }
    fn load_as_le<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 16];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(u128::from_le_bytes(buffer))
        }
    }
    fn load_as_be<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 16];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(u128::from_be_bytes(buffer))
        }
    }
}
// i128
impl AsyncSave for i128 {
    fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_ne_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_le<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_le_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_be_bytes()).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for i128 {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 16];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(i128::from_ne_bytes(buffer))
        }
    }
    fn load_as_le<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 16];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(i128::from_le_bytes(buffer))
        }
    }
    fn load_as_be<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 16];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(i128::from_be_bytes(buffer))
        }
    }
}

// usize
impl AsyncSave for usize {
    fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write_all(&self.to_ne_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_le<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write_all(&self.to_le_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write_all(&self.to_be_bytes()).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for usize {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; std::mem::size_of::<usize>()];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(usize::from_ne_bytes(buffer))
        }
    }
    fn load_as_le<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; std::mem::size_of::<usize>()];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(usize::from_le_bytes(buffer))
        }
    }
    fn load_as_be<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; std::mem::size_of::<usize>()];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(usize::from_be_bytes(buffer))
        }
    }
}
// isize
impl AsyncSave for isize {
    fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write_all(&self.to_ne_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_le<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write_all(&self.to_le_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write_all(&self.to_be_bytes()).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for isize {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; std::mem::size_of::<isize>()];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(isize::from_ne_bytes(buffer))
        }
    }
    fn load_as_le<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; std::mem::size_of::<isize>()];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(isize::from_le_bytes(buffer))
        }
    }
    fn load_as_be<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; std::mem::size_of::<isize>()];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(isize::from_be_bytes(buffer))
        }
    }
}

// f32
impl AsyncSave for f32 {
    fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_ne_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_le<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_le_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_be_bytes()).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for f32 {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 4];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(f32::from_ne_bytes(buffer))
        }
    }
    fn load_as_le<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 4];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(f32::from_le_bytes(buffer))
        }
    }
    fn load_as_be<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 4];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(f32::from_be_bytes(buffer))
        }
    }
}
// f64
impl AsyncSave for f64 {
    fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_ne_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_le<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_le_bytes()).await?;
            Ok(())
        }
    }
    fn save_as_be<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
        W: AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write(&self.to_be_bytes()).await?;
            Ok(())
        }
    }
}
impl AsyncLoad for f64 {
    fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 8];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(f64::from_ne_bytes(buffer))
        }
    }
    fn load_as_le<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 8];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(f64::from_le_bytes(buffer))
        }
    }
    fn load_as_be<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
        R: AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let mut buffer = [0u8; 8];
            reader.as_mut().read_exact(&mut buffer).await?;
            Ok(f64::from_be_bytes(buffer))
        }
    }
}

// Tuple implementations
macro_rules! impl_tuple {
    ($($idx:tt : $ty:ident),+) => {
        impl<$($ty: AsyncSave + Send + Sync),+> AsyncSave for ($($ty,)+) {
            fn save_as_ne<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
                W: AsyncWrite + Send,
            {
                async move {
                    $(
                        self.$idx.save_as_ne(writer.as_mut()).await?;
                    )+
                    Ok(())
                }
            }

            fn save_as_le<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
                W: AsyncWrite + Send,
            {
                async move {
                    $(
                        self.$idx.save_as_le(writer.as_mut()).await?;
                    )+
                    Ok(())
                }
            }

            fn save_as_be<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send where
                W: AsyncWrite + Send,
            {
                async move {
                    $(
                        self.$idx.save_as_be(writer.as_mut()).await?;
                    )+
                    Ok(())
                }
            }
        }

        impl<$($ty: AsyncLoad + Send),+> AsyncLoad for ($($ty,)+) {
            fn load_as_ne<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
                R: AsyncRead + Send,
                Self: Sized,
            {
                async move {
                    Ok(($(
                        $ty::load_as_ne(reader.as_mut()).await?,
                    )+))
                }
            }

            fn load_as_le<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
                R: AsyncRead + Send,
                Self: Sized,
            {
                async move {
                    Ok(($(
                        $ty::load_as_le(reader.as_mut()).await?,
                    )+))
                }
            }

            fn load_as_be<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, Error>> + Send where
                R: AsyncRead + Send,
                Self: Sized,
            {
                async move {
                    Ok(($(
                        $ty::load_as_be(reader.as_mut()).await?,
                    )+))
                }
            }
        }
    }
}

impl_tuple!(0: T1);
impl_tuple!(0: T1, 1: T2);
impl_tuple!(0: T1, 1: T2, 2: T3);
impl_tuple!(0: T1, 1: T2, 2: T3, 3: T4);
impl_tuple!(0: T1, 1: T2, 2: T3, 3: T4, 4: T5);
impl_tuple!(0: T1, 1: T2, 2: T3, 3: T4, 4: T5, 5: T6);
impl_tuple!(0: T1, 1: T2, 2: T3, 3: T4, 4: T5, 5: T6, 6: T7);
impl_tuple!(0: T1, 1: T2, 2: T3, 3: T4, 4: T5, 5: T6, 6: T7, 7: T8);
impl_tuple!(0: T1, 1: T2, 2: T3, 3: T4, 4: T5, 5: T6, 6: T7, 7: T8, 8: T9);
impl_tuple!(0: T1, 1: T2, 2: T3, 3: T4, 4: T5, 5: T6, 6: T7, 7: T8, 8: T9, 9: T10);
impl_tuple!(0: T1, 1: T2, 2: T3, 3: T4, 4: T5, 5: T6, 6: T7, 7: T8, 8: T9, 9: T10, 10: T11);
impl_tuple!(0: T1, 1: T2, 2: T3, 3: T4, 4: T5, 5: T6, 6: T7, 7: T8, 8: T9, 9: T10, 10: T11, 11: T12);
