use crate::types::encoding::{de::async_reader::AsyncDecode, enc};
use std::io::{self, Read, Write};
#[derive(Debug, PartialEq, Eq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub revision: u8,
}

impl Version {
    pub const V_1_0_0: Self = Version {
        major: 1,
        minor: 0,
        revision: 0,
    };
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
        let mut buf = [0; 8];
        Self::try_parse(buf)
    }

    pub fn try_parse(datas: [u8; 8]) -> io::Result<Self> {
        if &datas[0..5] == b"AMQP\xd0" {
            Ok(Version {
                major: datas[5],
                minor: datas[6],
                revision: datas[7],
            })
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid AMQP version",
            ))
        }
    }

    pub fn as_bytes(&self) -> [u8; 8] {
        [
            b'A',
            b'M',
            b'Q',
            b'P',
            0xd0,
            self.major,
            self.minor,
            self.revision,
        ]
    }
}
