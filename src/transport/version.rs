use std::io::{self, Write, Read};

pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub revision: u8,
}

impl Version {
    pub fn write<W: Write>(self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&[
            b'A',
            b'M',
            b'Q',
            b'P',
            0xd0,
            self.major,
            self.minor,
            self.revision,
        ])
    }
    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut buf = [0;8];
        reader.read_exact(&mut buf);
        if &buf[0..5] == b"AMQP\xd0" {
            Ok(Version {
                major: buf[5],
                minor: buf[6],
                revision: buf[7],
            })
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Invalid AMQP version"))
        }
    }
}
