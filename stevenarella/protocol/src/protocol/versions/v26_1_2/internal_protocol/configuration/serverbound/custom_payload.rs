use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};

pub(crate) fn read_custom_payload_configuration_serverbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::serverbound::internal_ids::ConfigurationCustomPayloadServerbound => {
            let packet =
                packet::configuration::serverbound::ConfigurationCustomPayloadServerbound {
                    channel: Serializable::read_from(buf)?,
                    data: Serializable::read_from(buf)?,
                };
            Ok(Some(Packet::PluginMessageServerbound(
                packet::play::serverbound::PluginMessageServerbound {
                    channel: packet.channel,
                    data: packet.data,
                },
            )))
        }
        _ => Ok(None),
    }
}
