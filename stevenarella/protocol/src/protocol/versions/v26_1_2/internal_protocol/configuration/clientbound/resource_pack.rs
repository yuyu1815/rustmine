use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, UUID,
};

pub(crate) fn read_resource_pack_configuration_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::clientbound::internal_ids::ConfigurationResourcePackPopClientbound => {
            let id_present: bool = Serializable::read_from(buf)?;
            let _packet = packet::configuration::clientbound::ConfigurationResourcePackPopClientbound {
                id_present,
                id: if id_present {
                    Some(Serializable::read_from(buf)?)
                } else {
                    None
                },
            };
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "ResourcePackPop".to_owned(),
                    data: Vec::new(),
                },
            )))
        }
        packet::configuration::clientbound::internal_ids::ConfigurationResourcePackPushClientbound => {
            let id: UUID = Serializable::read_from(buf)?;
            let url: String = Serializable::read_from(buf)?;
            let hash: String = Serializable::read_from(buf)?;
            let required: bool = Serializable::read_from(buf)?;
            let prompt_present: bool = Serializable::read_from(buf)?;
            let _packet = packet::configuration::clientbound::ConfigurationResourcePackPushClientbound {
                id,
                url,
                hash,
                required,
                prompt_present,
                prompt_data: if prompt_present {
                    Serializable::read_from(buf)?
                } else {
                    Vec::new()
                },
            };
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "ResourcePackPush".to_owned(),
                    data: Vec::new(),
                },
            )))
        }
        _ => Ok(None),
    }
}
