#[allow(unused_imports)]
use super::{
    Result, error_msg,
    EndianType, WriteBytesExt, ReadBytesExt,
    Saveable, Loadable,
};


// u8
impl Saveable for Vec<u8> {
    fn save_to<W>(&self, writer: W) -> Result<()> where
        W: std::io::Write
    {
        (self as &[u8]).save_to(writer)
    }
}
impl Loadable for Vec<u8> {
    fn load_from<R>(mut reader: R) -> Result<Self> where
        R: std::io::Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        reader.read_exact(&mut buffer)?;

        Ok(buffer)
    }
}


// u16
impl Saveable for Vec<u16> {
    fn save_to<W>(&self, writer: W) -> Result<()> where
        W: std::io::Write
    {
        (self as &[u16]).save_to(writer)
    }
}
impl Loadable for Vec<u16> {
    fn load_from<R>(mut reader: R) -> Result<Self> where
        R: std::io::Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        reader.read_u16_into::<EndianType>(&mut buffer)?;

        Ok(buffer)
    }
}


// u32
impl Saveable for Vec<u32> {
    fn save_to<W>(&self, writer: W) -> Result<()> where
        W: std::io::Write
    {
        (self as &[u32]).save_to(writer)
    }
}
impl Loadable for Vec<u32> {
    fn load_from<R>(mut reader: R) -> Result<Self> where
        R: std::io::Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        reader.read_u32_into::<EndianType>(&mut buffer)?;

        Ok(buffer)
    }
}


// u64
impl Saveable for Vec<u64> {
    fn save_to<W>(&self, writer: W) -> Result<()> where
        W: std::io::Write
    {
        (self as &[u64]).save_to(writer)
    }
}
impl Loadable for Vec<u64> {
    fn load_from<R>(mut reader: R) -> Result<Self> where
        R: std::io::Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        reader.read_u64_into::<EndianType>(&mut buffer)?;

        Ok(buffer)
    }
}


// usize
impl Saveable for Vec<usize> {
    fn save_to<W>(&self, writer: W) -> Result<()> where
        W: std::io::Write
    {
        (self as &[usize]).save_to(writer)
    }
}
impl Loadable for Vec<usize> {
    fn load_from<R>(reader: R) -> Result<Self> where
        R: std::io::Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let buffer = Vec::<u32>::load_from(reader)?;
        #[cfg(target_pointer_width = "64")]
        let buffer = Vec::<u64>::load_from(reader)?;

        Ok(buffer.into_iter().map(|v| v as usize).collect())
    }
}


// i16
impl Saveable for Vec<i16> {
    fn save_to<W>(&self, writer: W) -> Result<()> where
        W: std::io::Write
    {
        (self as &[i16]).save_to(writer)
    }
}
impl Loadable for Vec<i16> {
    fn load_from<R>(mut reader: R) -> Result<Self> where
        R: std::io::Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        reader.read_i16_into::<EndianType>(&mut buffer)?;

        Ok(buffer)
    }
}


// i32
impl Saveable for Vec<i32> {
    fn save_to<W>(&self, writer: W) -> Result<()> where
        W: std::io::Write
    {
        (self as &[i32]).save_to(writer)
    }
}
impl Loadable for Vec<i32> {
    fn load_from<R>(mut reader: R) -> Result<Self> where
        R: std::io::Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        reader.read_i32_into::<EndianType>(&mut buffer)?;

        Ok(buffer)
    }
}


// i64
impl Saveable for Vec<i64> {
    fn save_to<W>(&self, writer: W) -> Result<()> where
        W: std::io::Write
    {
        (self as &[i64]).save_to(writer)
    }
}
impl Loadable for Vec<i64> {
    fn load_from<R>(mut reader: R) -> Result<Self> where
        R: std::io::Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let len = reader.read_u32::<EndianType>()? as usize;
        #[cfg(target_pointer_width = "64")]
        let len = reader.read_u64::<EndianType>()? as usize;

        let mut buffer = vec![0; len];
        reader.read_i64_into::<EndianType>(&mut buffer)?;

        Ok(buffer)
    }
}


// isize
impl Saveable for Vec<isize> {
    fn save_to<W>(&self, writer: W) -> Result<()> where
        W: std::io::Write
    {
        (self as &[isize]).save_to(writer)
    }
}
impl Loadable for Vec<isize> {
    fn load_from<R>(reader: R) -> Result<Self> where
        R: std::io::Read,
        Self: Sized,
    {
        #[cfg(target_pointer_width = "32")]
        let buffer = Vec::<i32>::load_from(reader)?;
        #[cfg(target_pointer_width = "64")]
        let buffer = Vec::<i64>::load_from(reader)?;

        Ok(buffer.into_iter().map(|v| v as isize).collect())
    }
}
