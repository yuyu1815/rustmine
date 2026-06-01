use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(super) fn read_update_tags_configuration_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::clientbound::internal_ids::ConfigurationUpdateTagsClientbound => {
            let registry_payload_count = VarInt::read_from(buf)?.0;
            if registry_payload_count < 0 {
                return Err(Error::Err(format!(
                    "negative update_tags registry-payload count {}",
                    registry_payload_count
                )));
            }
            let mut registry_payloads = Vec::with_capacity(registry_payload_count as usize);
            for _ in 0..registry_payload_count {
                let registry_key = Serializable::read_from(buf)?;
                let tag_count = VarInt::read_from(buf)?.0;
                if tag_count < 0 {
                    return Err(Error::Err(format!(
                        "negative update_tags tag count {}",
                        tag_count
                    )));
                }
                let mut tags = Vec::with_capacity(tag_count as usize);
                for _ in 0..tag_count {
                    let tag_key = Serializable::read_from(buf)?;
                    let entry_count = VarInt::read_from(buf)?.0;
                    if entry_count < 0 {
                        return Err(Error::Err(format!(
                            "negative update_tags entry count {}",
                            entry_count
                        )));
                    }
                    let mut entry_ids = Vec::with_capacity(entry_count as usize);
                    for _ in 0..entry_count {
                        entry_ids.push(VarInt::read_from(buf)?.0);
                    }
                    tags.push(
                        packet::configuration::clientbound::ConfigurationUpdateTagsTagPayload {
                            tag_key,
                            entry_ids,
                        },
                    );
                }
                registry_payloads.push(
                    packet::configuration::clientbound::ConfigurationUpdateTagsRegistryPayload {
                        registry_key,
                        tags,
                    },
                );
            }
            let _packet = packet::configuration::clientbound::ConfigurationUpdateTagsClientbound {
                registry_payloads,
            };
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "UpdateTags".to_owned(),
                    data: Vec::new(),
                },
            )))
        }
        _ => Ok(None),
    }
}
