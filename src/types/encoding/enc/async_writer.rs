use tokio::io::{self, AsyncWrite, AsyncWriteExt};

use crate::transport::{version::Version, framing::FrameHeader};

pub trait AsyncEncode<W: AsyncWrite>: Sized {
    async fn async_encode(&self, writer: &mut W) -> io::Result<()>;
}

impl<W: AsyncWrite + Unpin> AsyncEncode<W> for Version {
    async fn async_encode(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.as_bytes()).await
    }
}


// Framing
// 

impl<W: AsyncWrite + Unpin> AsyncEncode<W> for FrameHeader {
    async fn async_encode(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.size.to_be_bytes()).await?;
        writer.write_all(&[self.doff, self.frame_type]).await
    }
}