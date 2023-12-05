use crate::{
    transport::{performative::open::Open, version::Version},
    types::encoding::{de::async_reader::AsyncDecode, enc::async_writer::AsyncEncode},
};

use tokio::{io, net::TcpStream};
pub async fn server_bind(stream: &mut TcpStream) -> io::Result<()> {
    // version negotiation
    let version = Version::async_decode(stream).await?;
    // wait a version and response a version, hear we just use v1.0.0
    if Version::V_1_0_0 != version {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "expect amqp 1.0.0",
        ));
    }
    Version::V_1_0_0.async_encode(stream).await?;

    //
    Ok(())
}

// pub async fn start(stream: &mut TcpStream) {
//     let (rx, tx) = stream.split();
//     let read_buf = io::ReadBuf::new(&mut [0; 1024]);
//     rx.read_buf(buf);
//     tokio::select! {}
// }
