use std::io::{self, Read, Write};

use crate::definitions::{MAJOR, MINOR, REVISION};
#[derive(Debug, PartialEq, Eq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub revision: u8,
}

impl Version {
    pub const V_1_0_0: Self = Version {
        major: MAJOR,
        minor: MINOR,
        revision: REVISION,
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

    pub fn try_parse(data: [u8; 8]) -> io::Result<Self> {
        if &data[0..5] == b"AMQP\xd0" {
            Ok(Version {
                major: data[5],
                minor: data[6],
                revision: data[7],
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
