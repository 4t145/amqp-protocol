
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uuid(pub(crate) [u8; 16]);


impl From<[u8; 16]> for Uuid 
{
    fn from(value: [u8; 16]) -> Self {
        Uuid(value)
    }
}