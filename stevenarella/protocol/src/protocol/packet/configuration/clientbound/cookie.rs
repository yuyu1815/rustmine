use crate::protocol::*;
use std::io;

#[derive(Default, Debug)]
pub struct ConfigurationCookieRequestClientbound {
    pub key: String,
}

impl PacketType for ConfigurationCookieRequestClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            super::internal_ids::ConfigurationCookieRequestClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.key.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationStoreCookieClientbound {
    pub key: String,
    pub payload: Vec<u8>,
}

impl PacketType for ConfigurationStoreCookieClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            super::internal_ids::ConfigurationStoreCookieClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.key.write_to(buf)?;
        VarInt(self.payload.len() as i32).write_to(buf)?;
        buf.write_all(&self.payload)?;
        Ok(())
    }
}
