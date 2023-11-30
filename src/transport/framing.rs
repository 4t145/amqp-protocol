use std::io::{self, Read, Write};

pub struct FrameHead {
    pub size: u32,
    pub doff: u8,
    pub frame_type: u8,
}

impl FrameHead {
    pub fn write<W: Write>(self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.size.to_be_bytes())?;
        writer.write_all(&[self.doff, self.frame_type])
    }
    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf);
        let size = u32::from_be_bytes(buf);
        let mut buf = [0; 2];
        reader.read_exact(&mut buf);
        let (doff, frame_type) = buf.into();
        Ok(FrameHead {
            size,
            doff,
            frame_type,
        })
    }
}
