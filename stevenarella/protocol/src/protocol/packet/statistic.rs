use std::io;

use crate::protocol::{Error, Serializable, VarInt};

#[derive(Debug, Default)]
pub struct Statistic {
    pub name: String,
    pub value: VarInt,
}

impl Serializable for Statistic {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        Ok(Statistic {
            name: Serializable::read_from(buf)?,
            value: Serializable::read_from(buf)?,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.name.write_to(buf)?;
        self.value.write_to(buf)
    }
}
