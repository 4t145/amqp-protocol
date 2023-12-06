use std::sync::{atomic::AtomicU8, Arc};

use crate::{
    transport::{framing::FrameHeader, performative::open::Open, version::Version},
    types::{
        encoding::{
            de::{async_reader::AsyncDecode, slice::View},
            enc::async_writer::AsyncEncode,
        },
        serde::de::{from_slice, DeserializeError},
        value::Value,
    },
};

use bytes::BytesMut;
use serde::Deserialize;
use tokio::{
    io::{self, AsyncReadExt},
    net::TcpStream,
    sync::Mutex,
};
#[derive(Debug, Default)]
#[repr(u8)]
pub enum State {
    #[default]
    Start = 0,
    HdrRcvd(Version),
    HdrSent,
    HdrExch,
    OpenRcvd,
    OpenSent,
    OpenPipe,
    ClosePipe,
    OcPipe,
    Opened,
    CloseRcvd,
    CloseSent,
    Discarding,
    End,
}

impl State {}

pub struct ServerConnection<'c> {
    pub container_id: &'c str,
    pub stream: TcpStream,
    pub exthdr_buffer: &'c mut BytesMut,
    pub body_buffer: &'c mut BytesMut,
}

impl<'c> ServerConnection<'c> {
    pub async fn negotiation(mut self) -> io::Result<()> {
        let mut state = State::Start;
        let (mut rx, mut tx) = self.stream.into_split();
        const THIS_VERSION: Version = Version::V_1_0_0;
        loop {
            match state {
                State::Start => {
                    tokio::select! {
                        result = rx.readable() => {
                            result?;
                            let version = Version::async_decode(&mut rx).await?;
                            state = State::HdrRcvd(version);
                        },
                        result = tx.writable() => {
                            result?;
                            let version = Version::async_encode(&THIS_VERSION, &mut tx).await?;
                            state = State::HdrExch;
                        }
                    }
                }
                State::HdrRcvd(version) => {
                    if version != THIS_VERSION {
                        state = State::End;
                    } else {
                        THIS_VERSION.async_encode(&mut tx).await?;
                        state = State::HdrSent;
                    }
                }
                State::HdrSent => {
                    tokio::select! {
                        result = rx.readable() => {
                            result?;
                            let version = Version::async_decode(&mut rx).await?;
                            state = State::HdrRcvd(version);
                        },
                        result = tx.writable() => {
                            result?;
                            let open = todo!();
                            let open = Open {
                                container_id: self.container_id,
                                ..Default::default()
                            };
                            state = State::HdrExch;
                        }
                    }
                }
                State::HdrExch => todo!(),
                State::OpenRcvd => todo!(),
                State::OpenSent => todo!(),
                State::OpenPipe => todo!(),
                State::ClosePipe => todo!(),
                State::OcPipe => todo!(),
                State::Opened => todo!(),
                State::CloseRcvd => todo!(),
                State::CloseSent => todo!(),
                State::Discarding => todo!(),
                State::End => todo!(),
            }
        }

        Ok(())
    }

    pub async fn start(mut self) -> io::Result<()> {
        let stream: &mut TcpStream = &mut self.stream;
        let (mut rx, mut tx) = stream.split();
        // +8
        let header = FrameHeader::async_decode(&mut rx).await?;
        let exthdr_size = header
            .exthdr_size()
            .ok_or(io::Error::other("invalid exthdr_size"))?;
        let body_size = header
            .body_size()
            .ok_or(io::Error::other("invalid body_size"))?;
        self.exthdr_buffer.resize(exthdr_size, 0);
        self.body_buffer.resize(body_size, 0);
        rx.read_exact(self.exthdr_buffer).await?;
        rx.read_exact(self.body_buffer).await?;
        let open = from_slice::<Open>(&self.body_buffer[..])?;
        // rx.read_buf(&mut buf);
        // tokio::select! {}
        Ok(())
    }
}
