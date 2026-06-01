use std::io;

use crate::protocol::{Error, LenPrefixedBytes, Serializable, VarInt};

#[derive(Debug, Default)]
pub struct BlockChangeRecord {
    pub xz: u8,
    pub y: u8,
    pub block_id: VarInt,
}

impl Serializable for BlockChangeRecord {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        Ok(BlockChangeRecord {
            xz: Serializable::read_from(buf)?,
            y: Serializable::read_from(buf)?,
            block_id: Serializable::read_from(buf)?,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.xz.write_to(buf)?;
        self.y.write_to(buf)?;
        self.block_id.write_to(buf)
    }
}

#[derive(Debug, Default)]
pub struct ChunkMeta {
    pub x: i32,
    pub z: i32,
    pub bitmask: u16,
}

impl Serializable for ChunkMeta {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        Ok(ChunkMeta {
            x: Serializable::read_from(buf)?,
            z: Serializable::read_from(buf)?,
            bitmask: Serializable::read_from(buf)?,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.x.write_to(buf)?;
        self.z.write_to(buf)?;
        self.bitmask.write_to(buf)
    }
}

#[derive(Debug, Default)]
pub struct ChunkBiomeData {
    pub pos: i64,
    pub buffer: LenPrefixedBytes<VarInt>,
}

impl Serializable for ChunkBiomeData {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        Ok(ChunkBiomeData {
            pos: Serializable::read_from(buf)?,
            buffer: Serializable::read_from(buf)?,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.pos.write_to(buf)?;
        self.buffer.write_to(buf)
    }
}

#[derive(Debug, Default)]
pub struct ExplosionRecord {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Serializable for ExplosionRecord {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        Ok(ExplosionRecord {
            x: Serializable::read_from(buf)?,
            y: Serializable::read_from(buf)?,
            z: Serializable::read_from(buf)?,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.x.write_to(buf)?;
        self.y.write_to(buf)?;
        self.z.write_to(buf)
    }
}

#[derive(Debug, Default)]
pub struct MapIcon {
    pub direction_type: i8,
    pub x: i8,
    pub z: i8,
}

impl Serializable for MapIcon {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        Ok(MapIcon {
            direction_type: Serializable::read_from(buf)?,
            x: Serializable::read_from(buf)?,
            z: Serializable::read_from(buf)?,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.direction_type.write_to(buf)?;
        self.x.write_to(buf)?;
        self.z.write_to(buf)
    }
}
