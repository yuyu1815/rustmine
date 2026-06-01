use std::io;

use crate::protocol::{Error, LenPrefixed, Serializable, VarInt};

#[derive(Debug, Default)]
pub struct Tags {
    pub tag_name: String,
    pub entries: LenPrefixed<VarInt, VarInt>,
}

impl Serializable for Tags {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        Ok(Tags {
            tag_name: Serializable::read_from(buf)?,
            entries: Serializable::read_from(buf)?,
        })
    }

    fn write_to<W: io::Write>(&self, _: &mut W) -> Result<(), Error> {
        unimplemented!()
    }
}
