use std::{io, net::TcpStream};

use crate::{transport::{version::Version, performative::open::Open}, types::serde::de::from_reader};

pub fn bind(mut stream: TcpStream) -> io::Result<()> {
    let version = Version::read(&mut stream)?;
    // wait a version and response a version, hear we just use v1.0.0
    if Version::V_1_0_0 != version {
        return Ok(());
    }
    Version::V_1_0_0.write(&mut stream)?;
    let open = from_reader::<Open>(&mut stream)?;
    
    Ok(())
}
