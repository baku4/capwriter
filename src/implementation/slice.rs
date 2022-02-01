#[allow(unused_imports)]
use super::{
    Result, error_msg,
    EndianType, WriteBytesExt, ReadBytesExt,
    Saveable, Loadable,
};


// u8
impl Saveable for &[u8] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        writer.write_all(self)?;

        Ok(())
    }
}

// u16
impl Saveable for &[u16] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        self.iter().for_each(|v| {
            writer.write_u16::<EndianType>(*v);
        });

        Ok(())
    }
}


// u32
impl Saveable for &[u32] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        self.iter().for_each(|v| {
            writer.write_u32::<EndianType>(*v);
        });

        Ok(())
    }
}


// u64
impl Saveable for &[u64] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        self.iter().for_each(|v| {
            writer.write_u64::<EndianType>(*v);
        });

        Ok(())
    }
}


// usize
impl Saveable for &[usize] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        self.iter().for_each(|v| {
            #[cfg(target_pointer_width = "32")]
            writer.write_u32::<EndianType>(*v as u32);
            #[cfg(target_pointer_width = "64")]
            writer.write_u64::<EndianType>(*v as u64);
        });

        Ok(())
    }
}


// i16
impl Saveable for &[i16] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        self.iter().for_each(|v| {
            writer.write_i16::<EndianType>(*v);
        });

        Ok(())
    }
}


// i32
impl Saveable for &[i32] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        self.iter().for_each(|v| {
            writer.write_i32::<EndianType>(*v);
        });

        Ok(())
    }
}


// i64
impl Saveable for &[i64] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        self.iter().for_each(|v| {
            writer.write_i64::<EndianType>(*v);
        });

        Ok(())
    }
}


// isize
impl Saveable for &[isize] {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write
    {
        #[cfg(target_pointer_width = "32")]
        writer.write_u32::<EndianType>(self.len() as u32)?;
        #[cfg(target_pointer_width = "64")]
        writer.write_u64::<EndianType>(self.len() as u64)?;

        self.iter().for_each(|v| {
            #[cfg(target_pointer_width = "32")]
            writer.write_i32::<EndianType>(*v as i32);
            #[cfg(target_pointer_width = "64")]
            writer.write_i64::<EndianType>(*v as i64);
        });

        Ok(())
    }
}
