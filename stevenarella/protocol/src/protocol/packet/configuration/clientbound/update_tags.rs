use crate::protocol::*;
use std::io;

#[derive(Default, Debug)]
pub struct ConfigurationUpdateTagsTagPayload {
    pub tag_key: String,
    pub entry_ids: Vec<i32>,
}

#[derive(Default, Debug)]
pub struct ConfigurationUpdateTagsRegistryPayload {
    pub registry_key: String,
    pub tags: Vec<ConfigurationUpdateTagsTagPayload>,
}

#[derive(Default, Debug)]
pub struct ConfigurationUpdateTagsClientbound {
    pub registry_payloads: Vec<ConfigurationUpdateTagsRegistryPayload>,
}

impl PacketType for ConfigurationUpdateTagsClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            super::internal_ids::ConfigurationUpdateTagsClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        VarInt(self.registry_payloads.len() as i32).write_to(buf)?;
        for registry in &self.registry_payloads {
            registry.registry_key.write_to(buf)?;
            VarInt(registry.tags.len() as i32).write_to(buf)?;
            for tag in &registry.tags {
                tag.tag_key.write_to(buf)?;
                VarInt(tag.entry_ids.len() as i32).write_to(buf)?;
                for entry_id in &tag.entry_ids {
                    VarInt(*entry_id).write_to(buf)?;
                }
            }
        }
        Ok(())
    }
}
