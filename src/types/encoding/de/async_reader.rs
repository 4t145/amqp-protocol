use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWrite};

use crate::transport::{version::Version, framing::FrameHeader};

pub trait AsyncDecode<R: AsyncRead>: Sized {
    async fn async_decode(reader: &mut R) -> io::Result<Self>;
}

impl<R: AsyncRead + Unpin> AsyncDecode<R> for char {
    async fn async_decode(reader: &mut R) -> io::Result<Self> {
        let charcode = reader.read_u32().await?;
        char::from_u32(charcode).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid charcode",
        ))
    }
}

impl<const N: usize, R: AsyncRead + Unpin> AsyncDecode<R> for [u8; N] {
    async fn async_decode(reader: &mut R) -> io::Result<Self> {
        let mut buf = [0; N];
        reader.read_exact(&mut buf).await?;
        Ok(buf)
    }
}

// protocol types
//
//
//
//

impl<R: tokio::io::AsyncRead + Unpin> AsyncDecode<R> for Version {
    async fn async_decode(reader: &mut R) -> io::Result<Self> {
        let mut buf = [0; 8];
        reader.read_exact(&mut buf);
        Self::try_parse(buf)
    }
}

// Framing

impl<R: tokio::io::AsyncRead + Unpin> AsyncDecode<R> for FrameHeader {
    async fn async_decode(reader: &mut R) -> io::Result<Self> {
        let size = reader.read_u32().await?;
        let doff = reader.read_u8().await?;
        let frame_type = reader.read_u8().await?;
        let ext = reader.read_u16().await?;
        Ok(FrameHeader {
            size,
            doff,
            frame_type,
            ext,
        })
    }
}
