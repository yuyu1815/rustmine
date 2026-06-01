use crate::protocol::*;
use std::io;

#[derive(Default, Debug)]
pub struct ConfigurationKeepAliveClientbound_i64 {
    pub id: i64,
}

impl PacketType for ConfigurationKeepAliveClientbound_i64 {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            super::internal_ids::ConfigurationKeepAliveClientbound_i64,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.id.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationPingClientbound_i32 {
    pub id: i32,
}

impl PacketType for ConfigurationPingClientbound_i32 {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            super::internal_ids::ConfigurationPingClientbound_i32,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.id.write_to(buf)?;
        Ok(())
    }
}
