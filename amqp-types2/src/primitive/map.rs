use std::io;

use crate::{constructor::Constructor, value::Value};

use super::ListIter;
#[derive(Debug, Clone)]
pub struct MapIter<'frame> {
    inner: ListIter<'frame>,
}

impl<'frame> Iterator for MapIter<'frame> {
    type Item = io::Result<(Value<'frame>, Value<'frame>)>;
    fn next(&mut self) -> Option<Self::Item> {
        let key = self.inner.next()?;
        match key {
            Ok(key) => {
                let value = self.inner.next()?;
                match value {
                    Ok(value) => Some(Ok((key, value))),
                    Err(err) => Some(Err(err)),
                }
            }
            Err(err) => Some(Err(err)),
        }
    }
}
