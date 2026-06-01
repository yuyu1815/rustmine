use std::io;

use crate::protocol::{Error, Serializable};

#[derive(Debug, Default)]
pub struct SpawnProperty {
    pub name: String,
    pub value: String,
    pub signature: String,
}

impl Serializable for SpawnProperty {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        Ok(SpawnProperty {
            name: Serializable::read_from(buf)?,
            value: Serializable::read_from(buf)?,
            signature: Serializable::read_from(buf)?,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.name.write_to(buf)?;
        self.value.write_to(buf)?;
        self.signature.write_to(buf)
    }
}
