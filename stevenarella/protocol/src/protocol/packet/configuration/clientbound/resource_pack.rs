use crate::protocol::*;
use std::io;

#[derive(Default, Debug)]
pub struct ConfigurationResourcePackPopClientbound {
    pub id_present: bool,
    pub id: Option<UUID>,
}

impl PacketType for ConfigurationResourcePackPopClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            super::internal_ids::ConfigurationResourcePackPopClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.id_present.write_to(buf)?;
        if let Some(ref id) = self.id {
            id.write_to(buf)?;
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationResourcePackPushClientbound {
    pub id: UUID,
    pub url: String,
    pub hash: String,
    pub required: bool,
    pub prompt_present: bool,
    pub prompt_data: Vec<u8>,
}

impl PacketType for ConfigurationResourcePackPushClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            super::internal_ids::ConfigurationResourcePackPushClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.id.write_to(buf)?;
        self.url.write_to(buf)?;
        self.hash.write_to(buf)?;
        self.required.write_to(buf)?;
        self.prompt_present.write_to(buf)?;
        self.prompt_data.write_to(buf)?;
        Ok(())
    }
}
