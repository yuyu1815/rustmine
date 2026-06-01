use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};

pub(crate) fn read_custom_payload_configuration_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::clientbound::internal_ids::ConfigurationCustomPayloadClientbound => {
            let packet =
                packet::configuration::clientbound::ConfigurationCustomPayloadClientbound {
                    channel: Serializable::read_from(buf)?,
                    data: Serializable::read_from(buf)?,
                };
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: packet.channel,
                    data: packet.data,
                },
            )))
        }
        _ => Ok(None),
    }
}
