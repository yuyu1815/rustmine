use crate::protocol::*;
use std::io;

#[derive(Default, Debug)]
pub struct ConfigurationDisconnectClientbound {
    pub reason: format::Component,
}

impl PacketType for ConfigurationDisconnectClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            super::internal_ids::ConfigurationDisconnectClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.reason.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationResetChatClientbound {
    pub empty: (),
}

impl PacketType for ConfigurationResetChatClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            super::internal_ids::ConfigurationResetChatClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.empty.write_to(buf)?;
        Ok(())
    }
}
