use std::io;
pub mod connection;

pub trait State<R: io::Read>: Sized {
    fn enter(reader: &mut R) -> io::Result<Self>;
}
