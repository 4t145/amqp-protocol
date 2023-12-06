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
};
#[derive(Debug, Default)]
#[repr(u8)]
pub enum State {
    #[default]
    Start = 0,
    HdrRcvd,
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
#[derive(Debug, Clone)]
pub struct AtomicState(Arc<AtomicU8>);

impl AtomicState {
    pub fn new() -> Self {
        Self(AtomicU8::new(State::Start as u8).into())
    }
    pub fn load(&self) -> State {
        State::load_from_atomic_u8(&self.0)
    }
    pub fn store(&self, state: State) {
        state.store(&self.0)
    }
}

impl State {
    pub fn load_from_atomic_u8(state: &AtomicU8) -> Self {
        let state = state.load(std::sync::atomic::Ordering::SeqCst);
        match state {
            0 => State::Start,
            1 => State::HdrRcvd,
            2 => State::HdrSent,
            3 => State::HdrExch,
            4 => State::OpenRcvd,
            5 => State::OpenSent,
            6 => State::OpenPipe,
            7 => State::ClosePipe,
            8 => State::OcPipe,
            9 => State::Opened,
            10 => State::CloseRcvd,
            11 => State::CloseSent,
            12 => State::Discarding,
            13 => State::End,
            _ => unreachable!(),
        }
    }
    pub fn store(self, state: &AtomicU8) {
        state.store(self as u8, std::sync::atomic::Ordering::SeqCst);
    }
}

pub struct ServerConnection<'c> {
    pub stream: TcpStream,
    pub exthdr_buffer: &'c mut BytesMut,
    pub body_buffer: &'c mut BytesMut,
}

impl<'c> ServerConnection<'c> {
    pub async fn negotiation(mut self) -> io::Result<()> {
        let mut state = AtomicState::new();
        let stream: &mut TcpStream = &mut self.stream;
        let (mut rx, mut tx) = stream.split();
        const THIS_VERSION: Version = Version::V_1_0_0;
        let rhalf = async move {
            let version = Version::async_decode(stream).await?;
            match state.load() {
                State::Start => {
                    if version == THIS_VERSION {
                        state.store(State::HdrRcvd);
                        Version::V_1_0_0.async_encode(stream).await?;
                    } else {
                        state.store(State::End);
                    }
                }
                State::HdrSent => todo!(),
                State::HdrExch => todo!(),
                State::OpenPipe => todo!(),
                State::OcPipe => todo!(),
                _ => {
                    todo!("close")
                }
            }

            
        };
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
        tx.
        // rx.read_buf(&mut buf);
        // tokio::select! {}
        Ok(())
    }
}
